# Sanctions (Art. 34)

The engine calculates maximum administrative fines under Art. 34 of Directive (EU) 2022/2555.

## Formula

**Essential entities** (Art. 34(4)):

```
max(10,000,000 EUR, 2% of total worldwide annual turnover)
```

**Important entities** (Art. 34(5)):

```
max(7,000,000 EUR, 1.4% of total worldwide annual turnover)
```

**OutOfScope**: no sanction (returns `None`).

## Implementation

In `crates/rules/src/engine.rs`, the `calculate_sanction()` function takes `EntityCategory` and `annual_revenue_eur_m` (in millions) and returns:

- Essential: `f64::max(10_000_000.0, revenue * 1_000_000.0 * 0.02)`
- Important: `f64::max(7_000_000.0, revenue * 1_000_000.0 * 0.014)`

## Examples from tests

| Category | Revenue | 2% / 1.4% | Floor | Result |
|----------|---------|-----------|-------|--------|
| Essential | 50M EUR | 1M | 10M | 10,000,000 EUR |
| Essential | 1,000M EUR | 20M | 10M | 20,000,000 EUR |

These are verified by `sanction_essential_uses_floor` and `sanction_essential_uses_two_percent` tests.

Note: the engine calculates the **theoretical maximum**. The actual fine is determined by the competent authority based on circumstances (Art. 34(3)).
