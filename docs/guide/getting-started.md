# Quick Start

## Prerequisiti

- **Rust** ≥ 1.85 (2024 edition)
- ~500MB di RAM per ONNX Runtime (modello BGE-Small)
- macOS, Linux o Windows con toolchain Rust

## Installazione

```bash
# Clona il repository
git clone https://github.com/fabriziosalmi/nis2-model.git
cd nis2-model

# Compila tutto il workspace
cargo build --workspace
```

::: tip Prima compilazione
La prima compilazione scarica automaticamente il modello BGE-Small-EN-v1.5 (~33MB). Le compilazioni successive saranno istantanee.
:::

## Esecuzione dei test

```bash
# Esegui tutti i 61 test
cargo test --workspace
```

## Indicizzazione dei testi legali

Prima di poter effettuare ricerche semantiche, è necessario popolare il vector store:

```bash
# Indicizza 49 chunk (35 NIS2 + 14 DORA) in LanceDB
cargo run --bin indexer
```

Output atteso:
```
INFO indexer: Parsed 35 NIS2 chunks
INFO indexer: Indexed 35 NIS2 chunks
INFO indexer: Parsed 14 DORA chunks
INFO indexer: Indexed 14 DORA chunks
INFO indexer: Indexing complete.
```

## Ricerca semantica

```bash
# Cerca disposizioni sull'autenticazione multi-fattore
cargo run --bin search -- "autenticazione multi-fattore"
```

```
1. [score: 0.6443] NIS2 Art. 21(2)(j)
   uso di soluzioni di autenticazione a più fattori...
```

## Generazione report

```bash
# Report di conformità completo in italiano
cargo run --bin report
```

```
## Ambito di applicazione
Sulla base dei dati forniti, risulta che l'azienda rientra
nell'ambito di applicazione della Direttiva (UE) 2022/2555 (NIS2),
in qualità di soggetto essenziale ai sensi dell'Art. 3(1).

## Obblighi rilevanti (16 totali)
- Art. 20(1): Governance organi direttivi
- Art. 21(2)(a): Analisi dei rischi
...
```

## Server MCP

```bash
# Avvia il server MCP (JSON-RPC su stdio)
cargo run --bin mcp-server
```

Vedi la sezione [MCP Tools](/api/mcp-tools) per i dettagli sui tool disponibili.

## Prossimi passi

- Leggi i [Concetti chiave](/guide/concepts)
- Esplora l'[Architettura](/architecture/overview)
- Consulta le [API](/api/mcp-tools)
