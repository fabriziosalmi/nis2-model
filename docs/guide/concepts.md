# Concetti chiave

## Determinismo

Ogni funzione del motore è **deterministica**: dato lo stesso input, produce sempre lo stesso output. Non ci sono elementi stocastici, LLM non vincolati, o dipendenze da servizi esterni.

```
f(CompanyProfile) → ComplianceStatus  // sempre uguale
```

## Chunk semantico

I testi legali non sono suddivisi per numero di parole, ma per **unità logica normativa**:

| Livello | Esempio | ID deterministico |
|---------|---------|-------------------|
| Articolo | Art. 21 | `nis2-21` |
| Paragrafo | Art. 21, par. 2 | `nis2-21-2` |
| Lettera | Art. 21(2)(a) | `nis2-21-2-a` |

Ogni chunk conserva il suo `LegalReference` completo per la tracciabilità.

## Classificazione delle entità

Il motore implementa l'albero decisionale dell'Art. 2 e 3 NIS2:

```
┌──────────────────────────────────────────────┐
│               CompanyProfile                  │
│  { sector, employees, annual_revenue }        │
└──────────────────┬───────────────────────────┘
                   │
     ┌─────────────┼──────────────┐
     │             │              │
  Annex I      Annex II     Nessun Annex
  + size ≥ M   + size ≥ M        │
     │             │              │
  Essential    Important     Out of Scope
  Art. 3(1)    Art. 3(2)
```

::: info Soglia dimensionale
Un'impresa è "media" (e quindi potenzialmente in ambito) se ha **≥ 50 dipendenti** oppure **≥ €10M di fatturato**, secondo la Raccomandazione UE 2003/361/EC.
:::

## Soggetti sempre in ambito

Alcuni settori sono in ambito **indipendentemente dalla dimensione** (Art. 2(2)):

- `digital_infrastructure`
- `ict_service_management_b2b`
- `public_administration`

## Zero-Hallucination

La garanzia di zero-hallucination si basa su tre livelli:

1. **Catalogo immutabile** — i 16 obblighi sono codificati in Rust, non generati
2. **Template deterministico** — il report è costruito per sostituzione di variabili
3. **Grammatica GBNF** — quando si usa un SLM, la generazione è vincolata a livello di token

```rust
// Parole vietate nel report
let forbidden = ["potrebbe", "forse", "probabilmente", "eventualmente"];
```
