# Rule Engine

**Crate**: `nis2-rules`

## Responsabilità

Valutazione deterministica della conformità NIS2. Ogni decisione è un albero booleano senza ambiguità.

## Moduli

| Modulo | Responsabilità |
|--------|---------------|
| `engine` | Applicabilità, classificazione, calcolo sanzioni |
| `obligations` | Catalogo immutabile dei 16 obblighi |
| `schema` | Tipi con derivazione JSON Schema |
| `validation` | Validazione runtime input/output |
| `adapters` | Plugin WASM via Extism |

## Albero decisionale

```rust
pub fn evaluate(profile: &CompanyProfile) -> ComplianceStatus
```

```
CompanyProfile
     │
     ├── Settore in Annex I + dimensione ≥ media
     │   └── Essential (Art. 3(1))
     │
     ├── Settore in Annex II + dimensione ≥ media
     │   └── Important (Art. 3(2))
     │
     ├── Settore "always in scope" (qualsiasi dimensione)
     │   └── Essential (Art. 2(2))
     │
     └── Altrimenti
         └── OutOfScope
```

## Settori

### Annex I (Altamente critici)
`energy`, `transport`, `banking`, `financial_market_infrastructure`, `health`, `drinking_water`, `waste_water`, `digital_infrastructure`, `ict_service_management_b2b`, `public_administration`, `space`

### Annex II (Altri settori critici)
`postal_courier`, `waste_management`, `chemicals`, `food`, `manufacturing`, `digital_providers`, `research`

## Catalogo obblighi

### Art. 20 — Governance (2 obblighi)

| ID | Descrizione |
|----|-------------|
| `nis2_art20_1` | Approvazione misure di gestione rischi |
| `nis2_art20_2` | Formazione obbligatoria cybersecurity organi direttivi |

### Art. 21(2) — Misure di cybersecurity (10 obblighi)

| ID | Lettera | Descrizione |
|----|---------|-------------|
| `nis2_art21_2_a` | (a) | Politiche di analisi dei rischi |
| `nis2_art21_2_b` | (b) | Gestione degli incidenti |
| `nis2_art21_2_c` | (c) | Continuità operativa e gestione crisi |
| `nis2_art21_2_d` | (d) | Sicurezza catena di approvvigionamento |
| `nis2_art21_2_e` | (e) | Sicurezza sviluppo e manutenzione sistemi |
| `nis2_art21_2_f` | (f) | Valutazione efficacia misure |
| `nis2_art21_2_g` | (g) | Igiene informatica e formazione |
| `nis2_art21_2_h` | (h) | Crittografia e cifratura |
| `nis2_art21_2_i` | (i) | Sicurezza risorse umane e controllo accessi |
| `nis2_art21_2_j` | (j) | Autenticazione multi-fattore |

### Art. 23 — Segnalazione incidenti (4 obblighi)

| ID | Descrizione | Tempistica |
|----|-------------|------------|
| `nis2_art23_1` | Notifica incidenti significativi | Senza ritardo |
| `nis2_art23_4_a` | Preallarme | 24 ore |
| `nis2_art23_4_b` | Notifica completa | 72 ore |
| `nis2_art23_4_d` | Relazione finale | 30 giorni |

## Sanzioni (Art. 34)

| Categoria | Formula | Floor |
|-----------|---------|-------|
| Essential | max(€10M, 2% fatturato mondiale) | €10.000.000 |
| Important | max(€7M, 1.4% fatturato mondiale) | €7.000.000 |

## JSON Schema Validation

Tutti i tipi derivano `schemars::JsonSchema`:

```rust
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct CompanyProfile { ... }
```

Validazione runtime:

```rust
validation::validate_company_profile(&json)?;
validation::validate_compliance_status(&json)?;
```

## Test

```bash
cargo test -p nis2-rules  # 20 test
```
