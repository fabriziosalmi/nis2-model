//! JSON Schema validation — runtime schema enforcement for inputs and outputs.
//!
//! Uses [`schemars`] for schema generation and [`jsonschema`] for validation.

use anyhow::Result;
use jsonschema::Validator;
use schemars::schema_for;

use crate::schema::{CompanyProfile, ComplianceStatus};

/// Generate the JSON Schema for [`CompanyProfile`] (engine input).
pub fn company_profile_schema() -> serde_json::Value {
    serde_json::to_value(schema_for!(CompanyProfile)).expect("Schema serialization cannot fail")
}

/// Generate the JSON Schema for [`ComplianceStatus`] (engine output).
pub fn compliance_status_schema() -> serde_json::Value {
    serde_json::to_value(schema_for!(ComplianceStatus)).expect("Schema serialization cannot fail")
}

/// Validate a JSON value against the [`CompanyProfile`] schema.
pub fn validate_company_profile(value: &serde_json::Value) -> Result<()> {
    let schema = company_profile_schema();
    let validator = Validator::new(&schema)
        .map_err(|e| anyhow::anyhow!("Invalid schema: {e}"))?;

    if !validator.is_valid(value) {
        // Collect detailed errors
        let result = validator.iter_errors(value);
        let messages: Vec<String> = result
            .map(|e| format!("{}: {}", e.instance_path, e))
            .collect();
        anyhow::bail!(
            "CompanyProfile validation failed:\n{}",
            messages.join("\n")
        );
    }
    Ok(())
}

/// Validate a JSON value against the [`ComplianceStatus`] schema.
pub fn validate_compliance_status(value: &serde_json::Value) -> Result<()> {
    let schema = compliance_status_schema();
    let validator = Validator::new(&schema)
        .map_err(|e| anyhow::anyhow!("Invalid schema: {e}"))?;

    if !validator.is_valid(value) {
        let result = validator.iter_errors(value);
        let messages: Vec<String> = result
            .map(|e| format!("{}: {}", e.instance_path, e))
            .collect();
        anyhow::bail!(
            "ComplianceStatus validation failed:\n{}",
            messages.join("\n")
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_company_profile_passes() {
        let json = serde_json::json!({
            "name": "Acme S.r.l.",
            "sector": "energy",
            "employees": 500,
            "annual_revenue_eur_m": 100.0,
            "balance_sheet_eur_m": 80.0,
            "services": ["electricity_distribution"],
            "member_states": ["IT"]
        });
        assert!(validate_company_profile(&json).is_ok());
    }

    #[test]
    fn missing_required_field_fails() {
        let json = serde_json::json!({
            "name": "Acme S.r.l.",
            // missing "sector"
            "employees": 500,
            "annual_revenue_eur_m": 100.0,
            "balance_sheet_eur_m": 80.0,
            "services": [],
            "member_states": ["IT"]
        });
        assert!(validate_company_profile(&json).is_err());
    }

    #[test]
    fn wrong_type_fails() {
        let json = serde_json::json!({
            "name": "Acme S.r.l.",
            "sector": "energy",
            "employees": "not_a_number", // wrong type
            "annual_revenue_eur_m": 100.0,
            "balance_sheet_eur_m": 80.0,
            "services": [],
            "member_states": ["IT"]
        });
        assert!(validate_company_profile(&json).is_err());
    }

    #[test]
    fn engine_output_validates_against_schema() {
        use crate::engine::evaluate;

        let profile = CompanyProfile {
            name: "Test S.r.l.".into(),
            sector: "energy".into(),
            sub_sector: None,
            employees: 500,
            annual_revenue_eur_m: 100.0,
            balance_sheet_eur_m: 80.0,
            services: vec![],
            member_states: vec!["IT".into()],
        };

        let status = evaluate(&profile);
        let json = serde_json::to_value(&status).unwrap();
        assert!(validate_compliance_status(&json).is_ok());
    }
}
