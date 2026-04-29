---
layout: home

hero:
  name: NIS2 Compliance Engine
  text: Conformita deterministica, esecuzione locale
  tagline: Zero hallucination — Zero cloud — Output matematicamente vincolato
  actions:
    - theme: brand
      text: Quick Start
      link: /guide/getting-started
    - theme: alt
      text: Architettura
      link: /architecture/overview
    - theme: alt
      text: GitHub
      link: https://github.com/fabriziosalmi/nis2-model

features:
  - title: Conformita NIS2 e DORA
    details: Implementazione completa della Direttiva (UE) 2022/2555 e del Regolamento (UE) 2022/2554. 16 obblighi mappati, sanzioni calcolate, tempistiche incidenti.
  - title: Zero Hallucination
    details: Ogni output e derivato deterministicamente dai dati strutturati. Grammatica GBNF vincola la generazione a livello di token. Nessun contenuto inventato.
  - title: 100% Locale
    details: Nessuna API cloud, nessuna chiave, nessun dato esfiltrato. Embedding ONNX, LanceDB embedded, inferenza locale. Privacy by design.
  - title: Prestazioni Native
    details: Scritto in Rust 2024 edition. Workspace a 6 crate. 61 test deterministici. Compilazione nativa senza overhead di runtime.
  - title: MCP Integration
    details: 4 tool esposti via Model Context Protocol (JSON-RPC 2.0). Compatibile con Claude Desktop e qualsiasi client MCP.
  - title: Rule Engine Deterministico
    details: Alberi decisionali booleani per applicabilita, classificazione, obblighi e sanzioni. Validazione JSON Schema in ingresso e in uscita.
---
