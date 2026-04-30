//! Export dataset to JSON for the web chatbot.
//!
//! Enriches each entry with severity, deadline, and standards mapping
//! derived from the category at export time. No struct changes needed.
//!
//! Usage: cargo run --bin export_dataset > docs/public/dataset.json

use nis2_chat::dataset;

fn severity_for(cat: &str) -> &'static str {
    match cat {
        c if c.contains("incident") => "critical",
        c if c.contains("sanctions") => "critical",
        c if c.contains("governance") => "high",
        c if c.contains("access_control") => "high",
        c if c.contains("encryption") => "high",
        c if c.contains("business_continuity") => "high",
        c if c.contains("supply_chain") => "high",
        c if c.contains("vulnerability") => "high",
        c if c.contains("network") => "medium",
        c if c.contains("detection") => "medium",
        c if c.contains("email") => "medium",
        c if c.contains("risk") => "medium",
        c if c.contains("physical") => "medium",
        c if c.contains("development") => "medium",
        c if c.contains("asset") => "medium",
        c if c.contains("remote") => "medium",
        _ => "info",
    }
}

fn deadline_for(cat: &str) -> &'static str {
    match cat {
        c if c.contains("incident") => "24h-72h-30d",
        c if c.contains("sanctions") => "",
        c if c.contains("risk") => "annual",
        c if c.contains("business_continuity") => "annual",
        _ => "",
    }
}

fn standards_for(cat: &str) -> Vec<serde_json::Value> {
    let stubs = match cat {
        c if c.contains("incident") => vec!["ISO 27001 A.16", "NIST CSF RS.RP", "CIS 17"],
        c if c.contains("governance") => vec!["ISO 27001 A.5", "NIST CSF GV.OC"],
        c if c.contains("access_control") => vec!["ISO 27001 A.9", "NIST CSF PR.AC", "CIS 5,6"],
        c if c.contains("encryption") => vec!["ISO 27001 A.10", "NIST CSF PR.DS"],
        c if c.contains("business_continuity") => vec!["ISO 27001 A.17", "NIST CSF RC.RP"],
        c if c.contains("supply_chain") => vec!["ISO 27001 A.15", "NIST CSF ID.SC"],
        c if c.contains("vulnerability") => vec!["ISO 27001 A.12.6", "NIST CSF ID.RA", "CIS 7"],
        c if c.contains("risk") => vec!["ISO 27001 A.8", "NIST CSF ID.RA"],
        c if c.contains("network") => vec!["ISO 27001 A.13", "NIST CSF PR.AC", "CIS 12,13"],
        c if c.contains("detection") => vec!["ISO 27001 A.12.4", "NIST CSF DE.CM", "CIS 8"],
        c if c.contains("email") => vec!["ISO 27001 A.13.2", "CIS 9"],
        c if c.contains("documentation") => vec!["ISO 27001 A.7", "NIST CSF GV.PO"],
        c if c.contains("remote") => vec!["ISO 27001 A.6.2", "NIST CSF PR.AC"],
        c if c.contains("physical") => vec!["ISO 27001 A.11", "NIST CSF PR.AC"],
        c if c.contains("legal") => vec!["ISO 27001 A.18"],
        c if c.contains("sanctions") => vec!["NIS2 Art. 34"],
        c if c.contains("asset") => vec!["ISO 27001 A.8", "NIST CSF ID.AM", "CIS 1,2"],
        c if c.contains("development") => vec!["ISO 27001 A.14", "NIST CSF PR.IP", "CIS 16"],
        _ => vec![],
    };
    
    stubs.into_iter().map(|s| {
        let url = if s.starts_with("ISO") { "https://www.iso.org/isoiec-27001-information-security.html" }
        else if s.starts_with("NIST") { "https://www.nist.gov/cyberframework" }
        else if s.starts_with("CIS") { "https://www.cisecurity.org/controls/" }
        else { "https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32022L2555" };
        serde_json::json!({ "label": s, "url": url })
    }).collect()
}

fn main() {
    let entries = dataset::build_dataset();
    let mut items: Vec<serde_json::Value> = Vec::new();
    for e in &entries {
        items.push(serde_json::json!({
            "q": e.question,
            "a": e.answer,
            "c": e.category,
            "f": e.follow_ups,
            "s": severity_for(&e.category),
            "d": deadline_for(&e.category),
            "r": standards_for(&e.category),
        }));
    }
    println!("{}", serde_json::to_string(&items).unwrap());
}
