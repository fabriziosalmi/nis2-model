#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use crate::engine::evaluate;
use crate::schema::CompanyProfile;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn evaluate_profile_json(profile_json: &str) -> String {
    let profile: CompanyProfile = match serde_json::from_str(profile_json) {
        Ok(p) => p,
        Err(e) => return format!(r#"{{"error": "{}"}}"#, e),
    };

    let status = evaluate(&profile);
    serde_json::to_string(&status).unwrap_or_else(|e| format!(r#"{{"error": "{}"}}"#, e))
}
