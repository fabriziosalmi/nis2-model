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
    let meets_size_threshold = profile.employees >= MEDIUM_ENTERPRISE_EMPLOYEES
        || profile.annual_revenue_eur_m >= MEDIUM_ENTERPRISE_REVENUE_EUR_M;

    let (applicable, category) = match (in_annex_i, in_annex_ii, meets_size_threshold) {
        (true, _, true) => (true, EntityCategory::Essential),
        (_, true, true) => (true, EntityCategory::Important),
        // Special cases: certain digital infrastructure is always in scope (Art. 2(2))
        (true, _, false) if is_always_in_scope(&profile.sector) => {
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
        };
    }

    // Build full obligation set for in-scope entities
    let obligations = build_obligations();

    let max_sanction_eur = Some(calculate_max_sanction(&category, profile.annual_revenue_eur_m));

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
fn is_always_in_scope(sector: &str) -> bool {
    matches!(
        sector,
        "digital_infrastructure" | "ict_service_management_b2b" | "public_administration"
    )
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
}
