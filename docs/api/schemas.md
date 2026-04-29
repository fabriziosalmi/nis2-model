# JSON Schema

Il rule engine genera e valida JSON Schema a runtime per tutti i tipi di input e output.

## CompanyProfile (Input)

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": [
    "name", "sector", "employees",
    "annual_revenue_eur_m", "balance_sheet_eur_m",
    "services", "member_states"
  ],
  "properties": {
    "name": { "type": "string" },
    "sector": { "type": "string" },
    "sub_sector": { "type": ["string", "null"] },
    "employees": { "type": "integer", "minimum": 0 },
    "annual_revenue_eur_m": { "type": "number" },
    "balance_sheet_eur_m": { "type": "number" },
    "services": {
      "type": "array",
      "items": { "type": "string" }
    },
    "member_states": {
      "type": "array",
      "items": { "type": "string" }
    }
  }
}
```

## ComplianceStatus (Output)

```json
{
  "type": "object",
  "required": ["applicable", "category", "obligations"],
  "properties": {
    "applicable": { "type": "boolean" },
    "category": {
      "type": "string",
      "enum": ["Essential", "Important", "OutOfScope"]
    },
    "obligations": {
      "type": "array",
      "items": { "$ref": "#/definitions/Obligation" }
    },
    "max_sanction_eur": { "type": ["number", "null"] },
    "incident_reporting": {
      "type": ["object", "null"],
      "properties": {
        "early_warning_hours": { "type": "integer" },
        "notification_hours": { "type": "integer" },
        "final_report_days": { "type": "integer" }
      }
    }
  }
}
```

## Obligation

```json
{
  "type": "object",
  "required": ["id", "article_ref", "description", "legal_text", "status"],
  "properties": {
    "id": { "type": "string" },
    "article_ref": { "type": "string" },
    "description": { "type": "string" },
    "legal_text": { "type": "string" },
    "status": {
      "type": "string",
      "enum": ["Pending", "Compliant", "NonCompliant", "PartiallyCompliant"]
    }
  }
}
```

## Validazione runtime

```rust
use nis2_rules::validation;

// Validazione input
let json = serde_json::json!({ ... });
validation::validate_company_profile(&json)?;

// Validazione output
let status = engine::evaluate(&profile);
let output_json = serde_json::to_value(&status)?;
validation::validate_compliance_status(&output_json)?;
```

::: tip Generazione automatica
Gli schema sono generati automaticamente dal codice Rust via `schemars::JsonSchema`. Non sono scritti a mano.
:::
