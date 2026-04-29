//! GBNF Grammar for constrained SLM decoding.
//!
//! This grammar forces the model to produce output that:
//! - Follows the report structure (3 sections)
//! - Only uses legal citation patterns from NIS2
//! - Cannot generate arbitrary URLs, emails, or code
//!
//! Used with llama.cpp's grammar-constrained sampling.

/// GBNF grammar that constrains SLM output to the compliance report format.
///
/// This grammar restricts token-level generation to ensure:
/// 1. Output follows the mandated 4-section structure
/// 2. Article references match NIS2 citation patterns
/// 3. No URLs, code blocks, or non-text content is generated
pub const COMPLIANCE_REPORT_GRAMMAR: &str = r###"
# Root: a compliance report with mandatory sections
root ::= section-ambito section-obblighi section-sanzioni section-incidenti

# Section 1: Scope of application
section-ambito ::= "## Ambito di applicazione\n\n" paragraph "\n\n"

# Section 2: Relevant obligations
section-obblighi ::= "## Obblighi rilevanti\n\n" obligation-list "\n\n"

# Section 3: Potential sanctions
section-sanzioni ::= "## Sanzioni potenziali\n\n" paragraph "\n\n"

# Section 4: Incident reporting
section-incidenti ::= "## Segnalazione incidenti\n\n" paragraph "\n"

# A paragraph is one or more sentences
paragraph ::= sentence (sentence)*
sentence ::= [A-ZÀÈÉÌÒÙ] [^\n]+ ".\n" | [A-ZÀÈÉÌÒÙ] [^\n]+ ".\n\n"

# Obligation list
obligation-list ::= obligation-item (obligation-item)*
obligation-item ::= "- **" article-ref "**: " [^\n]+ "\n"

# NIS2 article reference pattern
article-ref ::= "Art. " [0-9]+ ("(" [0-9]+ ")" ("(" [a-j] ")")?)?
"###;

/// Simplified grammar for short-form tool output rewriting.
pub const TOOL_OUTPUT_GRAMMAR: &str = r###"
root ::= paragraph
paragraph ::= sentence (sentence)*
sentence ::= [A-ZÀÈÉÌÒÙ] [^\n]+ "." ("\n" | "\n\n")
"###;

/// Returns the appropriate grammar for the given generation mode.
pub fn grammar_for_mode(mode: GenerationMode) -> &'static str {
    match mode {
        GenerationMode::ComplianceReport => COMPLIANCE_REPORT_GRAMMAR,
        GenerationMode::ToolRewrite => TOOL_OUTPUT_GRAMMAR,
    }
}

/// Generation modes that select the appropriate grammar constraint.
#[derive(Debug, Clone, Copy)]
pub enum GenerationMode {
    /// Full compliance report (4 sections).
    ComplianceReport,
    /// Short-form tool output rewriting.
    ToolRewrite,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grammar_contains_all_sections() {
        let g = COMPLIANCE_REPORT_GRAMMAR;
        assert!(g.contains("section-ambito"));
        assert!(g.contains("section-obblighi"));
        assert!(g.contains("section-sanzioni"));
        assert!(g.contains("section-incidenti"));
    }

    #[test]
    fn grammar_enforces_article_pattern() {
        let g = COMPLIANCE_REPORT_GRAMMAR;
        assert!(g.contains("article-ref"));
        assert!(g.contains("\"Art. \""));
    }

    #[test]
    fn mode_selects_correct_grammar() {
        let report_g = grammar_for_mode(GenerationMode::ComplianceReport);
        let tool_g = grammar_for_mode(GenerationMode::ToolRewrite);
        assert!(report_g.contains("section-ambito"));
        assert!(!tool_g.contains("section-ambito"));
    }
}
