# REST API

**Crate**: `nis2-api` | **Files**: `handlers.rs`, `routes.rs` | **Tests**: 6

The Axum HTTP server exposes 6 endpoints under `/api/v1`. Start with:

```bash
cargo run --bin api-server                 # default port 8080
cargo run --bin api-server -- --port 3000  # custom port
```

## Endpoints

### GET /api/v1/health

Returns server status, version, test count, and crate count.

### POST /api/v1/evaluate

Accepts a full `CompanyProfile` JSON body. Returns the complete `ComplianceStatus` JSON.

### POST /api/v1/applicability

Accepts `{ "sector", "employees", "annual_revenue_eur_m" }`. The `sector` field is normalized to lowercase.

Returns `{ "applicable", "category", "sector", "employees", "disclaimer", "sector_warning" }`. The `sector_warning` field is present only when the sector code is not recognized (returns `null` otherwise).

### POST /api/v1/sanctions

Same input as applicability. Returns `{ "category", "max_sanction_eur", "legal_basis" }`.

### POST /api/v1/obligations

Same input as applicability. Returns an array of `{ "id", "article_ref", "description" }` objects (16 items for in-scope entities).

### POST /api/v1/report

Accepts a full `ReportRequest` with `name`, `sector`, `employees`, `annual_revenue_eur_m`, `balance_sheet_eur_m`, and optional `sub_sector`, `services`, `member_states`. Returns `{ "report", "format" }` where `report` is the Italian markdown text and `format` is `"markdown"`.

## Tests (6)

`health_check`, `evaluate_energy_company`, `applicability_check`, `sanctions_check`, `obligations_returns_16`, `report_generation`

## Security

- **CORS**: Restricted by default (same-origin). Set `CORS_ORIGIN` env var to allow a specific external origin.
- **Headers**: API responses include `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`, `Referrer-Policy: strict-origin-when-cross-origin`.
- **Input**: Sector codes are normalized (lowercase, trimmed). Unrecognized sectors return `OutOfScope` with a warning.
- **Auth**: Not included — deployers should add authentication (API key or JWT) before production exposure.

## Legal

All responses include a `disclaimer` field. API outputs are automated classifications and do not constitute legal advice.
