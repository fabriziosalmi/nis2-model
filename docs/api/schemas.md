# JSON Schema

All input/output types derive `schemars::JsonSchema` for automatic schema generation. Runtime validation uses `jsonschema` crate.

## CompanyProfile (input to `engine::evaluate`)

| Field | Type | Required |
|-------|------|----------|
| `name` | String | Yes |
| `sector` | String | Yes |
| `sub_sector` | String or null | No |
| `employees` | u32 | Yes |
| `annual_revenue_eur_m` | f64 | Yes |
| `balance_sheet_eur_m` | f64 | Yes |
| `services` | Vec\<String\> | Yes |
| `member_states` | Vec\<String\> | Yes |

> **Note:** The API normalizes `sector` to lowercase and trims whitespace. The rule engine performs case-sensitive matching against known sector codes.

## ComplianceStatus (output of `engine::evaluate`)

| Field | Type | Description |
|-------|------|-------------|
| `applicable` | bool | Whether NIS2 applies |
| `category` | enum | Essential, Important, or OutOfScope |
| `obligations` | Vec\<Obligation\> | 16 items when in scope, 0 when out |
| `max_sanction_eur` | Option\<f64\> | None when out of scope |
| `incident_reporting` | Option\<IncidentReporting\> | None when out of scope |

## ApplicabilityResponse (API /api/v1/applicability)

| Field | Type | Description |
|-------|------|-------------|
| `applicable` | bool | Whether NIS2 applies |
| `category` | String | Essential, Important, or OutOfScope |
| `sector` | String | Normalized sector code |
| `employees` | u32 | Input employee count |
| `disclaimer` | String | Legal disclaimer text |
| `sector_warning` | String or null | Warning when sector is unrecognized |

## Obligation

| Field | Type |
|-------|------|
| `id` | String (e.g. `nis2_art21_2_a`) |
| `article_ref` | String (e.g. `Art. 21(2)(a)`) |
| `description` | String (Italian) |
| `legal_text` | String (Italian) |
| `status` | enum: Pending, Compliant, NonCompliant, PartiallyCompliant |

## IncidentReporting

| Field | Type | Value |
|-------|------|-------|
| `early_warning_hours` | u32 | 24 |
| `notification_hours` | u32 | 72 |
| `final_report_days` | u32 | 30 |

## Runtime validation

```rust
nis2_rules::validation::validate_company_profile(&json_value)?;
nis2_rules::validation::validate_compliance_status(&json_value)?;
```

Schemas are generated from Rust types at compile time via `schemars`, not written by hand.
