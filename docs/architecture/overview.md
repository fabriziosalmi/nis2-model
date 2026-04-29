# I 5 Pilastri

L'architettura del NIS2 Compliance Engine è organizzata in una pipeline unidirezionale a 5 stadi.

## Flusso dati

```
Testi Legali UE → Ingestion → Vector DB → Rule Engine → SLM → Report
                      ↓            ↓           ↓          ↓
                   JSON chunks  LanceDB   ComplianceStatus  Italiano formale
                   (49 prov.)   (HNSW)    (16 obblighi)     (zero-halluc.)
```

## Architettura dei crate

```
┌─────────────────────────────────────────────────────────────────┐
│                        Cargo Workspace                          │
├────────────┬────────────┬────────────┬────────────┬─────────────┤
│ ingestion  │  vectordb  │   rules    │ mcp-server │     slm     │
│            │            │            │            │             │
│ • Parser   │ • Embedder │ • Engine   │ • Protocol │ • Prompt    │
│ • Chunker  │ • LanceDB  │ • Schema   │ • Tools    │ • Grammar   │
│ • Types    │ • Search   │ • Obbligh. │ • Server   │ • Inference │
│            │            │ • Valid.   │            │ • Report    │
│            │            │ • Adapter  │            │             │
├────────────┴────────────┴────────────┴────────────┴─────────────┤
│                       nis2-api (axum REST)                       │
└─────────────────────────────────────────────────────────────────┘
```

## I 5 Pilastri in dettaglio

### 1. [Ingestion Pipeline](/architecture/ingestion)

Estrae e struttura i testi normativi da fonti ufficiali (PDF, JSON). Ogni disposizione è suddivisa in chunk semantici per unità logica (articolo → paragrafo → lettera).

### 2. [Vector Store](/architecture/vectordb)

Indicizza i chunk testuali come vettori 384-dimensionali (BGE-Small) in LanceDB embedded. Ricerca semantica HNSW per recuperare le disposizioni rilevanti.

### 3. [Rule Engine](/architecture/rules)

Alberi decisionali booleani deterministici. Valuta applicabilità (Art. 2/3), mappa 16 obblighi (Art. 20/21/23), calcola sanzioni (Art. 34). Validazione JSON Schema in ingresso e uscita.

### 4. [MCP Server](/architecture/mcp)

Espone il rule engine come 4 tool MCP via JSON-RPC 2.0 su stdio. Compatibile con Claude Desktop e qualsiasi client MCP.

### 5. [SLM Engine](/architecture/slm)

Trasforma i dati strutturati di conformità in report in italiano formale. Grammatica GBNF per generazione vincolata. Backend template per output deterministico senza modello.

## Principi architetturali

| Principio | Implementazione |
|-----------|----------------|
| **Determinismo** | Nessun elemento stocastico, temperatura = 0.0 |
| **Zero Trust** | Nessuna API cloud, esecuzione 100% locale |
| **Tracciabilità** | Ogni output cita l'articolo sorgente |
| **Validazione** | JSON Schema su tutti i confini tra crate |
| **Estensibilità** | Plugin WASM via Extism per adapter custom |
