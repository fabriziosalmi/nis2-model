# SLM Engine

**Crate**: `nis2-slm`

## Responsabilità

Trasforma dati strutturati di conformità in report in italiano formale, con garanzia di zero hallucination.

## Moduli

| Modulo | Responsabilità |
|--------|---------------|
| `prompt` | System prompt, ChatML template, InferenceParams |
| `grammar` | GBNF per generazione vincolata |
| `inference` | Trait backend + TemplateBackend |
| `report` | Generatore deterministico di report |

## Prompt Engineering

Il system prompt vincola il comportamento del modello:

```
REGOLE INDEROGABILI:
1. Rispondi ESCLUSIVAMENTE in italiano formale
2. Cita SOLO gli articoli forniti nel contesto JSON
3. Non fornire MAI consulenza legale
4. Struttura: Ambito → Obblighi → Sanzioni
5. No "potrebbe", "forse", "probabilmente"
```

### Formato ChatML

```
<|im_start|>system
{system_prompt}
<|im_end|>
<|im_start|>user
DATI DI CONFORMITÀ (JSON):
```json
{compliance_status}
```
<|im_end|>
<|im_start|>assistant
```

## GBNF Grammar

La grammatica forza una struttura a 4 sezioni:

```gbnf
root ::= section-ambito section-obblighi section-sanzioni section-incidenti
section-ambito ::= "## Ambito di applicazione\n\n" paragraph "\n\n"
article-ref ::= "Art. " [0-9]+ ("(" [0-9]+ ")" ("(" [a-j] ")")?)?
```

## Backend di inferenza

### `InferenceBackend` Trait

```rust
pub trait InferenceBackend {
    fn generate(&self, prompt: &str, params: &InferenceParams) -> Result<String>;
    fn name(&self) -> &str;
}
```

### `TemplateBackend` (default)

Genera report **senza modello**, per sostituzione di variabili:

- Zero hallucination garantita
- Nessun download di modello
- Generazione istantanea
- Output deterministico

### `LlamaCppBackend` (planned)

Inferenza con modelli GGUF locali (es. Qwen-2.5-0.5B-Instruct):

- Temperatura = 0.0 (greedy decoding)
- Grammatica GBNF applicata a livello di token
- Post-validazione contro lo schema

## Parametri di inferenza

```rust
InferenceParams {
    temperature: 0.0,      // Deterministico
    max_tokens: 2048,      // Sufficiente per report completo
    top_p: 1.0,            // Disabilitato con temp=0
    repeat_penalty: 1.1,   // Penalità ripetizione
}
```

## Report di output

Il report segue una struttura fissa:

1. **Ambito di applicazione** — classificazione del soggetto
2. **Obblighi rilevanti** — elenco dei 16 obblighi con Art. di riferimento
3. **Sanzioni potenziali** — importo massimo con base legale
4. **Segnalazione incidenti** — tempistiche 24h / 72h / 30gg

## Test

```bash
cargo test -p nis2-slm  # 18 test
```

Test critici:
- `report_never_contains_uncertain_language` — parole vietate
- `report_uses_deterministic_formula` — formula "Sulla base dei dati forniti..."
- `template_backend_cites_articles` — tutti gli articoli citati
