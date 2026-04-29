# Vector Store

**Crate**: `nis2-vectordb`

## Responsabilità

Indicizza chunk legali come vettori densi e li recupera per similarità semantica.

## Stack

| Componente | Tecnologia | Dettaglio |
|------------|------------|-----------|
| Embedding | BGE-Small-EN-v1.5 | 384 dimensioni, ONNX Runtime |
| Storage | LanceDB | Embedded, serverless, Arrow-native |
| Indice | HNSW | Approximate Nearest Neighbor |

## Embedder

```rust
pub struct Embedder {
    model: Mutex<TextEmbedding>,  // Interior mutability
    dims: usize,                   // 384 per BGE-Small
}
```

L'`Embedder` usa `Mutex` per soddisfare il requisito `&mut self` di fastembed, consentendo embedding concorrenti in contesti async.

## LanceDB Store

```rust
pub struct LanceStore {
    db: lancedb::Connection,
    embedder: Embedder,
    dims: i32,
}
```

### Schema Arrow

| Colonna | Tipo | Nullable |
|---------|------|----------|
| `id` | Utf8 | No |
| `directive` | Utf8 | No |
| `article` | Utf8 | No |
| `paragraph` | Utf8 | No |
| `letter` | Utf8 | Sì |
| `text` | Utf8 | No |
| `vector` | FixedSizeList(Float32, 384) | No |

## CLI

### Indexer

```bash
cargo run --bin indexer
```

Popola il vector store con tutti i chunk da `data/sources/`.

### Search

```bash
cargo run --bin search -- "gestione degli incidenti"
```

Restituisce i 5 chunk più simili con score di similarità.

## Esempio di ricerca

```
Query: "requisiti autenticazione multi-fattore"

1. [score: 0.6443] NIS2 Art. 21(2)(j)
   uso di soluzioni di autenticazione a più fattori...

2. [score: 0.6429] NIS2 Art. 21(2)(i)
   sicurezza delle risorse umane, strategie di controllo dell'accesso...
```
