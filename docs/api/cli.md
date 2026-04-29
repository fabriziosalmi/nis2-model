# CLI

Il workspace fornisce 5 binari eseguibili.

## `indexer`

Popola il vector store LanceDB con i testi legali strutturati.

```bash
cargo run --bin indexer
```

| Argomento | Default | Descrizione |
|-----------|---------|-------------|
| — | `data/sources/` | Directory sorgente JSON |
| — | `data/lancedb/` | Directory database |

## `search`

Ricerca semantica sui testi legali indicizzati.

```bash
cargo run --bin search -- "<query>"
```

| Argomento | Descrizione |
|-----------|-------------|
| `<query>` | Testo di ricerca in linguaggio naturale |

### Esempio

```bash
cargo run --bin search -- "gestione dei rischi informatici"

Query: "gestione dei rischi informatici"
1. [score: 0.6840] NIS2 Art. 21(2)(b) — gestione degli incidenti
2. [score: 0.6447] DORA Art. 6(1) — quadro gestione rischi informatici
```

## `report`

Genera un report di conformità completo in italiano.

```bash
cargo run --bin report
```

Output: report strutturato in 4 sezioni (Ambito, Obblighi, Sanzioni, Incidenti).

## `demo`

Output JSON grezzo del rule engine.

```bash
cargo run --bin demo
```

Output: `ComplianceStatus` JSON pretty-printed.

## `mcp-server`

Avvia il server MCP su stdio.

```bash
cargo run --bin mcp-server
```

Variabili d'ambiente:

| Variabile | Default | Descrizione |
|-----------|---------|-------------|
| `RUST_LOG` | `info` | Livello di log (stderr) |
