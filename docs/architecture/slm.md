# SLM Engine

**Crate**: `nis2-slm` | **Files**: `prompt.rs`, `grammar.rs`, `inference.rs`, `report.rs` | **Tests**: 18

## What it does

Transforms structured `ComplianceStatus` data into an Italian-language compliance report. The current implementation uses a `TemplateBackend` that generates reports by template substitution -- no language model is involved.

## Modules

### `prompt.rs`

Builds a ChatML-formatted prompt from `ComplianceStatus` data. Defines `InferenceParams` with deterministic defaults (temperature=0.0, max_tokens=2048, top_p=1.0, repeat_penalty=1.1). The system prompt instructs the model to respond only in formal Italian, cite only provided articles, and never use hedging language.

### `grammar.rs`

Defines a GBNF grammar (`COMPLIANCE_REPORT_GRAMMAR`) that constrains output to a 4-section report structure with NIS2 article reference patterns. Also defines `TOOL_OUTPUT_GRAMMAR` for structured tool responses. Provides `grammar_for_mode()` to select between them.

### `inference.rs`

Defines the `InferenceBackend` trait with `generate(prompt, params) -> Result<String>` and `name() -> &str`. Implements `TemplateBackend` which ignores the prompt and instead parses the embedded JSON to generate reports deterministically.

### `report.rs`

The actual report generator. Produces 4 sections:

1. **Ambito di applicazione** -- scope determination with entity classification
2. **Obblighi rilevanti** -- all 16 obligations listed with article references
3. **Sanzioni potenziali** -- Art. 34 maximum penalty
4. **Segnalazione incidenti** -- 24h/72h/30d deadlines

For out-of-scope entities, produces a short non-applicability statement instead.

## Tests (18)

Prompt tests: `prompt_contains_system_instructions`, `prompt_contains_compliance_data`, `prompt_ends_with_assistant_tag`, `default_params_are_deterministic`

Grammar tests: `grammar_contains_all_sections`, `grammar_enforces_article_pattern`, `mode_selects_correct_grammar`

Inference tests: `template_backend_generates_report`, `template_backend_cites_articles`, `template_backend_name`

Report tests: `essential_report_has_all_sections`, `essential_report_cites_classification`, `essential_report_lists_all_obligations`, `essential_report_shows_sanction`, `essential_report_shows_deadlines`, `out_of_scope_report_states_non_applicability`, `report_never_contains_uncertain_language`, `report_uses_deterministic_formula`
