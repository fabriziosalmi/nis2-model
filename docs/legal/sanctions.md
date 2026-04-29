# Sanzioni Art. 34 NIS2

L'Art. 34 della Direttiva (UE) 2022/2555 stabilisce il regime sanzionatorio per le violazioni degli obblighi di cybersecurity.

## Formula di calcolo

### Soggetti essenziali (Art. 34(4))

$$
\text{Sanzione massima} = \max(€10.000.000,\ 2\%\ \text{fatturato mondiale annuo})
$$

### Soggetti importanti (Art. 34(5))

$$
\text{Sanzione massima} = \max(€7.000.000,\ 1,4\%\ \text{fatturato mondiale annuo})
$$

## Esempi

| Tipo | Fatturato | 2% / 1.4% | Floor | Sanzione max |
|------|-----------|-----------|-------|-------------|
| Essential | €50M | €1M | €10M | **€10.000.000** |
| Essential | €100M | €2M | €10M | **€10.000.000** |
| Essential | €1.000M | €20M | €10M | **€20.000.000** |
| Important | €50M | €700K | €7M | **€7.000.000** |
| Important | €1.000M | €14M | €7M | **€14.000.000** |

## Violazioni coperte

Le sanzioni si applicano in caso di violazione di:

- **Art. 21** — Misure di gestione dei rischi di cibersicurezza
- **Art. 23** — Obblighi di segnalazione

## Implementazione nel rule engine

```rust
fn calculate_max_sanction(category: &EntityCategory, revenue: f64) -> f64 {
    match category {
        Essential => f64::max(10_000_000.0, revenue * 1_000_000.0 * 0.02),
        Important => f64::max(7_000_000.0, revenue * 1_000_000.0 * 0.014),
        OutOfScope => 0.0,
    }
}
```

::: danger Nota bene
Questo motore calcola il **massimale teorico**. L'importo effettivo della sanzione è determinato dall'autorità competente sulla base delle circostanze concrete (Art. 34(3)).
:::
