# Panoramica

Il **NIS2 Compliance Engine** è un motore di conformità deterministico ed eseguibile localmente per la regolamentazione europea sulla cybersecurity.

## Il problema

Le organizzazioni soggette alla Direttiva NIS2 devono:

1. **Determinare l'applicabilità** — verificare se rientrano nell'ambito
2. **Identificare gli obblighi** — mappare i 16 requisiti di Art. 20, 21 e 23
3. **Calcolare il rischio sanzionatorio** — comprendere l'esposizione finanziaria
4. **Produrre documentazione** — report di conformità con citazioni normative esatte

I tool esistenti si basano su LLM cloud che **inventano articoli inesistenti**, forniscono **cifre approssimative** e richiedono **connettività internet**.

## La soluzione

Un motore di conformità che:

- **Gira interamente in locale** -- nessun dato esce dalla macchina
- **Deterministico** -- stesso input, stesso output, sempre
- **Cita solo fonti reali** -- ogni articolo proviene dal catalogo immutabile
- **Validato** -- 61 test automatizzati verificano la correttezza

## Direttive supportate

| Direttiva | Copertura |
|-----------|-----------|
| **NIS2** (EU 2022/2555) | Applicabilità, classificazione, Art. 20/21/23, Art. 34 |
| **DORA** (EU 2022/2554) | Testi indicizzati, ricerca semantica |

## Stack tecnologico

| Componente | Tecnologia |
|------------|------------|
| Linguaggio | Rust 2024 Edition |
| Embedding | BGE-Small-EN-v1.5 (fastembed / ONNX Runtime) |
| Vector Store | LanceDB embedded (HNSW) |
| Protocollo | MCP (Model Context Protocol) JSON-RPC 2.0 |
| Validazione | JSON Schema (schemars + jsonschema) |
| Plugin WASM | Extism |
