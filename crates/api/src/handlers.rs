//! HTTP request handlers for the compliance API.

use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

use nis2_rules::engine;
use nis2_rules::schema::{CompanyProfile, ComplianceStatus};
use nis2_slm::inference::{InferenceBackend, TemplateBackend};
use nis2_slm::prompt::{self, InferenceParams};

/// Health check response.
#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub tests_passing: u32,
    pub crates: u32,
}

/// GET /api/v1/health
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        tests_passing: 61,
        crates: 6,
    })
}

/// POST /api/v1/evaluate
/// Full compliance evaluation -- returns ComplianceStatus JSON.
pub async fn evaluate(
    Json(profile): Json<CompanyProfile>,
) -> Json<ComplianceStatus> {
    let status = engine::evaluate(&profile);
    Json(status)
}

/// Minimal applicability check input.
#[derive(Deserialize)]
pub struct ApplicabilityRequest {
    pub sector: String,
    pub employees: u32,
    pub annual_revenue_eur_m: f64,
}

/// Applicability response.
#[derive(Serialize, Deserialize)]
pub struct ApplicabilityResponse {
    pub applicable: bool,
    pub category: String,
    pub sector: String,
    pub employees: u32,
}

/// POST /api/v1/applicability
pub async fn applicability(
    Json(req): Json<ApplicabilityRequest>,
) -> Json<ApplicabilityResponse> {
    let profile = CompanyProfile {
        name: "N/A".into(),
        sector: req.sector.clone(),
        sub_sector: None,
        employees: req.employees,
        annual_revenue_eur_m: req.annual_revenue_eur_m,
        balance_sheet_eur_m: req.annual_revenue_eur_m,
        services: Vec::new(),
        member_states: vec!["IT".into()],
    };
    let status = engine::evaluate(&profile);
    Json(ApplicabilityResponse {
        applicable: status.applicable,
        category: format!("{:?}", status.category),
        sector: req.sector,
        employees: req.employees,
    })
}

/// Sanction response.
#[derive(Serialize, Deserialize)]
pub struct SanctionResponse {
    pub category: String,
    pub max_sanction_eur: Option<f64>,
    pub legal_basis: String,
}

/// POST /api/v1/sanctions
pub async fn sanctions(
    Json(req): Json<ApplicabilityRequest>,
) -> Json<SanctionResponse> {
    let profile = CompanyProfile {
        name: "N/A".into(),
        sector: req.sector,
        sub_sector: None,
        employees: req.employees,
        annual_revenue_eur_m: req.annual_revenue_eur_m,
        balance_sheet_eur_m: req.annual_revenue_eur_m,
        services: Vec::new(),
        member_states: vec!["IT".into()],
    };
    let status = engine::evaluate(&profile);
    Json(SanctionResponse {
        category: format!("{:?}", status.category),
        max_sanction_eur: status.max_sanction_eur,
        legal_basis: "Art. 34 Direttiva (UE) 2022/2555".into(),
    })
}

/// Obligation summary item.
#[derive(Serialize, Deserialize)]
pub struct ObligationItem {
    pub id: String,
    pub article_ref: String,
    pub description: String,
}

/// POST /api/v1/obligations
pub async fn obligations(
    Json(req): Json<ApplicabilityRequest>,
) -> Json<Vec<ObligationItem>> {
    let profile = CompanyProfile {
        name: "N/A".into(),
        sector: req.sector,
        sub_sector: None,
        employees: req.employees,
        annual_revenue_eur_m: req.annual_revenue_eur_m,
        balance_sheet_eur_m: req.annual_revenue_eur_m,
        services: Vec::new(),
        member_states: vec!["IT".into()],
    };
    let status = engine::evaluate(&profile);
    let items: Vec<ObligationItem> = status.obligations.iter().map(|o| ObligationItem {
        id: o.id.clone(),
        article_ref: o.article_ref.clone(),
        description: o.description.clone(),
    }).collect();
    Json(items)
}

/// Report request.
#[derive(Deserialize)]
pub struct ReportRequest {
    pub name: String,
    pub sector: String,
    #[serde(default)]
    pub sub_sector: Option<String>,
    pub employees: u32,
    pub annual_revenue_eur_m: f64,
    pub balance_sheet_eur_m: f64,
    #[serde(default)]
    pub services: Vec<String>,
    #[serde(default = "default_member_states")]
    pub member_states: Vec<String>,
}

fn default_member_states() -> Vec<String> {
    vec!["IT".into()]
}

/// Report response.
#[derive(Serialize, Deserialize)]
pub struct ReportResponse {
    pub report: String,
    pub format: String,
}

/// POST /api/v1/report
pub async fn report(
    Json(req): Json<ReportRequest>,
) -> Result<Json<ReportResponse>, (StatusCode, String)> {
    let profile = CompanyProfile {
        name: req.name.clone(),
        sector: req.sector,
        sub_sector: req.sub_sector,
        employees: req.employees,
        annual_revenue_eur_m: req.annual_revenue_eur_m,
        balance_sheet_eur_m: req.balance_sheet_eur_m,
        services: req.services,
        member_states: req.member_states,
    };
    let status = engine::evaluate(&profile);
    let full_prompt = prompt::build_prompt(&req.name, &status);
    let backend = TemplateBackend;
    match backend.generate(&full_prompt, &InferenceParams::default()) {
        Ok(text) => Ok(Json(ReportResponse {
            report: text,
            format: "markdown".into(),
        })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;
    use crate::routes::build_router;

    #[tokio::test]
    async fn health_check() {
        let app = build_router();
        let resp = app
            .oneshot(Request::get("/api/v1/health").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(resp.status(), 200);
    }

    #[tokio::test]
    async fn evaluate_energy_company() {
        let app = build_router();
        let body = serde_json::json!({
            "name": "Test Energia S.r.l.",
            "sector": "energy",
            "employees": 200,
            "annual_revenue_eur_m": 50.0,
            "balance_sheet_eur_m": 40.0,
            "services": [],
            "member_states": ["IT"]
        });
        let resp = app
            .oneshot(
                Request::post("/api/v1/evaluate")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), 200);
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let result: ComplianceStatus = serde_json::from_slice(&bytes).unwrap();
        assert!(result.applicable);
    }

    #[tokio::test]
    async fn applicability_check() {
        let app = build_router();
        let body = serde_json::json!({
            "sector": "food",
            "employees": 100,
            "annual_revenue_eur_m": 20.0
        });
        let resp = app
            .oneshot(
                Request::post("/api/v1/applicability")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), 200);
    }

    #[tokio::test]
    async fn sanctions_check() {
        let app = build_router();
        let body = serde_json::json!({
            "sector": "energy",
            "employees": 500,
            "annual_revenue_eur_m": 1000.0
        });
        let resp = app
            .oneshot(
                Request::post("/api/v1/sanctions")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), 200);
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let result: SanctionResponse = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(result.max_sanction_eur, Some(20_000_000.0));
    }

    #[tokio::test]
    async fn obligations_returns_16() {
        let app = build_router();
        let body = serde_json::json!({
            "sector": "energy",
            "employees": 200,
            "annual_revenue_eur_m": 50.0
        });
        let resp = app
            .oneshot(
                Request::post("/api/v1/obligations")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), 200);
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let items: Vec<ObligationItem> = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(items.len(), 16);
    }

    #[tokio::test]
    async fn report_generation() {
        let app = build_router();
        let body = serde_json::json!({
            "name": "Acme Energia S.p.A.",
            "sector": "energy",
            "employees": 250,
            "annual_revenue_eur_m": 180.0,
            "balance_sheet_eur_m": 150.0,
            "services": ["electricity"],
            "member_states": ["IT"]
        });
        let resp = app
            .oneshot(
                Request::post("/api/v1/report")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), 200);
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let result: ReportResponse = serde_json::from_slice(&bytes).unwrap();
        assert!(result.report.contains("Ambito di applicazione"));
        assert!(result.report.contains("Art. 21(2)(a)"));
    }
}
