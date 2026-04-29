# Ingestion Pipeline

**Crate**: `nis2-ingestion`

## Responsabilità

Estrae testi normativi da fonti strutturate e li suddivide in chunk semantici con riferimenti legali tracciabili.

## Tipi principali

### `LegalReference`

```rust
pub struct LegalReference {
    pub directive: Directive,    // NIS2 o DORA
    pub article: u32,            // Numero articolo
    pub paragraph: u32,          // Numero paragrafo
    pub letter: Option<char>,    // Lettera (a-j)
}
```

### `Chunk`

```rust
pub struct Chunk {
    pub id: String,              // Hash deterministico
    pub reference: LegalReference,
    pub text: String,            // Testo della disposizione
}
```

## ID deterministico

L'ID di ogni chunk è calcolato come hash SHA-256 del suo `LegalReference`:

```
nis2-21-2-a → SHA-256 → "a1b2c3d4..."
```

Questo garantisce che lo stesso articolo produca sempre lo stesso ID, indipendentemente dall'ordine di elaborazione.

## Parser JSON

Il parser legge file JSON strutturati con il formato:

```json
{
  "directive": "NIS2",
  "articles": [
    {
      "number": 21,
      "title": "Misure di gestione dei rischi",
      "paragraphs": [
        {
          "number": 2,
          "letters": [
            {
              "letter": "a",
              "text": "politiche di analisi dei rischi..."
            }
          ]
        }
      ]
    }
  ]
}
```

## Dati sorgente

| File | Direttiva | Articoli | Chunk |
|------|-----------|----------|-------|
| `nis2_articles.json` | NIS2 (2022/2555) | 8 | 35 |
| `dora_articles.json` | DORA (2022/2554) | 5 | 14 |
| **Totale** | | **13** | **49** |

## Test

```bash
cargo test -p nis2-ingestion  # 3 test
```

- `chunk_id_is_deterministic` — stesso input → stesso hash
- `different_references_produce_different_ids` — unicità
- `parse_json_articles` — roundtrip completo dal JSON
