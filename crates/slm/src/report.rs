//! Deterministic report generator.
//!
//! Transforms a [`ComplianceStatus`] into a formal Italian compliance report.
//! This is the template-based fallback that guarantees zero hallucination:
//! every word in the output is derived directly from the structured input.

use nis2_rules::schema::{ComplianceStatus, EntityCategory};

/// Generate a formal Italian compliance report from structured data.
///
/// This function produces a complete report with no invented content —
/// every statement is derived directly from the [`ComplianceStatus`] fields.
pub fn generate_report(company_name: &str, status: &ComplianceStatus) -> String {
    let mut report = String::with_capacity(4096);

    // Section 1: Scope
    report.push_str("## Ambito di applicazione\n\n");
    report.push_str(&generate_scope_section(company_name, status));
    report.push('\n');

    // Section 2: Obligations
    report.push_str("## Obblighi rilevanti\n\n");
    report.push_str(&generate_obligations_section(status));
    report.push('\n');

    // Section 3: Sanctions
    report.push_str("## Sanzioni potenziali\n\n");
    report.push_str(&generate_sanctions_section(status));
    report.push('\n');

    // Section 4: Incident reporting
    if status.incident_reporting.is_some() {
        report.push_str("## Segnalazione incidenti\n\n");
        report.push_str(&generate_incident_section(status));
        report.push('\n');
    }

    report
}

fn generate_scope_section(company_name: &str, status: &ComplianceStatus) -> String {
    if !status.applicable {
        return format!(
            "Sulla base dei dati forniti, risulta che l'azienda \"{}\" \
             non rientra nell'ambito di applicazione della Direttiva (UE) 2022/2555 (NIS2). \
             Pertanto, gli obblighi previsti dalla direttiva non sono applicabili al soggetto in esame.\n",
            company_name
        );
    }

    let category_text = match status.category {
        EntityCategory::Essential => "soggetto essenziale ai sensi dell'Art. 3(1)",
        EntityCategory::Important => "soggetto importante ai sensi dell'Art. 3(2)",
        EntityCategory::OutOfScope => "fuori ambito",
    };

    format!(
        "Sulla base dei dati forniti, risulta che l'azienda \"{}\" \
         rientra nell'ambito di applicazione della Direttiva (UE) 2022/2555 (NIS2), \
         in qualità di {}. Il soggetto è pertanto tenuto al rispetto degli obblighi \
         di cui agli articoli 20, 21 e 23 della direttiva.\n",
        company_name, category_text
    )
}

fn generate_obligations_section(status: &ComplianceStatus) -> String {
    if status.obligations.is_empty() {
        return "Nessun obbligo applicabile.\n".to_string();
    }

    let mut section = format!(
        "Sulla base della classificazione del soggetto, risultano applicabili \
         i seguenti {} obblighi previsti dalla Direttiva NIS2:\n\n",
        status.obligations.len()
    );

    for obligation in &status.obligations {
        section.push_str(&format!(
            "- **{}**: {}\n",
            obligation.article_ref, obligation.description
        ));
    }

    section
}

fn generate_sanctions_section(status: &ComplianceStatus) -> String {
    match status.max_sanction_eur {
        Some(sanction) => {
            let (article, percentage) = match status.category {
                EntityCategory::Essential => ("Art. 34(4)", "il 2%"),
                EntityCategory::Important => ("Art. 34(5)", "l'1,4%"),
                EntityCategory::OutOfScope => return "Nessuna sanzione applicabile.\n".to_string(),
            };

            format!(
                "Ai sensi dell'{} della Direttiva NIS2, in caso di violazione degli \
                 articoli 21 o 23, il soggetto è esposto a sanzioni amministrative pecuniarie \
                 fino a un massimo di €{:.0}. Tale importo corrisponde al maggiore tra il \
                 massimale fisso previsto dalla direttiva e {} del fatturato mondiale annuo.\n",
                article,
                sanction,
                percentage
            )
        }
        None => "Nessuna sanzione applicabile.\n".to_string(),
    }
}

fn generate_incident_section(status: &ComplianceStatus) -> String {
    match &status.incident_reporting {
        Some(ir) => {
            format!(
                "Ai sensi dell'Art. 23 della Direttiva NIS2, il soggetto è tenuto a rispettare \
                 le seguenti tempistiche di segnalazione in caso di incidente significativo:\n\n\
                 - **Preallarme** (Art. 23(4)(a)): entro {} ore dalla conoscenza dell'incidente.\n\
                 - **Notifica completa** (Art. 23(4)(b)): entro {} ore dalla conoscenza dell'incidente.\n\
                 - **Relazione finale** (Art. 23(4)(d)): entro {} giorni dalla notifica dell'incidente.\n",
                ir.early_warning_hours, ir.notification_hours, ir.final_report_days
            )
        }
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nis2_rules::engine;
    use nis2_rules::schema::CompanyProfile;

    fn essential_status() -> ComplianceStatus {
        engine::evaluate(&CompanyProfile {
            name: "Acme Energia S.p.A.".into(),
            sector: "energy".into(),
            sub_sector: None,
            employees: 250,
            annual_revenue_eur_m: 180.0,
            balance_sheet_eur_m: 150.0,
            services: vec![],
            member_states: vec!["IT".into()],
        })
    }

    fn out_of_scope_status() -> ComplianceStatus {
        engine::evaluate(&CompanyProfile {
            name: "Piccolo Negozio S.r.l.".into(),
            sector: "retail".into(),
            sub_sector: None,
            employees: 5,
            annual_revenue_eur_m: 0.3,
            balance_sheet_eur_m: 0.2,
            services: vec![],
            member_states: vec!["IT".into()],
        })
    }

    #[test]
    fn essential_report_has_all_sections() {
        let report = generate_report("Acme Energia S.p.A.", &essential_status());
        assert!(report.contains("## Ambito di applicazione"));
        assert!(report.contains("## Obblighi rilevanti"));
        assert!(report.contains("## Sanzioni potenziali"));
        assert!(report.contains("## Segnalazione incidenti"));
    }

    #[test]
    fn essential_report_cites_classification() {
        let report = generate_report("Acme Energia", &essential_status());
        assert!(report.contains("soggetto essenziale ai sensi dell'Art. 3(1)"));
    }

    #[test]
    fn essential_report_lists_all_obligations() {
        let report = generate_report("Acme Energia", &essential_status());
        assert!(report.contains("Art. 20(1)"));
        assert!(report.contains("Art. 21(2)(a)"));
        assert!(report.contains("Art. 21(2)(j)"));
        assert!(report.contains("Art. 23(4)(a)"));
    }

    #[test]
    fn essential_report_shows_sanction() {
        let report = generate_report("Acme Energia", &essential_status());
        // 2% of €180M = €3.6M < €10M floor → €10M
        assert!(report.contains("€10000000"));
        assert!(report.contains("Art. 34(4)"));
    }

    #[test]
    fn essential_report_shows_deadlines() {
        let report = generate_report("Acme Energia", &essential_status());
        assert!(report.contains("entro 24 ore"));
        assert!(report.contains("entro 72 ore"));
        assert!(report.contains("entro 30 giorni"));
    }

    #[test]
    fn out_of_scope_report_states_non_applicability() {
        let report = generate_report("Piccolo Negozio", &out_of_scope_status());
        assert!(report.contains("non rientra nell'ambito di applicazione"));
        assert!(!report.contains("## Segnalazione incidenti"));
    }

    #[test]
    fn report_never_contains_uncertain_language() {
        let report = generate_report("Test", &essential_status());
        let forbidden = ["potrebbe", "forse", "probabilmente", "eventualmente"];
        for word in &forbidden {
            assert!(
                !report.to_lowercase().contains(word),
                "Report contains forbidden word: {word}"
            );
        }
    }

    #[test]
    fn report_uses_deterministic_formula() {
        let report = generate_report("Test", &essential_status());
        assert!(report.contains("Sulla base dei dati forniti, risulta che"));
    }
}
