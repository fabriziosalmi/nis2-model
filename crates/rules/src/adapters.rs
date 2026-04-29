//! WASM Adapter system via Extism.
//!
//! Adapters are WASM plugins that transform heterogeneous input formats
//! (web forms, XML audits, vulnerability reports) into the standardized
//! [`CompanyProfile`] JSON consumed by the rule engine.

use anyhow::Result;
use extism::{Manifest, Plugin, Wasm};
use tracing::info;

use crate::schema::CompanyProfile;

/// An adapter that transforms arbitrary input into a [`CompanyProfile`].
pub struct Adapter {
    plugin: Plugin,
    /// Name of the exported transform function in the WASM module.
    function_name: String,
}

impl Adapter {
    /// Load an adapter from a local WASM file.
    pub fn from_file(path: &str, function_name: &str) -> Result<Self> {
        let wasm = Wasm::file(path);
        let manifest = Manifest::new([wasm]);
        let plugin = Plugin::new(&manifest, [], true)
            .map_err(|e| anyhow::anyhow!("Failed to load WASM plugin: {e}"))?;
        info!("Loaded WASM adapter from {path}, function: {function_name}");
        Ok(Self {
            plugin,
            function_name: function_name.to_string(),
        })
    }

    /// Load an adapter from raw WASM bytes.
    pub fn from_bytes(bytes: &[u8], function_name: &str) -> Result<Self> {
        let wasm = Wasm::data(bytes.to_vec());
        let manifest = Manifest::new([wasm]);
        let plugin = Plugin::new(&manifest, [], true)
            .map_err(|e| anyhow::anyhow!("Failed to load WASM plugin: {e}"))?;
        Ok(Self {
            plugin,
            function_name: function_name.to_string(),
        })
    }

    /// Run the adapter: takes raw input, returns a validated [`CompanyProfile`].
    ///
    /// The WASM plugin must accept a string and return a JSON string that
    /// deserializes into [`CompanyProfile`].
    pub fn transform(&mut self, input: &str) -> Result<CompanyProfile> {
        let output: String = self
            .plugin
            .call::<&str, String>(&self.function_name, input)
            .map_err(|e| anyhow::anyhow!("Adapter call failed: {e}"))?;

        let profile: CompanyProfile = serde_json::from_str(&output)
            .map_err(|e| anyhow::anyhow!("Adapter output is not valid CompanyProfile JSON: {e}"))?;

        Ok(profile)
    }
}

/// The built-in (non-WASM) adapter for direct JSON input.
/// This is the default path when the input is already a well-formed CompanyProfile.
pub fn identity_adapter(json_input: &str) -> Result<CompanyProfile> {
    let profile: CompanyProfile = serde_json::from_str(json_input)?;
    Ok(profile)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_adapter_parses_valid_json() {
        let input = r#"{
            "name": "Test S.r.l.",
            "sector": "energy",
            "employees": 200,
            "annual_revenue_eur_m": 50.0,
            "balance_sheet_eur_m": 40.0,
            "services": ["electricity_generation"],
            "member_states": ["IT", "DE"]
        }"#;

        let profile = identity_adapter(input).unwrap();
        assert_eq!(profile.name, "Test S.r.l.");
        assert_eq!(profile.sector, "energy");
        assert_eq!(profile.employees, 200);
        assert_eq!(profile.member_states.len(), 2);
    }

    #[test]
    fn identity_adapter_rejects_invalid_json() {
        let input = r#"{"name": "Test"}"#; // missing required fields
        assert!(identity_adapter(input).is_err());
    }
}
