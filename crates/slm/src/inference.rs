//! Inference trait and backends.
//!
//! Abstracts over SLM inference backends:
//! - [`TemplateBackend`] — deterministic template-based generation (no model needed)
//! - Future: `LlamaCppBackend` — llama.cpp GGUF inference (behind feature flag)

use anyhow::Result;

use crate::prompt::InferenceParams;

/// Trait for SLM inference backends.
pub trait InferenceBackend {
    /// Generate text from a prompt with the given parameters.
    fn generate(&self, prompt: &str, params: &InferenceParams) -> Result<String>;

    /// Returns the backend name for logging.
    fn name(&self) -> &str;
}

/// Template-based backend — produces deterministic output without a model.
///
/// This is the default backend used when no GGUF model is available.
/// It extracts structured data from the prompt context and produces
/// a formatted report using Rust string templates.
///
/// This guarantees:
/// - Zero hallucination (output is purely derived from input data)
/// - No model download required
/// - Instant generation
/// - Deterministic output
pub struct TemplateBackend;

impl InferenceBackend for TemplateBackend {
    fn generate(&self, prompt: &str, _params: &InferenceParams) -> Result<String> {
        // Extract the JSON context from the prompt
        let json_start = prompt
            .find("```json\n")
            .map(|i| i + 8)
            .ok_or_else(|| anyhow::anyhow!("No JSON context found in prompt"))?;
        let json_end = prompt[json_start..]
            .find("\n```")
            .map(|i| json_start + i)
            .ok_or_else(|| anyhow::anyhow!("Unterminated JSON block in prompt"))?;

        let json_str = &prompt[json_start..json_end];
        let status: nis2_rules::schema::ComplianceStatus = serde_json::from_str(json_str)?;

        // Extract company name from prompt
        let company_name = prompt
            .find("l'azienda \"")
            .and_then(|i| {
                let start = i + "l'azienda \"".len();
                prompt[start..].find('"').map(|end| &prompt[start..start + end])
            })
            .unwrap_or("N/A");

        Ok(crate::report::generate_report(company_name, &status))
    }

    fn name(&self) -> &str {
        "template"
    }
}

/// Placeholder for the llama.cpp backend.
///
/// When implemented, this will:
/// 1. Load a GGUF model (e.g. Qwen-2.5-0.5B-Instruct)
/// 2. Apply the GBNF grammar constraint
/// 3. Generate with temperature=0.0
/// 4. Post-validate against the compliance schema
pub struct LlamaCppBackend {
    _model_path: String,
}

impl LlamaCppBackend {
    /// Create a new llama.cpp backend (model loading deferred to first call).
    pub fn new(model_path: &str) -> Self {
        Self {
            _model_path: model_path.to_string(),
        }
    }
}

impl InferenceBackend for LlamaCppBackend {
    fn generate(&self, prompt: &str, _params: &InferenceParams) -> Result<String> {
        // Delegate to template backend until llama.cpp is integrated
        tracing::warn!(
            "LlamaCppBackend not yet implemented — falling back to template generation"
        );
        let template = TemplateBackend;
        template.generate(prompt, _params)
    }

    fn name(&self) -> &str {
        "llama.cpp (pending)"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt;
    use nis2_rules::engine;
    use nis2_rules::schema::CompanyProfile;

    fn test_compliance() -> nis2_rules::schema::ComplianceStatus {
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
    fn template_backend_generates_report() {
        let status = test_compliance();
        let full_prompt = prompt::build_prompt("Test S.r.l.", &status);

        let backend = TemplateBackend;
        let params = InferenceParams::default();
        let output = backend.generate(&full_prompt, &params).unwrap();

        assert!(output.contains("Ambito di applicazione"));
        assert!(output.contains("Obblighi rilevanti"));
        assert!(output.contains("Sanzioni potenziali"));
        assert!(output.contains("Test S.r.l."));
    }

    #[test]
    fn template_backend_cites_articles() {
        let status = test_compliance();
        let full_prompt = prompt::build_prompt("Test S.r.l.", &status);

        let backend = TemplateBackend;
        let output = backend.generate(&full_prompt, &InferenceParams::default()).unwrap();

        assert!(output.contains("Art. 21(2)"));
        assert!(output.contains("Art. 20"));
        assert!(output.contains("Art. 23"));
    }

    #[test]
    fn template_backend_name() {
        assert_eq!(TemplateBackend.name(), "template");
    }
}
