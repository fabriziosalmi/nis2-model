# Incident Reporting (Art. 23)

The engine sets incident reporting deadlines as defined in Art. 23 of Directive (EU) 2022/2555.

## Deadlines

| Phase | Deadline | Article | Field in IncidentReporting |
|-------|----------|---------|---------------------------|
| Early warning | 24 hours | Art. 23(4)(a) | `early_warning_hours: 24` |
| Full notification | 72 hours | Art. 23(4)(b) | `notification_hours: 72` |
| Final report | 30 days | Art. 23(4)(d) | `final_report_days: 30` |

## Implementation

In `crates/rules/src/engine.rs`, when an entity is in scope, the `IncidentReporting` struct is set:

```rust
IncidentReporting {
    early_warning_hours: 24,
    notification_hours: 72,
    final_report_days: 30,
}
```

For out-of-scope entities, `incident_reporting` is `None`.

## Test

The `incident_reporting_deadlines_correct` test in `engine.rs` verifies these exact values.
