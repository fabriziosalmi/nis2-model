//! Quick demo: run the full engine and print the JSON output.

fn main() {
    let profile = nis2_rules::schema::CompanyProfile {
        name: "Acme Energia S.p.A.".into(),
        sector: "energy".into(),
        sub_sector: Some("electricity_distribution".into()),
        employees: 250,
        annual_revenue_eur_m: 180.0,
        balance_sheet_eur_m: 150.0,
        services: vec!["electricity_distribution".into(), "grid_management".into()],
        member_states: vec!["IT".into(), "DE".into()],
    };

    let result = nis2_rules::engine::evaluate(&profile);
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
