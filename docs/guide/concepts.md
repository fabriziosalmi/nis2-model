# Key Concepts

## Determinism

Every function in the engine is deterministic: same input produces the same output. There are no stochastic elements.

```
engine::evaluate(&CompanyProfile) -> ComplianceStatus  // always identical
```

## Entity classification

The engine implements the decision tree from Art. 2 and 3 of NIS2:

- Sector in Annex I + meets size threshold = **Essential** (Art. 3(1))
- Sector in Annex II + meets size threshold = **Important** (Art. 3(2))
- Sectors `digital_infrastructure`, `ict_service_management_b2b`, `public_administration` = **Essential** regardless of size (Art. 2(2))
- Everything else = **OutOfScope**

The size threshold follows a simplified model from EU Recommendation 2003/361/EC: 50+ employees OR 10M+ EUR annual revenue.

> **Note:** This is a simplification. The full Recommendation 2003/361/EC also considers balance sheet totals and linked/partner enterprise relationships. See the [Limitations](/legal/terms) section for details.

## Obligation catalog

The engine maps 16 obligations from three articles, hardcoded in `obligations.rs`:

- **Art. 20**: 2 governance obligations (management approval, mandatory training)
- **Art. 21(2)(a-j)**: 10 cybersecurity measures (risk analysis, incident management, business continuity, supply chain, etc.)
- **Art. 23**: 4 incident reporting obligations (CSIRT notification, 24h early warning, 72h full notification, 30-day final report)

All obligations start with status `Pending`.

## Sanctions (Art. 34)

- Essential entities: max(10,000,000 EUR, 2% of worldwide annual turnover)
- Important entities: max(7,000,000 EUR, 1.4% of worldwide annual turnover)

## Semantic chunking

Legal texts are split by legal unit (article/paragraph/letter), not by word count. Each chunk has a `LegalReference` with directive, article number, paragraph number, and optional letter.

## Report generation

The `TemplateBackend` generates reports by substituting structured `ComplianceStatus` data into fixed Italian-language templates. No generative model is involved. The report has 4 sections: scope, obligations, sanctions, incident reporting.

The codebase includes a test (`report_never_contains_uncertain_language`) that verifies the output never contains hedging words like "potrebbe", "forse", "probabilmente", or "eventualmente". All report text uses the framing "il motore di analisi classifica" to distinguish automated classification from legal opinion.
