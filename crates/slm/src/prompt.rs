//! System prompt and template engineering for the SLM.
//!
//! The prompt is constructed deterministically from the ComplianceStatus JSON.
//! The SLM's only job is to transform this structured data into formal Italian prose.

use nis2_rules::schema::ComplianceStatus;

/// The system prompt that locks the SLM to factual report generation.
pub const SYSTEM_PROMPT: &str = r#"Sei un assistente tecnico-legale specializzato in conformità NIS2 e DORA.

REGOLE INDEROGABILI:
1. Rispondi ESCLUSIVAMENTE in italiano formale.
2. Cita SOLO gli articoli e i commi forniti nel contesto JSON — nessuna invenzione.
3. Non fornire MAI consulenza legale. Usa sempre la formula: "Sulla base dei dati forniti, risulta che…"
4. Se un dato è mancante, dichiaralo esplicitamente: "Dato non disponibile per la valutazione."
5. Struttura ogni risposta in: Ambito di applicazione, Obblighi rilevanti, Sanzioni potenziali.
6. Non usare mai frasi come "potrebbe", "forse", "probabilmente" — solo fatti deterministici.
7. Ogni obbligo DEVE essere citato con il riferimento normativo esatto (es. "Art. 21(2)(a)").
8. Le sanzioni DEVONO riportare gli importi esatti dal contesto, mai valori inventati."#;

/// Inference parameters for constrained generation.
#[derive(Debug, Clone)]
pub struct InferenceParams {
    /// Temperature — 0.0 for fully deterministic output.
    pub temperature: f32,
    /// Maximum tokens to generate.
    pub max_tokens: u32,
    /// Top-p sampling (1.0 = disabled when temperature is 0).
    pub top_p: f32,
    /// Repetition penalty.
    pub repeat_penalty: f32,
}

impl Default for InferenceParams {
    fn default() -> Self {
        Self {
            temperature: 0.0,
            max_tokens: 2048,
            top_p: 1.0,
            repeat_penalty: 1.1,
        }
    }
}

/// Build the full prompt for the SLM from a ComplianceStatus.
///
/// The prompt follows the ChatML format used by Qwen/Llama instruction models.
pub fn build_prompt(company_name: &str, status: &ComplianceStatus) -> String {
    let context_json =
        serde_json::to_string_pretty(status).expect("ComplianceStatus serialization cannot fail");

    format!(
        r#"<|im_start|>system
{SYSTEM_PROMPT}
<|im_end|>
<|im_start|>user
Genera un report di conformità NIS2 per l'azienda "{company_name}" basandoti esclusivamente sui seguenti dati strutturati.

DATI DI CONFORMITÀ (JSON):
```json
{context_json}
```

Il report deve contenere:
1. AMBITO DI APPLICAZIONE: se l'azienda rientra nell'ambito NIS2 e la sua classificazione
2. OBBLIGHI RILEVANTI: elenco degli obblighi applicabili con riferimento normativo
3. SANZIONI POTENZIALI: importo massimo della sanzione
4. SEGNALAZIONE INCIDENTI: tempistiche obbligatorie

Usa SOLO i dati forniti. Non inventare nulla.
<|im_end|>
<|im_start|>assistant
"#
    )
}

/// Build a minimal prompt for a single tool output (non-report context).
pub fn build_tool_prompt(tool_name: &str, tool_output: &str) -> String {
    format!(
        r#"<|im_start|>system
{SYSTEM_PROMPT}
<|im_end|>
<|im_start|>user
Riscrivi il seguente output del tool "{tool_name}" in italiano formale, senza aggiungere informazioni.

OUTPUT DEL TOOL:
{tool_output}
<|im_end|>
<|im_start|>assistant
"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use nis2_rules::engine;
    use nis2_rules::schema::CompanyProfile;

    fn test_status() -> ComplianceStatus {
        let profile = CompanyProfile {
            name: "Test S.r.l.".into(),
            sector: "energy".into(),
            sub_sector: None,
            employees: 200,
            annual_revenue_eur_m: 50.0,
            balance_sheet_eur_m: 40.0,
            services: vec![],
            member_states: vec!["IT".into()],
        };
        engine::evaluate(&profile)
    }

    #[test]
    fn prompt_contains_system_instructions() {
        let prompt = build_prompt("Test S.r.l.", &test_status());
        assert!(prompt.contains("REGOLE INDEROGABILI"));
        assert!(prompt.contains("<|im_start|>system"));
    }

    #[test]
    fn prompt_contains_compliance_data() {
        let prompt = build_prompt("Test S.r.l.", &test_status());
        assert!(prompt.contains("\"applicable\": true"));
        assert!(prompt.contains("Art. 21(2)(j)"));
        assert!(prompt.contains("Test S.r.l."));
    }

    #[test]
    fn prompt_ends_with_assistant_tag() {
        let prompt = build_prompt("Test S.r.l.", &test_status());
        assert!(prompt.trim_end().ends_with("<|im_start|>assistant"));
    }

    #[test]
    fn default_params_are_deterministic() {
        let params = InferenceParams::default();
        assert_eq!(params.temperature, 0.0);
        assert_eq!(params.max_tokens, 2048);
    }
}
