//! Legal text parser — extracts structured provisions from raw text sources.
//!
//! Future: integrate `pdfium-render` for PDF extraction. Currently provides
//! a JSON-based import path for pre-structured texts.

use anyhow::Result;
use std::path::Path;

use crate::chunk::make_chunk;
use crate::types::{Chunk, Directive, LegalReference};

/// An intermediate representation of a parsed article, typically loaded from
/// pre-structured JSON.
#[derive(Debug, serde::Deserialize)]
pub struct RawArticle {
    pub directive: String,
    pub article: u16,
    pub paragraphs: Vec<RawParagraph>,
}

/// A single paragraph within an article.
#[derive(Debug, serde::Deserialize)]
pub struct RawParagraph {
    pub number: u16,
    pub text: String,
    #[serde(default)]
    pub letters: Vec<RawLetter>,
}

/// A lettered sub-point within a paragraph.
#[derive(Debug, serde::Deserialize)]
pub struct RawLetter {
    pub letter: char,
    pub text: String,
}

/// Parses the directive string into the [`Directive`] enum.
fn parse_directive(s: &str) -> Result<Directive> {
    match s.to_lowercase().as_str() {
        "nis2" | "2022/2555" => Ok(Directive::Nis2),
        "dora" | "2022/2554" => Ok(Directive::Dora),
        other => anyhow::bail!("Unknown directive: {other}"),
    }
}

/// Loads pre-structured JSON articles from a file and converts them to [`Chunk`]s.
pub fn load_articles_from_json(path: &Path) -> Result<Vec<Chunk>> {
    let content = std::fs::read_to_string(path)?;
    let articles: Vec<RawArticle> = serde_json::from_str(&content)?;
    let mut chunks = Vec::new();

    for article in articles {
        let directive = parse_directive(&article.directive)?;

        for paragraph in &article.paragraphs {
            if paragraph.letters.is_empty() {
                // Paragraph-level chunk
                let reference = LegalReference {
                    directive,
                    article: article.article,
                    paragraph: paragraph.number,
                    letter: None,
                };
                chunks.push(make_chunk(reference, &paragraph.text));
            } else {
                // Letter-level chunks
                for letter in &paragraph.letters {
                    let reference = LegalReference {
                        directive,
                        article: article.article,
                        paragraph: paragraph.number,
                        letter: Some(letter.letter),
                    };
                    chunks.push(make_chunk(reference, &letter.text));
                }
            }
        }
    }

    Ok(chunks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn parse_json_articles() {
        let json = serde_json::json!([{
            "directive": "nis2",
            "article": 21,
            "paragraphs": [{
                "number": 2,
                "text": "Le misure di cui al paragrafo 1 comprendono almeno gli elementi seguenti:",
                "letters": [
                    { "letter": "a", "text": "politiche di analisi dei rischi e di sicurezza dei sistemi informatici" },
                    { "letter": "b", "text": "gestione degli incidenti" }
                ]
            }]
        }]);

        let mut f = NamedTempFile::new().unwrap();
        write!(f, "{json}").unwrap();

        let chunks = load_articles_from_json(f.path()).unwrap();
        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0].reference.article, 21);
        assert_eq!(chunks[0].reference.letter, Some('a'));
        assert_eq!(chunks[1].reference.letter, Some('b'));
    }
}
