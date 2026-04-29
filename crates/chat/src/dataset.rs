//! Pre-computed Q&A dataset for NIS2 compliance.

use crate::cache::CacheEntry;
use nis2_rules::engine;
use nis2_rules::schema::CompanyProfile;

/// Build the pre-computed Q&A dataset. Embeddings are empty and must be
/// filled at warmup time by the chat engine.
pub fn build_dataset() -> Vec<CacheEntry> {
    let mut entries = Vec::new();

    // --- Applicability ---
    for sector in &["energy", "transport", "banking", "health", "food", "chemicals"] {
        let profile = CompanyProfile {
            name: "N/A".into(), sector: sector.to_string(), sub_sector: None,
            employees: 200, annual_revenue_eur_m: 50.0, balance_sheet_eur_m: 50.0,
            services: vec![], member_states: vec!["IT".into()],
        };
        let status = engine::evaluate(&profile);
        let cat = format!("{:?}", status.category);
        entries.push(CacheEntry {
            question: format!("Does NIS2 apply to {} companies?", sector),
            answer: format!("Yes. The {} sector is in scope. Entities meeting the size threshold are classified as {}.", sector, cat),
            category: "applicability".into(), embedding: vec![],
        });
    }

    entries.push(CacheEntry {
        question: "Is a small retail shop subject to NIS2?".into(),
        answer: "No. Retail is not listed in Annex I or II of NIS2. The entity is OutOfScope.".into(),
        category: "applicability".into(), embedding: vec![],
    });

    entries.push(CacheEntry {
        question: "Does NIS2 apply to my company?".into(),
        answer: "NIS2 applies to entities in Annex I (11 sectors) or Annex II (7 sectors) meeting the size threshold of 50+ employees or 10M+ EUR revenue.".into(),
        category: "applicability".into(), embedding: vec![],
    });

    // --- Sanctions ---
    entries.push(CacheEntry {
        question: "What are the NIS2 penalties?".into(),
        answer: "Art. 34: Essential entities face max(10M EUR, 2% turnover). Important entities face max(7M EUR, 1.4% turnover).".into(),
        category: "sanctions".into(), embedding: vec![],
    });
    entries.push(CacheEntry {
        question: "How much is the NIS2 fine?".into(),
        answer: "Essential: up to 10M EUR or 2% of global turnover. Important: up to 7M EUR or 1.4%. Defined in Art. 34 of Directive (EU) 2022/2555.".into(),
        category: "sanctions".into(), embedding: vec![],
    });

    // --- Obligations ---
    entries.push(CacheEntry {
        question: "What are the NIS2 obligations?".into(),
        answer: "16 obligations: Art. 20 (2 governance), Art. 21(2)(a-j) (10 cybersecurity measures), Art. 23 (4 incident reporting).".into(),
        category: "obligations".into(), embedding: vec![],
    });
    entries.push(CacheEntry {
        question: "What cybersecurity measures does NIS2 require?".into(),
        answer: "Art. 21(2) mandates 10 measures: (a) risk analysis, (b) incident handling, (c) business continuity, (d) supply chain, (e) system security, (f) effectiveness assessment, (g) cyber hygiene, (h) cryptography, (i) access control, (j) MFA.".into(),
        category: "obligations".into(), embedding: vec![],
    });

    // --- Incidents ---
    entries.push(CacheEntry {
        question: "What are the NIS2 incident reporting deadlines?".into(),
        answer: "Art. 23(4): (a) early warning within 24 hours, (b) full notification within 72 hours, (d) final report within 30 days.".into(),
        category: "incidents".into(), embedding: vec![],
    });
    entries.push(CacheEntry {
        question: "How quickly must I report a cybersecurity incident?".into(),
        answer: "24h early warning, 72h full notification, 30d final report. Per Art. 23 of NIS2.".into(),
        category: "incidents".into(), embedding: vec![],
    });

    // --- Classification ---
    entries.push(CacheEntry {
        question: "What is the difference between essential and important entities?".into(),
        answer: "Essential (Art. 3(1)): Annex I sectors, higher sanctions (10M/2%). Important (Art. 3(2)): Annex II sectors, lower sanctions (7M/1.4%).".into(),
        category: "classification".into(), embedding: vec![],
    });

    // --- DORA ---
    entries.push(CacheEntry {
        question: "What is DORA?".into(),
        answer: "DORA (Regulation (EU) 2022/2554) covers ICT risk in finance. This model indexes 14 DORA provisions for search but does not evaluate DORA compliance.".into(),
        category: "dora".into(), embedding: vec![],
    });

    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dataset_has_entries() {
        assert!(build_dataset().len() >= 15);
    }

    #[test]
    fn all_entries_have_content() {
        for e in build_dataset() {
            assert!(!e.question.is_empty());
            assert!(!e.answer.is_empty());
            assert!(!e.category.is_empty());
        }
    }

    #[test]
    fn no_uncertain_language() {
        let forbidden = ["maybe", "perhaps", "might", "could be", "potrebbe", "forse"];
        for e in build_dataset() {
            let lower = e.answer.to_lowercase();
            for w in &forbidden { assert!(!lower.contains(w), "{} in: {}", w, e.question); }
        }
    }
}
