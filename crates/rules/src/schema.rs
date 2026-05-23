//! Schema definitions for rule engine inputs and outputs.
//!
//! All types derive [`schemars::JsonSchema`] for automatic JSON Schema generation
//! and [`jsonschema`]-based runtime validation.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Company profile — input to the compliance rule engine.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, utoipa::ToSchema)]
pub struct CompanyProfile {
    /// Company name.
    pub name: String,
    /// Sector code from NIS2 Annex I / Annex II.
    pub sector: String,
    /// Sub-sector classification.
    pub sub_sector: Option<String>,
    /// Total number of employees.
    pub employees: u32,
    /// Annual revenue in EUR (millions).
    pub annual_revenue_eur_m: f64,
    /// Annual balance sheet total in EUR (millions).
    pub balance_sheet_eur_m: f64,
    /// List of services provided that may fall under NIS2.
    pub services: Vec<String>,
    /// EU member states where the company operates.
    pub member_states: Vec<String>,
}

/// Entity classification under NIS2.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, utoipa::ToSchema)]
pub enum EntityCategory {
    /// Soggetto essenziale (Art. 3(1))
    Essential,
    /// Soggetto importante (Art. 3(2))
    Important,
    /// Non rientra nell'ambito di applicazione
    OutOfScope,
}

/// A single compliance obligation derived from Art. 21(2) NIS2.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, utoipa::ToSchema)]
pub struct Obligation {
    /// Unique identifier for this obligation (e.g. "nis2_art21_2_a").
    pub id: String,
    /// NIS2 article reference (e.g. "Art. 21(2)(a)").
    pub article_ref: String,
    /// Human-readable description of the requirement.
    pub description: String,
    /// Italian legal text from the directive.
    pub legal_text: String,
    /// Whether this obligation is currently met.
    pub status: ObligationStatus,
}

/// Status of a compliance obligation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, utoipa::ToSchema)]
pub enum ObligationStatus {
    /// Not yet evaluated.
    Pending,
    /// Requirement satisfied.
    Compliant,
    /// Requirement not satisfied.
    NonCompliant,
    /// Partially satisfied — remediation needed.
    PartiallyCompliant,
}

/// The complete output of the compliance rule engine.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, utoipa::ToSchema)]
pub struct ComplianceStatus {
    /// Whether NIS2 applies to this entity at all.
    pub applicable: bool,
    /// Entity classification.
    pub category: EntityCategory,
    /// List of applicable obligations with their status.
    pub obligations: Vec<Obligation>,
    /// Maximum potential sanction in EUR.
    pub max_sanction_eur: Option<f64>,
    /// Incident reporting deadlines (if applicable).
    pub incident_reporting: Option<IncidentReporting>,
    /// Transposition-specific notes and warnings.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transposition_notes: Vec<String>,
}

/// Art. 23 NIS2 — Incident reporting deadlines.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, utoipa::ToSchema)]
pub struct IncidentReporting {
    /// Hours for initial early warning (Art. 23(4)(a)).
    pub early_warning_hours: u32,
    /// Hours for full incident notification (Art. 23(4)(b)).
    pub notification_hours: u32,
    /// Days for final report (Art. 23(4)(d)).
    pub final_report_days: u32,
}
