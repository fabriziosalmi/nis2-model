//! Pre-computed multilingual Q&A dataset for NIS2 compliance.
//!
//! Generates 100+ entries in EN, IT, DE, FR, ES from the rule engine.

use crate::cache::CacheEntry;
use nis2_rules::engine;
use nis2_rules::schema::CompanyProfile;

struct Lang {
    code: &'static str,
    applies_yes: &'static str,
    applies_no: &'static str,
    sector_q: &'static str,
    essential_q: &'static str,
    important_q: &'static str,
    sanction_q: &'static str,
    obligation_q: &'static str,
    incident_q: &'static str,
    scope_q: &'static str,
    mfa_q: &'static str,
    supply_q: &'static str,
    crypto_q: &'static str,
    training_q: &'static str,
    governance_q: &'static str,
    dora_q: &'static str,
    threshold_q: &'static str,
    diff_q: &'static str,
}

const LANGS: &[Lang] = &[
    Lang {
        code: "en",
        applies_yes: "Yes. The {sector} sector is listed in {annex} of NIS2. Entities with 50+ employees or 10M+ EUR revenue are classified as {cat}.",
        applies_no: "No. The {sector} sector is not in Annex I or II of NIS2. The entity is OutOfScope.",
        sector_q: "Does NIS2 apply to {sector} companies?",
        essential_q: "What is an essential entity under NIS2?",
        important_q: "What is an important entity under NIS2?",
        sanction_q: "What are the NIS2 sanctions?",
        obligation_q: "What are the NIS2 obligations?",
        incident_q: "What are the NIS2 incident reporting deadlines?",
        scope_q: "Does NIS2 apply to my company?",
        mfa_q: "Does NIS2 require multi-factor authentication?",
        supply_q: "What does NIS2 say about supply chain security?",
        crypto_q: "What does NIS2 require for encryption?",
        training_q: "Does NIS2 require cybersecurity training?",
        governance_q: "What are the NIS2 governance requirements?",
        dora_q: "What is DORA?",
        threshold_q: "What is the NIS2 size threshold?",
        diff_q: "What is the difference between essential and important?",
    },
    Lang {
        code: "it",
        applies_yes: "Si. Il settore {sector} e' elencato nell'{annex} della NIS2. Soggetti con 50+ dipendenti o 10M+ EUR fatturato sono classificati come {cat}.",
        applies_no: "No. Il settore {sector} non e' negli Allegati I o II della NIS2. Il soggetto e' fuori ambito.",
        sector_q: "La NIS2 si applica alle aziende del settore {sector}?",
        essential_q: "Cos'e' un soggetto essenziale nella NIS2?",
        important_q: "Cos'e' un soggetto importante nella NIS2?",
        sanction_q: "Quali sono le sanzioni NIS2?",
        obligation_q: "Quali sono gli obblighi NIS2?",
        incident_q: "Quali sono le scadenze di segnalazione incidenti NIS2?",
        scope_q: "La NIS2 si applica alla mia azienda?",
        mfa_q: "La NIS2 richiede l'autenticazione multi-fattore?",
        supply_q: "Cosa dice la NIS2 sulla sicurezza della catena di approvvigionamento?",
        crypto_q: "Cosa richiede la NIS2 per la crittografia?",
        training_q: "La NIS2 richiede formazione in cybersecurity?",
        governance_q: "Quali sono i requisiti di governance della NIS2?",
        dora_q: "Cos'e' il DORA?",
        threshold_q: "Qual e' la soglia dimensionale della NIS2?",
        diff_q: "Qual e' la differenza tra soggetto essenziale e importante?",
    },
    Lang {
        code: "de",
        applies_yes: "Ja. Der Sektor {sector} ist in {annex} der NIS2 gelistet. Unternehmen mit 50+ Mitarbeitern oder 10M+ EUR Umsatz werden als {cat} eingestuft.",
        applies_no: "Nein. Der Sektor {sector} ist nicht in Anhang I oder II der NIS2. Das Unternehmen faellt nicht in den Anwendungsbereich.",
        sector_q: "Gilt die NIS2 fuer {sector}-Unternehmen?",
        essential_q: "Was ist ein wesentliches Unternehmen unter NIS2?",
        important_q: "Was ist ein wichtiges Unternehmen unter NIS2?",
        sanction_q: "Welche Sanktionen sieht die NIS2 vor?",
        obligation_q: "Welche Pflichten hat die NIS2?",
        incident_q: "Welche Meldefristen gibt es bei NIS2?",
        scope_q: "Gilt die NIS2 fuer mein Unternehmen?",
        mfa_q: "Verlangt die NIS2 Multi-Faktor-Authentifizierung?",
        supply_q: "Was sagt die NIS2 zur Lieferkettensicherheit?",
        crypto_q: "Was verlangt die NIS2 zur Verschluesselung?",
        training_q: "Verlangt die NIS2 Cybersicherheitsschulungen?",
        governance_q: "Welche Governance-Anforderungen hat die NIS2?",
        dora_q: "Was ist DORA?",
        threshold_q: "Was ist die NIS2-Groessenschwelle?",
        diff_q: "Was ist der Unterschied zwischen wesentlich und wichtig?",
    },
    Lang {
        code: "fr",
        applies_yes: "Oui. Le secteur {sector} figure dans l'{annex} de NIS2. Les entites de 50+ employes ou 10M+ EUR CA sont classees {cat}.",
        applies_no: "Non. Le secteur {sector} ne figure pas aux Annexes I ou II de NIS2. L'entite est hors champ.",
        sector_q: "La NIS2 s'applique-t-elle aux entreprises du secteur {sector}?",
        essential_q: "Qu'est-ce qu'une entite essentielle sous NIS2?",
        important_q: "Qu'est-ce qu'une entite importante sous NIS2?",
        sanction_q: "Quelles sont les sanctions NIS2?",
        obligation_q: "Quelles sont les obligations NIS2?",
        incident_q: "Quels sont les delais de signalement NIS2?",
        scope_q: "La NIS2 s'applique-t-elle a mon entreprise?",
        mfa_q: "La NIS2 exige-t-elle l'authentification multi-facteurs?",
        supply_q: "Que dit la NIS2 sur la securite de la chaine d'approvisionnement?",
        crypto_q: "Que demande la NIS2 en matiere de chiffrement?",
        training_q: "La NIS2 exige-t-elle une formation en cybersecurite?",
        governance_q: "Quelles sont les exigences de gouvernance NIS2?",
        dora_q: "Qu'est-ce que DORA?",
        threshold_q: "Quel est le seuil de taille NIS2?",
        diff_q: "Quelle est la difference entre essentiel et important?",
    },
    Lang {
        code: "es",
        applies_yes: "Si. El sector {sector} esta en el {annex} de NIS2. Entidades con 50+ empleados o 10M+ EUR ingresos se clasifican como {cat}.",
        applies_no: "No. El sector {sector} no esta en los Anexos I o II de NIS2. La entidad esta fuera de ambito.",
        sector_q: "Se aplica NIS2 a empresas del sector {sector}?",
        essential_q: "Que es una entidad esencial en NIS2?",
        important_q: "Que es una entidad importante en NIS2?",
        sanction_q: "Cuales son las sanciones NIS2?",
        obligation_q: "Cuales son las obligaciones NIS2?",
        incident_q: "Cuales son los plazos de notificacion NIS2?",
        scope_q: "Se aplica NIS2 a mi empresa?",
        mfa_q: "NIS2 exige autenticacion multifactor?",
        supply_q: "Que dice NIS2 sobre la seguridad de la cadena de suministro?",
        crypto_q: "Que exige NIS2 en materia de cifrado?",
        training_q: "NIS2 exige formacion en ciberseguridad?",
        governance_q: "Cuales son los requisitos de gobernanza NIS2?",
        dora_q: "Que es DORA?",
        threshold_q: "Cual es el umbral de tamano NIS2?",
        diff_q: "Cual es la diferencia entre esencial e importante?",
    },
];

struct SectorInfo {
    code: &'static str,
    annex: &'static str,
}

const IN_SCOPE_SECTORS: &[SectorInfo] = &[
    SectorInfo { code: "energy", annex: "Annex I" },
    SectorInfo { code: "transport", annex: "Annex I" },
    SectorInfo { code: "banking", annex: "Annex I" },
    SectorInfo { code: "health", annex: "Annex I" },
    SectorInfo { code: "drinking_water", annex: "Annex I" },
    SectorInfo { code: "digital_infrastructure", annex: "Annex I" },
    SectorInfo { code: "space", annex: "Annex I" },
    SectorInfo { code: "food", annex: "Annex II" },
    SectorInfo { code: "chemicals", annex: "Annex II" },
    SectorInfo { code: "manufacturing", annex: "Annex II" },
    SectorInfo { code: "waste_management", annex: "Annex II" },
    SectorInfo { code: "postal_courier", annex: "Annex II" },
];

const OUT_OF_SCOPE: &[&str] = &["retail", "hospitality", "agriculture", "entertainment"];

fn eval(sector: &str) -> String {
    let p = CompanyProfile {
        name: "N/A".into(), sector: sector.into(), sub_sector: None,
        employees: 200, annual_revenue_eur_m: 50.0, balance_sheet_eur_m: 50.0,
        services: vec![], member_states: vec!["IT".into()],
    };
    format!("{:?}", engine::evaluate(&p).category)
}

fn tmpl(s: &str, sector: &str, annex: &str, cat: &str) -> String {
    s.replace("{sector}", sector).replace("{annex}", annex).replace("{cat}", cat)
}

// Answers are language-independent (English) since the legal references are universal.
const ESSENTIAL_A: &str = "An essential entity (Art. 3(1)) operates in an Annex I sector (energy, transport, banking, financial market infrastructure, health, drinking water, waste water, digital infrastructure, ICT B2B, public administration, space) and meets the size threshold. Digital infrastructure is essential regardless of size.";
const IMPORTANT_A: &str = "An important entity (Art. 3(2)) operates in an Annex II sector (postal/courier, waste management, chemicals, food, manufacturing, digital providers, research) and meets the size threshold (50+ employees or 10M+ EUR).";
const SANCTION_A: &str = "Art. 34: Essential = max(10M EUR, 2% worldwide turnover). Important = max(7M EUR, 1.4% worldwide turnover).";
const OBLIGATION_A: &str = "16 obligations: Art. 20 (2 governance), Art. 21(2)(a-j) (10 cybersecurity measures), Art. 23 (4 incident reporting: CSIRT notification, 24h, 72h, 30d).";
const INCIDENT_A: &str = "Art. 23(4): (a) early warning within 24h, (b) full notification within 72h, (d) final report within 30 days.";
const SCOPE_A: &str = "NIS2 applies to entities in Annex I (11 sectors) or Annex II (7 sectors) with 50+ employees or 10M+ EUR revenue. Digital infrastructure, ICT B2B, and public administration are in scope regardless of size.";
const MFA_A: &str = "Art. 21(2)(j) requires use of multi-factor or continuous authentication solutions, secured voice/video/text communications, and protected emergency communication systems where appropriate.";
const SUPPLY_A: &str = "Art. 21(2)(d) requires supply chain security measures including security aspects of relationships between each entity and its direct suppliers or service providers.";
const CRYPTO_A: &str = "Art. 21(2)(h) requires policies and procedures on the use of cryptography and, where appropriate, encryption.";
const TRAINING_A: &str = "Art. 20(2) requires management body members to undergo cybersecurity training. Art. 21(2)(g) requires basic cyber hygiene practices and cybersecurity training for all staff.";
const GOVERNANCE_A: &str = "Art. 20(1) requires management bodies to approve cybersecurity risk measures and oversee their implementation. Art. 20(2) mandates cybersecurity training for management.";
const DORA_A: &str = "DORA (Regulation (EU) 2022/2554) covers ICT operational resilience in the financial sector. This model indexes 14 DORA provisions for semantic search but does not evaluate DORA compliance.";
const THRESHOLD_A: &str = "The size threshold follows EU Recommendation 2003/361/EC: 50+ employees OR 10M+ EUR annual revenue. Entities below both thresholds in non-critical sectors are out of scope.";
const DIFF_A: &str = "Essential (Art. 3(1)): Annex I sectors, sanctions up to 10M/2%. Important (Art. 3(2)): Annex II sectors, sanctions up to 7M/1.4%. Essential entities face stricter supervision.";

/// Build the full multilingual Q&A dataset (100+ entries).
pub fn build_dataset() -> Vec<CacheEntry> {
    let mut entries = Vec::with_capacity(150);

    for lang in LANGS {
        // Per-sector applicability (12 in-scope * 5 langs = 60)
        for si in IN_SCOPE_SECTORS {
            let cat = eval(si.code);
            entries.push(CacheEntry {
                question: tmpl(lang.sector_q, si.code, si.annex, &cat),
                answer: tmpl(lang.applies_yes, si.code, si.annex, &cat),
                category: "applicability".into(),
                embedding: vec![],
            });
        }
        // Out of scope (4 * 5 = 20)
        for sector in OUT_OF_SCOPE {
            entries.push(CacheEntry {
                question: tmpl(lang.sector_q, sector, "", "OutOfScope"),
                answer: tmpl(lang.applies_no, sector, "", "OutOfScope"),
                category: "applicability".into(),
                embedding: vec![],
            });
        }
        // Generic questions (13 * 5 = 65)
        let generic: &[(&str, &str)] = &[
            (lang.essential_q, ESSENTIAL_A),
            (lang.important_q, IMPORTANT_A),
            (lang.sanction_q, SANCTION_A),
            (lang.obligation_q, OBLIGATION_A),
            (lang.incident_q, INCIDENT_A),
            (lang.scope_q, SCOPE_A),
            (lang.mfa_q, MFA_A),
            (lang.supply_q, SUPPLY_A),
            (lang.crypto_q, CRYPTO_A),
            (lang.training_q, TRAINING_A),
            (lang.governance_q, GOVERNANCE_A),
            (lang.dora_q, DORA_A),
            (lang.threshold_q, THRESHOLD_A),
        ];
        for (q, a) in generic {
            entries.push(CacheEntry {
                question: q.to_string(),
                answer: a.to_string(),
                category: "general".into(),
                embedding: vec![],
            });
        }
        // Difference question
        entries.push(CacheEntry {
            question: lang.diff_q.to_string(),
            answer: DIFF_A.to_string(),
            category: "classification".into(),
            embedding: vec![],
        });
    }

    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dataset_has_100_plus_entries() {
        let ds = build_dataset();
        assert!(ds.len() >= 100, "Expected 100+, got {}", ds.len());
    }

    #[test]
    fn covers_five_languages() {
        let ds = build_dataset();
        // Italian
        assert!(ds.iter().any(|e| e.question.contains("NIS2 si applica")));
        // German
        assert!(ds.iter().any(|e| e.question.contains("Gilt die NIS2")));
        // French
        assert!(ds.iter().any(|e| e.question.contains("s'applique")));
        // Spanish
        assert!(ds.iter().any(|e| e.question.contains("Se aplica NIS2")));
        // English
        assert!(ds.iter().any(|e| e.question.contains("Does NIS2 apply")));
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
        let forbidden = ["maybe", "perhaps", "might", "could be",
                         "potrebbe", "forse", "probabilmente"];
        for e in build_dataset() {
            let lower = e.answer.to_lowercase();
            for w in &forbidden {
                assert!(!lower.contains(w), "{} in: {}", w, e.question);
            }
        }
    }

    #[test]
    fn all_12_sectors_covered() {
        let ds = build_dataset();
        for si in IN_SCOPE_SECTORS {
            assert!(ds.iter().any(|e| e.question.contains(si.code)),
                "Missing sector: {}", si.code);
        }
    }
}
