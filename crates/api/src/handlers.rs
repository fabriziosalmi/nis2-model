//! HTTP request handlers for the compliance API.

use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

use nis2_rules::engine::{self, ANNEX_I_SECTORS, ANNEX_II_SECTORS};
use nis2_rules::schema::{CompanyProfile, ComplianceStatus};
use nis2_slm::inference::{InferenceBackend, TemplateBackend};
use nis2_slm::prompt::{self, InferenceParams};

/// Legal disclaimer appended to all API responses.
const DISCLAIMER: &str = "Classificazione automatizzata — non costituisce consulenza legale. \
    Consultare un avvocato qualificato per una valutazione vincolante.";

/// Normalize sector input: lowercase, trimmed.
fn normalize_sector(s: &str) -> String {
    s.trim().to_lowercase().replace(' ', "_")
}

/// Check if a sector code is recognized.
fn is_known_sector(sector: &str) -> bool {
    ANNEX_I_SECTORS.contains(&sector) || ANNEX_II_SECTORS.contains(&sector)
}

/// Build a helpful error message for unrecognized sectors.
fn unknown_sector_hint(sector: &str) -> String {
    let all: Vec<&str> = ANNEX_I_SECTORS.iter().chain(ANNEX_II_SECTORS.iter()).copied().collect();
    format!(
        "Settore '{}' non riconosciuto. Il motore classifica il soggetto come OutOfScope \
         per settori sconosciuti. Settori validi: {}. \
         Verificare con un consulente se il settore è corretto.",
        sector,
        all.join(", ")
    )
}

static START_TIME: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();

fn get_uptime() -> u64 {
    START_TIME.get_or_init(std::time::Instant::now).elapsed().as_secs()
}

/// Health check response.
#[derive(Serialize, Deserialize, Debug, utoipa::ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub workspace_crates: u32,
}

/// GET /api/v1/health
#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Health status of the API", body = HealthResponse)
    )
)]
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        uptime_seconds: get_uptime(),
        workspace_crates: 7,
    })
}

/// POST /api/v1/evaluate
/// Full compliance evaluation -- returns ComplianceStatus JSON.
#[utoipa::path(
    post,
    path = "/api/v1/evaluate",
    request_body = CompanyProfile,
    responses(
        (status = 200, description = "Complete compliance status of the entity", body = ComplianceStatus)
    )
)]
pub async fn evaluate(
    Json(profile): Json<CompanyProfile>,
) -> Json<ComplianceStatus> {
    let status = engine::evaluate(&profile);
    Json(status)
}

/// Minimal applicability check input.
#[derive(Deserialize, utoipa::ToSchema)]
pub struct ApplicabilityRequest {
    pub sector: String,
    pub employees: u32,
    pub annual_revenue_eur_m: f64,
}

/// Applicability response.
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct ApplicabilityResponse {
    pub applicable: bool,
    pub category: String,
    pub sector: String,
    pub employees: u32,
    pub disclaimer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sector_warning: Option<String>,
}

/// POST /api/v1/applicability
#[utoipa::path(
    post,
    path = "/api/v1/applicability",
    request_body = ApplicabilityRequest,
    responses(
        (status = 200, description = "Minimal applicability check result", body = ApplicabilityResponse)
    )
)]
pub async fn applicability(
    Json(req): Json<ApplicabilityRequest>,
) -> Json<ApplicabilityResponse> {
    let sector = normalize_sector(&req.sector);
    let warning = if !is_known_sector(&sector) { Some(unknown_sector_hint(&sector)) } else { None };
    let profile = CompanyProfile {
        name: "N/A".into(),
        sector: sector.clone(),
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
        sector,
        employees: req.employees,
        disclaimer: DISCLAIMER.into(),
        sector_warning: warning,
    })
}

/// Sanction response.
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct SanctionResponse {
    pub category: String,
    pub max_sanction_eur: Option<f64>,
    pub legal_basis: String,
}

/// POST /api/v1/sanctions
#[utoipa::path(
    post,
    path = "/api/v1/sanctions",
    request_body = ApplicabilityRequest,
    responses(
        (status = 200, description = "Maximum potential sanction details", body = SanctionResponse)
    )
)]
pub async fn sanctions(
    Json(req): Json<ApplicabilityRequest>,
) -> Json<SanctionResponse> {
    let sector = normalize_sector(&req.sector);
    let profile = CompanyProfile {
        name: "N/A".into(),
        sector,
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
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct ObligationItem {
    pub id: String,
    pub article_ref: String,
    pub description: String,
}

/// POST /api/v1/obligations
#[utoipa::path(
    post,
    path = "/api/v1/obligations",
    request_body = ApplicabilityRequest,
    responses(
        (status = 200, description = "List of applicable security obligations", body = [ObligationItem])
    )
)]
pub async fn obligations(
    Json(req): Json<ApplicabilityRequest>,
) -> Json<Vec<ObligationItem>> {
    let sector = normalize_sector(&req.sector);
    let profile = CompanyProfile {
        name: "N/A".into(),
        sector,
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
#[derive(Deserialize, utoipa::ToSchema)]
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
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct ReportResponse {
    pub report: String,
    pub format: String,
}

/// POST /api/v1/report
#[utoipa::path(
    post,
    path = "/api/v1/report",
    request_body = ReportRequest,
    responses(
        (status = 200, description = "Markdown report text", body = ReportResponse)
    )
)]
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

/// POST /api/v1/report/pdf
/// Generate a server-side Typst PDF report.
#[utoipa::path(
    post,
    path = "/api/v1/report/pdf",
    request_body = ReportRequest,
    responses(
        (status = 200, description = "PDF report download", body = Vec<u8>, content_type = "application/pdf")
    )
)]
pub async fn report_pdf(
    Json(req): Json<ReportRequest>,
) -> Result<impl axum::response::IntoResponse, (StatusCode, String)> {
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
    let typst_markup = nis2_slm::report_typst::generate_typst_report(&req.name, &status);
    
    // Generate a unique filename using timestamp and thread ID
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let thread_id = std::thread::current().id();
    let file_prefix = format!("report_{}_{:?}", now, thread_id);
    let temp_dir = std::env::temp_dir();
    let typst_path = temp_dir.join(format!("{}.typ", file_prefix));
    let pdf_path = temp_dir.join(format!("{}.pdf", file_prefix));

    // Write typst content to temp file
    if let Err(e) = std::fs::write(&typst_path, typst_markup) {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to write typst file: {}", e)));
    }

    // Run typst compile
    let output = std::process::Command::new("typst")
        .arg("compile")
        .arg(&typst_path)
        .arg(&pdf_path)
        .output();

    // Clean up .typ file immediately
    let _ = std::fs::remove_file(&typst_path);

    match output {
        Ok(out) if out.status.success() => {
            // Read pdf bytes
            match std::fs::read(&pdf_path) {
                Ok(bytes) => {
                    // Clean up pdf file
                    let _ = std::fs::remove_file(&pdf_path);
                    
                    let filename = req.name.replace(' ', "_");
                    let response = axum::response::Response::builder()
                        .header(axum::http::header::CONTENT_TYPE, "application/pdf")
                        .header(
                            axum::http::header::CONTENT_DISPOSITION,
                            format!("attachment; filename=\"nis2_report_{}.pdf\"", filename),
                        )
                        .body(axum::body::Body::from(bytes))
                        .unwrap();
                    Ok(response)
                }
                Err(e) => {
                    let _ = std::fs::remove_file(&pdf_path);
                    Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read generated PDF: {}", e)))
                }
            }
        }
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Typst compilation failed: {}", stderr)))
        }
        Err(e) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to execute typst CLI: {}. Ensure typst is installed.", e)))
        }
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
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let health: HealthResponse = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(health.status, "ok");
        assert_eq!(health.workspace_crates, 7);
        assert!(health.uptime_seconds < 10);
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

    #[tokio::test]
    async fn report_pdf_generation() {
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
                Request::post("/api/v1/report/pdf")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        
        let status = resp.status();
        let headers = resp.headers().clone();
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let err_msg = String::from_utf8_lossy(&bytes).to_string();
        
        if status == 200 {
            assert_eq!(headers.get("content-type").unwrap(), "application/pdf");
            assert!(!bytes.is_empty());
        } else {
            panic!("PDF report generation failed: {}", err_msg);
        }
    }
}
