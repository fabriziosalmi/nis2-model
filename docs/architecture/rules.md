# Rule Engine

**Crate**: `nis2-rules` | **Files**: `engine.rs`, `obligations.rs`, `schema.rs`, `validation.rs`, `adapters.rs` | **Tests**: 32

## What it does

Deterministic boolean decision trees that evaluate a `CompanyProfile` and produce a `ComplianceStatus`.

## `engine::evaluate()`

The core function. It:

1. Checks if the sector is in `ANNEX_I_SECTORS` (11 sectors) or `ANNEX_II_SECTORS` (7 sectors)
2. Checks if the entity meets the size threshold (50+ employees OR 10M+ EUR revenue)
3. Special case: `digital_infrastructure`, `ict_service_management_b2b`, `public_administration` are always in scope
4. If in scope: attaches all 16 obligations (Art. 20: 2 + Art. 21: 10 + Art. 23: 4)
5. Calculates `max_sanction_eur` using Art. 34 formula
6. Sets `incident_reporting` deadlines (24h, 72h, 30d)

## Sectors

**Annex I** (11): `energy`, `transport`, `banking`, `financial_market_infrastructure`, `health`, `drinking_water`, `waste_water`, `digital_infrastructure`, `ict_service_management_b2b`, `public_administration`, `space`

**Annex II** (7): `postal_courier`, `waste_management`, `chemicals`, `food`, `manufacturing`, `digital_providers`, `research`

## Obligation catalog (`obligations.rs`)

- `art20_obligations()` -- 2 obligations (governance approval, mandatory training)
- `art21_obligations()` -- 10 obligations (Art. 21(2)(a) through (j))
- `art23_obligations()` -- 4 obligations (CSIRT notification, 24h, 72h, 30d)

All obligations are in Italian (as they are from the Italian transposition of the directive).

## JSON Schema validation (`validation.rs`)

All types derive `schemars::JsonSchema`. Runtime validation uses `jsonschema`:

- `validate_company_profile()` -- validates input
- `validate_compliance_status()` -- validates output

## WASM adapters (`adapters.rs`)

Provides an `IdentityAdapter` for Extism WASM plugin integration. Currently a passthrough that validates JSON structure.

## Tests (32)

Engine tests: `large_energy_company_is_essential`, `medium_food_company_is_important`, `small_unrelated_company_is_out_of_scope`, `digital_infra_always_in_scope`, `in_scope_entity_has_16_obligations`, `out_of_scope_has_no_obligations`, `all_obligations_are_pending_initially`, `sanction_essential_uses_floor`, `sanction_essential_uses_two_percent`, `incident_reporting_deadlines_correct`, `output_is_valid_json`

Edge case tests: `exactly_50_employees_meets_threshold`, `exactly_49_employees_below_threshold`, `exactly_10m_revenue_meets_threshold`, `revenue_9_99m_below_threshold`, `small_annex_ii_is_out_of_scope`, `annex_i_below_threshold_non_special_is_out_of_scope`, `zero_employees_with_high_revenue_meets_threshold`, `public_administration_always_in_scope`, `ict_service_management_always_in_scope`, `sector_is_case_sensitive`, `sanction_important_uses_1_4_percent`, `sanction_important_uses_floor`

Obligation tests: `art21_has_ten_obligations`, `all_obligation_ids_are_unique`, `all_obligations_start_pending`

Validation tests: `valid_company_profile_passes`, `missing_required_field_fails`, `wrong_type_fails`, `engine_output_validates_against_schema`

Adapter tests: `identity_adapter_parses_valid_json`, `identity_adapter_rejects_invalid_json`
