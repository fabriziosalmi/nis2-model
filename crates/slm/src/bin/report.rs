//! Report generation demo — produces a full Italian compliance report.

use nis2_rules::{engine, schema::CompanyProfile};
use nis2_slm::{inference::{InferenceBackend, TemplateBackend}, prompt::{self, InferenceParams}};

fn main() {
    let profile = CompanyProfile {
        name: "Meridiana Energia S.p.A.".into(),
        sector: "energy".into(),
        sub_sector: Some("electricity_distribution".into()),
        employees: 350,
        annual_revenue_eur_m: 220.0,
        balance_sheet_eur_m: 190.0,
        services: vec!["electricity_distribution".into(), "smart_grid".into()],
        member_states: vec!["IT".into()],
    };

    let status = engine::evaluate(&profile);
    let full_prompt = prompt::build_prompt(&profile.name, &status);

    eprintln!("=== PROMPT ({} chars) ===\n", full_prompt.len());

    let backend = TemplateBackend;
    let report = backend.generate(&full_prompt, &InferenceParams::default()).unwrap();

    println!("{report}");
}
