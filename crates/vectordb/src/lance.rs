//! LanceDB-backed vector store implementation.
//!
//! Provides embedded, serverless vector storage with HNSW indexing.

use std::sync::Arc;

use anyhow::Result;
use arrow_array::{
    Array, FixedSizeListArray, Float32Array, RecordBatch, StringArray,
};
use arrow_schema::{DataType, Field, Schema};
use futures::TryStreamExt;
use lancedb::query::{ExecutableQuery, QueryBase};
use tracing::info;

use crate::embed::Embedder;
use crate::store::SearchResult;
use nis2_ingestion::{Chunk, Directive, LegalReference};

/// The LanceDB table name for legal text chunks.
const TABLE_NAME: &str = "legal_chunks";

/// LanceDB-backed vector store for legal text chunks.
pub struct LanceStore {
    db: lancedb::Connection,
    embedder: Embedder,
    dims: i32,
}

impl LanceStore {
    /// Opens (or creates) a LanceDB database at the given path.
    pub async fn open(db_path: &str, embedder: Embedder) -> Result<Self> {
        let dims = embedder.dimensions() as i32;
        let db = lancedb::connect(db_path).execute().await?;
        Ok(Self { db, embedder, dims })
    }

    /// Build the Arrow schema for the legal chunks table.
    fn schema(&self) -> Arc<Schema> {
        Arc::new(Schema::new(vec![
            Field::new("id", DataType::Utf8, false),
            Field::new("directive", DataType::Utf8, false),
            Field::new("article", DataType::Utf8, false),
            Field::new("paragraph", DataType::Utf8, false),
            Field::new("letter", DataType::Utf8, true),
            Field::new("text", DataType::Utf8, false),
            Field::new(
                "vector",
                DataType::FixedSizeList(
                    Arc::new(Field::new("item", DataType::Float32, true)),
                    self.dims,
                ),
                false,
            ),
        ]))
    }

    /// Index a batch of chunks — embeds their text and stores them in LanceDB.
    pub async fn index_chunks(&self, chunks: &[Chunk]) -> Result<usize> {
        if chunks.is_empty() {
            return Ok(0);
        }

        let texts: Vec<&str> = chunks.iter().map(|c| c.text.as_str()).collect();
        let embeddings = self.embedder.embed_batch(&texts)?;

        let schema = self.schema();

        let ids = StringArray::from(
            chunks.iter().map(|c| c.id.as_str()).collect::<Vec<_>>(),
        );
        let directives = StringArray::from(
            chunks
                .iter()
                .map(|c| format!("{}", c.reference.directive))
                .collect::<Vec<String>>(),
        );
        let articles = StringArray::from(
            chunks
                .iter()
                .map(|c| c.reference.article.to_string())
                .collect::<Vec<String>>(),
        );
        let paragraphs = StringArray::from(
            chunks
                .iter()
                .map(|c| c.reference.paragraph.to_string())
                .collect::<Vec<String>>(),
        );
        let letters = StringArray::from(
            chunks
                .iter()
                .map(|c| c.reference.letter.map(|l| l.to_string()))
                .collect::<Vec<Option<String>>>(),
        );
        let text_col = StringArray::from(
            chunks.iter().map(|c| c.text.as_str()).collect::<Vec<_>>(),
        );

        // Flatten embeddings into a single FixedSizeList
        let flat: Vec<f32> = embeddings.iter().flatten().copied().collect();
        let values = Float32Array::from(flat);
        let vector_col = FixedSizeListArray::new(
            Arc::new(Field::new("item", DataType::Float32, true)),
            self.dims,
            Arc::new(values),
            None,
        );

        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![
                Arc::new(ids),
                Arc::new(directives),
                Arc::new(articles),
                Arc::new(paragraphs),
                Arc::new(letters),
                Arc::new(text_col),
                Arc::new(vector_col),
            ],
        )?;

        let count = batch.num_rows();
        let batches = vec![batch];

        // Create or append to the table
        let table_names = self.db.table_names().execute().await?;
        if table_names.contains(&TABLE_NAME.to_string()) {
            let table = self.db.open_table(TABLE_NAME).execute().await?;
            table.add(batches).execute().await?;
        } else {
            self.db
                .create_table(TABLE_NAME, batches)
                .execute()
                .await?;
        }

        info!("Indexed {count} chunks into LanceDB");
        Ok(count)
    }

    /// Perform semantic search — returns the top-k most similar chunks.
    pub async fn search(&self, query: &str, top_k: usize) -> Result<Vec<SearchResult>> {
        let query_vec = self.embedder.embed_one(query)?;

        let table = self.db.open_table(TABLE_NAME).execute().await?;
        let mut results_stream = table
            .vector_search(query_vec)?
            .limit(top_k)
            .execute()
            .await?;

        let mut search_results = Vec::new();

        while let Some(batch) = results_stream.try_next().await? {
            let ids = batch
                .column_by_name("id")
                .unwrap()
                .as_any()
                .downcast_ref::<StringArray>()
                .unwrap();
            let directives = batch
                .column_by_name("directive")
                .unwrap()
                .as_any()
                .downcast_ref::<StringArray>()
                .unwrap();
            let articles = batch
                .column_by_name("article")
                .unwrap()
                .as_any()
                .downcast_ref::<StringArray>()
                .unwrap();
            let paragraphs = batch
                .column_by_name("paragraph")
                .unwrap()
                .as_any()
                .downcast_ref::<StringArray>()
                .unwrap();
            let letters_col = batch
                .column_by_name("letter")
                .unwrap()
                .as_any()
                .downcast_ref::<StringArray>()
                .unwrap();
            let texts = batch
                .column_by_name("text")
                .unwrap()
                .as_any()
                .downcast_ref::<StringArray>()
                .unwrap();
            let distances = batch
                .column_by_name("_distance")
                .unwrap()
                .as_any()
                .downcast_ref::<Float32Array>()
                .unwrap();

            for i in 0..batch.num_rows() {
                let directive = match directives.value(i) {
                    s if s.contains("2555") => Directive::Nis2,
                    s if s.contains("2554") => Directive::Dora,
                    _ => Directive::Nis2,
                };
                let letter = if letters_col.is_null(i) {
                    None
                } else {
                    letters_col.value(i).chars().next()
                };

                let chunk = Chunk {
                    id: ids.value(i).to_string(),
                    reference: LegalReference {
                        directive,
                        article: articles.value(i).parse().unwrap_or(0),
                        paragraph: paragraphs.value(i).parse().unwrap_or(0),
                        letter,
                    },
                    text: texts.value(i).to_string(),
                };

                // LanceDB returns L2 distance; convert to similarity score
                let distance = distances.value(i);
                let score = 1.0 / (1.0 + distance);

                search_results.push(SearchResult { chunk, score });
            }
        }

        Ok(search_results)
    }
}
