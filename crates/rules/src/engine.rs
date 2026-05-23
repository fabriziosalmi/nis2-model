//! Core rule engine — evaluates compliance deterministically.
//!
//! Produces a complete [`ComplianceStatus`] with:
//! - Applicability determination (Art. 2)
//! - Entity classification (Art. 3)
//! - Full obligation catalog (Art. 20, 21, 23)
//! - Sanction calculation (Art. 34)
//! - Incident reporting deadlines (Art. 23)

use crate::obligations;
use crate::schema::{
    CompanyProfile, ComplianceStatus, EntityCategory, IncidentReporting, Obligation,
};

/// Sectors listed in NIS2 Annex I (highly critical sectors).
pub const ANNEX_I_SECTORS: &[&str] = &[
    "energy",
    "transport",
    "banking",
    "financial_market_infrastructure",
    "health",
    "drinking_water",
    "waste_water",
    "digital_infrastructure",
    "ict_service_management_b2b",
    "public_administration",
    "space",
];

/// Sectors listed in NIS2 Annex II (other critical sectors).
pub const ANNEX_II_SECTORS: &[&str] = &[
    "postal_courier",
    "waste_management",
    "chemicals",
    "food",
    "manufacturing",
    "digital_providers",
    "research",
];

/// Size thresholds per EU Recommendation 2003/361/EC.
const MEDIUM_ENTERPRISE_EMPLOYEES: u32 = 50;
const MEDIUM_ENTERPRISE_REVENUE_EUR_M: f64 = 10.0;

/// Evaluate NIS2 compliance for a given company profile.
///
/// This is the single deterministic entry point — given the same input,
/// it will always produce the same output, with zero ambiguity.
pub fn evaluate(profile: &CompanyProfile) -> ComplianceStatus {
    let in_annex_i = ANNEX_I_SECTORS.contains(&profile.sector.as_str());
    let in_annex_ii = ANNEX_II_SECTORS.contains(&profile.sector.as_str());
    
    // An enterprise exceeds the small enterprise threshold if:
    // - Headcount is >= 50
    // - OR BOTH Turnover AND Balance Sheet are >= 10M
    let meets_financial_threshold = profile.annual_revenue_eur_m >= MEDIUM_ENTERPRISE_REVENUE_EUR_M
        && profile.balance_sheet_eur_m >= MEDIUM_ENTERPRISE_REVENUE_EUR_M;
    let meets_size_threshold = profile.employees >= MEDIUM_ENTERPRISE_EMPLOYEES
        || meets_financial_threshold;

    let (applicable, category) = match (in_annex_i, in_annex_ii, meets_size_threshold) {
        (true, _, true) => (true, EntityCategory::Essential),
        (_, true, true) => (true, EntityCategory::Important),
        // Special cases: certain digital infrastructure is always in scope (Art. 2(2))
        (true, _, false) if is_always_in_scope(profile) => {
            (true, EntityCategory::Essential)
        }
        _ => (false, EntityCategory::OutOfScope),
    };

    if !applicable {
        return ComplianceStatus {
            applicable: false,
            category: EntityCategory::OutOfScope,
            obligations: Vec::new(),
            max_sanction_eur: None,
            incident_reporting: None,
            transposition_notes: Vec::new(),
        };
    }

    // Build full obligation set for in-scope entities
    let obligations = build_obligations();

    let mut max_sanction_eur = Some(calculate_max_sanction(&category, profile.annual_revenue_eur_m));
    let mut transposition_notes = Vec::new();

    // Check Italian transposition (D.Lgs. 138/2024)
    if profile.member_states.contains(&"IT".to_string()) {
        transposition_notes.push(
            "L'Italia ha recepito la direttiva con il D.Lgs. 138/2024. Le autorità nazionali competenti \
             (in particolare ACN - Agenzia per la Cybersicurezza Nazionale) definiscono termini, \
             registri specifici e adempimenti per i soggetti essenziali e importanti.".to_string()
        );

        if profile.sector == "public_administration" {
            // Public administrations in Italy are exempt from administrative fines under D.Lgs. 138/2024
            max_sanction_eur = Some(0.0);
            transposition_notes.push(
                "Nota per la PA: ai sensi del D.Lgs. 138/2024 (Art. 38/39), le amministrazioni pubbliche \
                 italiane sono escluse dall'applicazione delle sanzioni amministrative pecuniarie, \
                 ferma restando la responsabilità disciplinare e dirigenziale e l'obbligo di conformità.".to_string()
            );
        }
    }

    // Art. 23(4) incident reporting deadlines — same for all in-scope entities
    let incident_reporting = Some(IncidentReporting {
        early_warning_hours: 24,
        notification_hours: 72,
        final_report_days: 30,
    });

    ComplianceStatus {
        applicable,
        category,
        obligations,
        max_sanction_eur,
        incident_reporting,
        transposition_notes,
    }
}

/// Build the complete obligation set for in-scope entities.
fn build_obligations() -> Vec<Obligation> {
    let mut all = Vec::new();
    all.extend(obligations::art20_obligations());
    all.extend(obligations::art21_obligations());
    all.extend(obligations::art23_obligations());
    all
}

/// Certain entities are in scope regardless of size (Art. 2(2)).
fn is_always_in_scope(profile: &CompanyProfile) -> bool {
    match profile.sector.as_str() {
        "ict_service_management_b2b" | "public_administration" => true,
        "digital_infrastructure" => {
            let is_special_service = |s: &str| {
                let lower = s.to_lowercase();
                lower.contains("dns")
                    || lower.contains("tld")
                    || lower.contains("trust")
                    || lower.contains("telecom")
                    || lower.contains("connettiv")
                    || lower.contains("comunicaz")
                    || lower.contains("communication")
            };

            if let Some(ref sub) = profile.sub_sector {
                if is_special_service(sub) {
                    return true;
                }
            }
            if profile.services.iter().any(|s| is_special_service(s)) {
                return true;
            }

            // Cloud, hosting, datacenters, CDNs are NOT always in scope
            let has_other_services = if let Some(ref sub) = profile.sub_sector {
                let lower = sub.to_lowercase();
                lower.contains("cloud") || lower.contains("hosting") || lower.contains("datacent") || lower.contains("cdn")
            } else {
                profile.services.iter().any(|s| {
                    let lower = s.to_lowercase();
                    lower.contains("cloud") || lower.contains("hosting") || lower.contains("datacent") || lower.contains("cdn")
                })
            };

            if has_other_services {
                false
            } else {
                true
            }
        }
        _ => false,
    }
}

/// Calculate maximum sanction per Art. 34 NIS2.
fn calculate_max_sanction(category: &EntityCategory, annual_revenue_eur_m: f64) -> f64 {
    match category {
        // Essential: max(€10M, 2% of annual worldwide turnover)
        EntityCategory::Essential => {
            f64::max(10_000_000.0, annual_revenue_eur_m * 1_000_000.0 * 0.02)
        }
        // Important: max(€7M, 1.4% of annual worldwide turnover)
        EntityCategory::Important => {
            f64::max(7_000_000.0, annual_revenue_eur_m * 1_000_000.0 * 0.014)
        }
        EntityCategory::OutOfScope => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::ObligationStatus;

    fn test_profile(sector: &str, employees: u32, revenue: f64) -> CompanyProfile {
        CompanyProfile {
            name: "Test S.r.l.".into(),
            sector: sector.into(),
            sub_sector: None,
            employees,
            annual_revenue_eur_m: revenue,
            balance_sheet_eur_m: revenue,
            services: Vec::new(),
            member_states: vec!["IT".into()],
        }
    }

    #[test]
    fn large_energy_company_is_essential() {
        let result = evaluate(&test_profile("energy", 500, 100.0));
        assert!(result.applicable);
        assert_eq!(result.category, EntityCategory::Essential);
    }

    #[test]
    fn medium_food_company_is_important() {
        let result = evaluate(&test_profile("food", 60, 15.0));
        assert!(result.applicable);
        assert_eq!(result.category, EntityCategory::Important);
    }

    #[test]
    fn small_unrelated_company_is_out_of_scope() {
        let result = evaluate(&test_profile("retail", 10, 1.0));
        assert!(!result.applicable);
        assert_eq!(result.category, EntityCategory::OutOfScope);
        assert!(result.obligations.is_empty());
        assert!(result.max_sanction_eur.is_none());
        assert!(result.incident_reporting.is_none());
    }

    #[test]
    fn digital_infra_always_in_scope() {
        let result = evaluate(&test_profile("digital_infrastructure", 5, 0.5));
        assert!(result.applicable);
    }

    #[test]
    fn digital_infra_cloud_small_is_out_of_scope() {
        let mut p = test_profile("digital_infrastructure", 5, 0.5);
        p.services = vec!["Cloud Provider".to_string()];
        let result = evaluate(&p);
        assert!(!result.applicable);
        assert_eq!(result.category, EntityCategory::OutOfScope);
    }

    #[test]
    fn digital_infra_dns_small_is_essential() {
        let mut p = test_profile("digital_infrastructure", 5, 0.5);
        p.services = vec!["DNS Resolution".to_string()];
        let result = evaluate(&p);
        assert!(result.applicable);
        assert_eq!(result.category, EntityCategory::Essential);
    }

    #[test]
    fn sanction_essential_uses_two_percent() {
        let result = evaluate(&test_profile("energy", 500, 1_000.0));
        // 2% of €1B = €20M > €10M floor
        assert_eq!(result.max_sanction_eur, Some(20_000_000.0));
    }

    #[test]
    fn sanction_essential_uses_floor() {
        let result = evaluate(&test_profile("energy", 500, 100.0));
        // 2% of €100M = €2M < €10M floor → floor applies
        assert_eq!(result.max_sanction_eur, Some(10_000_000.0));
    }

    #[test]
    fn in_scope_entity_has_16_obligations() {
        let result = evaluate(&test_profile("energy", 500, 100.0));
        // 2 (Art.20) + 10 (Art.21) + 4 (Art.23) = 16
        assert_eq!(result.obligations.len(), 16);
    }

    #[test]
    fn all_obligations_are_pending_initially() {
        let result = evaluate(&test_profile("energy", 500, 100.0));
        for o in &result.obligations {
            assert_eq!(o.status, ObligationStatus::Pending);
        }
    }

    #[test]
    fn incident_reporting_deadlines_correct() {
        let result = evaluate(&test_profile("energy", 500, 100.0));
        let ir = result.incident_reporting.unwrap();
        assert_eq!(ir.early_warning_hours, 24);
        assert_eq!(ir.notification_hours, 72);
        assert_eq!(ir.final_report_days, 30);
    }

    #[test]
    fn out_of_scope_has_no_obligations() {
        let result = evaluate(&test_profile("retail", 3, 0.1));
        assert!(result.obligations.is_empty());
    }

    #[test]
    fn output_is_valid_json() {
        let result = evaluate(&test_profile("energy", 500, 100.0));
        let json = serde_json::to_string_pretty(&result).unwrap();
        assert!(json.contains("\"applicable\": true"));
        assert!(json.contains("Art. 21(2)(j)"));
    }

    // ---- Edge case tests (Round 2) ----

    #[test]
    fn exactly_50_employees_meets_threshold() {
        let result = evaluate(&test_profile("food", 50, 5.0));
        assert!(result.applicable, "50 employees should meet threshold");
        assert_eq!(result.category, EntityCategory::Important);
    }

    #[test]
    fn exactly_49_employees_below_threshold() {
        let result = evaluate(&test_profile("food", 49, 5.0));
        assert!(!result.applicable, "49 employees + 5M rev should not meet threshold");
    }

    #[test]
    fn exactly_10m_revenue_meets_threshold() {
        let result = evaluate(&test_profile("food", 10, 10.0));
        assert!(result.applicable, "10M revenue should meet threshold");
        assert_eq!(result.category, EntityCategory::Important);
    }

    #[test]
    fn revenue_9_99m_below_threshold() {
        let result = evaluate(&test_profile("food", 10, 9.99));
        assert!(!result.applicable, "9.99M revenue + 10 emp should not meet threshold");
    }

    #[test]
    fn small_annex_ii_is_out_of_scope() {
        let result = evaluate(&test_profile("chemicals", 30, 5.0));
        assert!(!result.applicable, "Small Annex II entity should be out of scope");
        assert_eq!(result.category, EntityCategory::OutOfScope);
    }

    #[test]
    fn annex_i_below_threshold_non_special_is_out_of_scope() {
        // energy is Annex I but NOT in is_always_in_scope
        let result = evaluate(&test_profile("energy", 10, 1.0));
        assert!(!result.applicable, "Small energy company should be out of scope");
    }

    #[test]
    fn zero_employees_with_high_revenue_meets_threshold() {
        // A holding company with 0 employees but high revenue
        let result = evaluate(&test_profile("banking", 0, 500.0));
        assert!(result.applicable, "0 employees but 500M revenue should meet threshold");
        assert_eq!(result.category, EntityCategory::Essential);
    }

    #[test]
    fn public_administration_always_in_scope() {
        let result = evaluate(&test_profile("public_administration", 3, 0.1));
        assert!(result.applicable, "public_administration should always be in scope");
        assert_eq!(result.category, EntityCategory::Essential);
    }

    #[test]
    fn ict_service_management_always_in_scope() {
        let result = evaluate(&test_profile("ict_service_management_b2b", 2, 0.05));
        assert!(result.applicable, "ict_service_management_b2b should always be in scope");
        assert_eq!(result.category, EntityCategory::Essential);
    }

    #[test]
    fn sector_is_case_sensitive() {
        // Engine expects lowercase — uppercase sector should be OutOfScope
        let result = evaluate(&test_profile("Energy", 500, 100.0));
        assert!(!result.applicable, "Uppercase 'Energy' should not match (API normalizes, engine does not)");
    }

    #[test]
    fn sanction_important_uses_1_4_percent() {
        let result = evaluate(&test_profile("food", 200, 1_000.0));
        // 1.4% of €1B = €14M > €7M floor
        assert_eq!(result.max_sanction_eur, Some(14_000_000.0));
    }

    #[test]
    fn sanction_important_uses_floor() {
        let result = evaluate(&test_profile("food", 60, 20.0));
        // 1.4% of €20M = €280K < €7M floor → floor applies
        assert_eq!(result.max_sanction_eur, Some(7_000_000.0));
    }

    #[test]
    fn financial_thresholds_require_both() {
        // Employees = 10 (< 50)
        // Revenue = 12.0 (>= 10.0)
        // Balance sheet = 8.0 (< 10.0)
        // Legally a small enterprise, so OutOfScope
        let profile = CompanyProfile {
            name: "Test Small S.r.l.".into(),
            sector: "food".into(),
            sub_sector: None,
            employees: 10,
            annual_revenue_eur_m: 12.0,
            balance_sheet_eur_m: 8.0,
            services: Vec::new(),
            member_states: vec!["IT".into()],
        };
        let result = evaluate(&profile);
        assert!(!result.applicable, "Must be OutOfScope if balance sheet is below 10M, even if revenue is above 10M");
    }

    #[test]
    fn italian_public_administration_has_zero_sanction_and_notes() {
        let profile = CompanyProfile {
            name: "Comune di Prova".into(),
            sector: "public_administration".into(),
            sub_sector: None,
            employees: 150,
            annual_revenue_eur_m: 0.0,
            balance_sheet_eur_m: 0.0,
            services: Vec::new(),
            member_states: vec!["IT".into()],
        };
        let result = evaluate(&profile);
        assert!(result.applicable);
        assert_eq!(result.max_sanction_eur, Some(0.0));
        assert!(!result.transposition_notes.is_empty());
        assert!(result.transposition_notes[1].contains("escluse dall'applicazione delle sanzioni"));
    }
}
