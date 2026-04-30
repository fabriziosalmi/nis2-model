//! Pre-computed multilingual Q&A dataset for NIS2 compliance.
//!
//! Generates 100+ entries in EN, IT, DE, FR, ES from the rule engine.

use crate::cache::CacheEntry;
use nis2_rules::engine;
use nis2_rules::schema::CompanyProfile;

#[allow(dead_code)]
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
        obligation_q: "Quali sono i principali doveri imposti dalla NIS2?",
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
const OBLIGATION_A: &str = "The NIS2 Directive introduces **16 primary obligations** structured across 3 critical domains:

### 1. Governance (Art. 20)
*   **Approval & Oversight:** Management bodies must explicitly approve cybersecurity risk management measures and oversee their implementation.
*   **Mandatory Training:** Members of the management body are required to undergo regular cybersecurity training to understand and assess risks.

### 2. Cybersecurity Risk Management (Art. 21)
Entities must implement minimum security measures, including:
*   **Risk Analysis:** Continuous risk assessment and information system security policies.
*   **Incident Handling:** Procedures for prevention, detection, and response to incidents.
*   **Business Continuity:** Backup management, disaster recovery, and crisis management.
*   **Supply Chain Security:** Evaluating the security of direct suppliers and service providers.
*   **System Security:** Secure acquisition, development, and maintenance of systems, including vulnerability handling.
*   **Effectiveness Assessment:** Regular audits and testing of cybersecurity measures.
*   **Cyber Hygiene:** Basic cyber hygiene practices and continuous training for all staff.
*   **Cryptography:** Policies and procedures regarding the use of cryptography and encryption.
*   **Access Control:** Human resources security, access control policies, and asset management.
*   **Secure Communications:** Mandatory use of multi-factor authentication (MFA) and secured voice/video communications.

### 3. Incident Reporting (Art. 23)
Strict timelines for notifying the national CSIRT of any significant incident:
*   **24h Early Warning:** Initial notification of the incident and potential cross-border impact.
*   **72h Full Notification:** Comprehensive update, initial assessment of severity, and indicators of compromise.
*   **Intermediate Report:** Provided upon request by the CSIRT.
*   **30-Day Final Report:** Detailed report including root cause, mitigation measures applied, and ongoing impact.";
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
                follow_ups: vec![],
                embedding: vec![],
            });
        }
        // Out of scope (4 * 5 = 20)
        for sector in OUT_OF_SCOPE {
            entries.push(CacheEntry {
                question: tmpl(lang.sector_q, sector, "", "OutOfScope"),
                answer: tmpl(lang.applies_no, sector, "", "OutOfScope"),
                category: "applicability".into(),
                follow_ups: vec![],
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
                follow_ups: vec![],
                embedding: vec![],
            });
        }
        // Difference question
        entries.push(CacheEntry {
            question: lang.diff_q.to_string(),
            answer: DIFF_A.to_string(),
            category: "classification".into(),
            follow_ups: vec![],
            embedding: vec![],
        });
    }

    // --- Practical operational Q&A (language-independent, the panic questions) ---
    let practical: &[(&str, &str, &str)] = &[
        // Passwords & access
        ("Do we need to rotate passwords and secrets under NIS2?",
         "Yes. Art. 21(2)(i) requires access control and asset management policies. Credential rotation, elimination of shared/root passwords, and secret lifecycle management fall under this obligation.",
         "access_control"),
        ("Can we still use shared root passwords?",
         "No. Art. 21(2)(i) requires proper access control. Shared root accounts violate the principle of individual accountability. Each administrator must have a unique credential.",
         "access_control"),
        ("Should we use SSH keys instead of passwords?",
         "Art. 21(2)(j) mandates strong authentication. SSH key-based authentication is a recognized implementation of this requirement. Password-only access to critical systems is insufficient.",
         "access_control"),
        ("Do we need a password policy?",
         "Yes. Art. 21(2)(i) covers access control policies. A password policy with minimum complexity, rotation intervals, and prohibition of reuse is a baseline measure.",
         "access_control"),
        ("Is multi-factor authentication mandatory?",
         "Art. 21(2)(j) requires use of multi-factor or continuous authentication solutions where appropriate. For access to critical systems and administrative interfaces, MFA is expected.",
         "access_control"),
        // Encryption
        ("Do we need encryption at rest?",
         "Art. 21(2)(h) requires cryptography and encryption policies. Encryption of data at rest protects against unauthorized access to stored data, especially on removable media and backups.",
         "encryption"),
        ("Do we need encryption in transit?",
         "Art. 21(2)(h) covers cryptography policies. TLS/mTLS for data in transit is a standard implementation. Art. 21(2)(j) also requires secured communications.",
         "encryption"),
        ("Do we need a vault for secrets management?",
         "Art. 21(2)(h) and (i) together require proper cryptographic key management and access control. A secrets vault (e.g. HashiCorp Vault, AWS Secrets Manager) is a common implementation pattern.",
         "encryption"),
        ("What encryption standards does NIS2 require?",
         "Art. 21(2)(h) mandates cryptography policies but does not prescribe specific algorithms. ENISA guidelines recommend AES-256 for data at rest and TLS 1.2+ for data in transit.",
         "encryption"),
        // Incident response
        ("What do we do if we get hacked?",
         "Art. 23: (1) Notify your CSIRT without undue delay. (4)(a) Send early warning within 24h. (4)(b) Full notification within 72h. (4)(d) Final report within 30 days. Art. 21(2)(b) requires an incident handling procedure to be in place before any incident occurs.",
         "incident_response"),
        ("Do we need an incident response plan?",
         "Yes. Art. 21(2)(b) requires incident handling procedures. This includes detection, analysis, containment, eradication, recovery, and post-incident review.",
         "incident_response"),
        ("Who do we notify after a breach?",
         "Art. 23(1): Notify your national CSIRT or competent authority. The early warning (24h) must indicate whether the incident is suspected to be caused by unlawful or malicious acts and whether it could have cross-border impact.",
         "incident_response"),
        // Business continuity
        ("Do we need backups?",
         "Yes. Art. 21(2)(c) explicitly requires business continuity measures including backup management and disaster recovery. Regular tested backups are a minimum requirement.",
         "business_continuity"),
        ("Do we need a disaster recovery plan?",
         "Art. 21(2)(c) requires business continuity and crisis management, including disaster recovery capabilities. The plan must be documented, tested, and regularly updated.",
         "business_continuity"),
        ("How often should we test our backups?",
         "Art. 21(2)(c) requires business continuity measures and Art. 21(2)(f) requires assessing effectiveness of security measures. Regular backup restoration tests are an expected practice.",
         "business_continuity"),
        // Supply chain
        ("Do we need to audit our vendors?",
         "Art. 21(2)(d) requires supply chain security including security aspects of relationships with direct suppliers and service providers. Vendor security assessments are expected.",
         "supply_chain"),
        ("Are our cloud providers covered by NIS2?",
         "Cloud providers (IaaS, PaaS, SaaS) fall under digital infrastructure (Annex I). Additionally, Art. 21(2)(d) requires you to manage supply chain risk from your cloud providers.",
         "supply_chain"),
        // Vulnerability management
        ("Do we need to patch our systems?",
         "Art. 21(2)(e) requires security in system acquisition, development, and maintenance including vulnerability management and disclosure. Timely patching is a core requirement.",
         "vulnerability_mgmt"),
        ("Do we need vulnerability scanning?",
         "Art. 21(2)(e) covers vulnerability management. Regular scanning, penetration testing, and coordinated vulnerability disclosure processes are expected measures.",
         "vulnerability_mgmt"),
        // Governance
        ("Is the board personally liable under NIS2?",
         "Art. 20(1) states management bodies approve and oversee cybersecurity measures and can be held responsible for violations. This creates personal accountability at board level.",
         "governance"),
        ("Does the CEO need cybersecurity training?",
         "Art. 20(2) explicitly requires management body members to undergo cybersecurity training. This includes C-level executives and board members.",
         "governance"),
        // Risk assessment
        ("Do we need a risk assessment?",
         "Art. 21(2)(a) requires risk analysis and information system security policies. A formal risk assessment is the foundational measure from which all other Art. 21 obligations derive.",
         "risk_assessment"),
        ("How often should we do risk assessments?",
         "Art. 21(2)(a) requires ongoing risk analysis. Art. 21(2)(f) requires policies to assess effectiveness. Annual risk assessments with event-driven reviews are standard practice.",
         "risk_assessment"),
        // Network security
        ("Do we need network segmentation?",
         "Art. 21(2)(a) covers information system security policies. Network segmentation is a recognized risk mitigation measure to limit lateral movement in case of breach.",
         "network_security"),
        ("Do we need a firewall?",
         "Art. 21(2)(a) requires information system security. Firewalls, IDS/IPS, and network monitoring are baseline security measures expected under this obligation.",
         "network_security"),
    ];

    for (q, a, cat) in practical {
        entries.push(CacheEntry {
            question: q.to_string(),
            answer: a.to_string(),
            category: cat.to_string(),
            follow_ups: vec![],
            embedding: vec![],
        });
    }

    // --- Extended operational Q&A (batch 2) ---
    let extended: &[(&str, &str, &str)] = &[
        // SIEM / SOC / Logging
        ("Do we need a SIEM under NIS2?",
         "Art. 21(2)(b) requires incident handling which implies detection capabilities. A SIEM or equivalent log correlation system is a standard means to detect and investigate incidents.",
         "detection"),
        ("Do we need centralized logging?",
         "Art. 21(2)(b) incident handling and Art. 21(2)(f) effectiveness assessment both require audit trails. Centralized logging is the baseline for detection, forensics, and compliance evidence.",
         "detection"),
        ("Do we need a SOC?",
         "NIS2 does not mandate a SOC by name, but Art. 21(2)(b) requires incident detection and handling capabilities. A SOC (internal or outsourced) is a common way to meet this obligation.",
         "detection"),
        ("Do we need endpoint detection and response?",
         "Art. 21(2)(b) requires incident handling including detection. EDR is a recognized measure for endpoint visibility. Art. 21(2)(e) also covers vulnerability management on endpoints.",
         "detection"),
        // Asset management
        ("Do we need an asset inventory?",
         "Art. 21(2)(i) requires asset management. You cannot protect what you do not know exists. A complete inventory of hardware, software, data, and network assets is foundational.",
         "asset_management"),
        ("Do we need to classify our data?",
         "Art. 21(2)(a) risk analysis requires understanding what data you hold and its criticality. Data classification enables proportionate security measures per Art. 21.",
         "asset_management"),
        // Secure development
        ("Does NIS2 require secure software development?",
         "Art. 21(2)(e) explicitly requires security in system acquisition, development, and maintenance. This covers secure SDLC, code review, and vulnerability management in custom software.",
         "development"),
        ("Do we need code reviews?",
         "Art. 21(2)(e) covers security in development and maintenance. Code review, static analysis, and security testing are recognized practices under this obligation.",
         "development"),
        ("Do we need penetration testing?",
         "Art. 21(2)(e) requires vulnerability management and Art. 21(2)(f) requires assessing effectiveness of security measures. Penetration testing is a standard method for both.",
         "development"),
        // Privileged access
        ("Do we need privileged access management?",
         "Art. 21(2)(i) requires access control and asset management. PAM solutions that enforce least privilege, session recording, and just-in-time access are recognized implementations.",
         "access_control"),
        ("What is the principle of least privilege under NIS2?",
         "Art. 21(2)(i) covers access control strategies. Least privilege means users and systems receive only the minimum access necessary. This applies to all accounts, especially administrative ones.",
         "access_control"),
        // Email / phishing
        ("Does NIS2 address phishing protection?",
         "Art. 21(2)(g) requires basic cyber hygiene and training. Anti-phishing measures (SPF, DKIM, DMARC, user awareness training, email filtering) fall under hygiene and incident prevention.",
         "email_security"),
        ("Do we need email security under NIS2?",
         "Art. 21(2)(g) cyber hygiene and Art. 21(2)(j) secured communications both apply. Email security measures including encryption (S/MIME or TLS), anti-spam, and anti-phishing are expected.",
         "email_security"),
        // Zero trust
        ("Does NIS2 require zero trust?",
         "NIS2 does not mandate zero trust by name. However, Art. 21(2)(i) access control, Art. 21(2)(j) continuous authentication, and Art. 21(2)(a) risk-based policies align with zero trust principles.",
         "architecture"),
        // Physical security
        ("Does NIS2 cover physical security?",
         "Art. 21(2)(a) requires information system security policies which include physical and environmental security of network and information systems. Physical access controls are in scope.",
         "physical"),
        // Remote work
        ("What does NIS2 say about remote work?",
         "Art. 21(2)(j) requires secured communications and Art. 21(2)(i) requires access control. Remote access must use VPN or zero-trust network access with MFA. BYOD policies fall under Art. 21(2)(a).",
         "remote_work"),
        ("Do we need a VPN?",
         "Art. 21(2)(j) requires secured communications. VPN or equivalent encrypted tunnels are standard for remote access to internal resources. Split tunneling policies should be risk-assessed per Art. 21(2)(a).",
         "remote_work"),
        ("Do we need a BYOD policy?",
         "Art. 21(2)(a) risk analysis and Art. 21(2)(i) asset management together require policies for personal devices accessing corporate systems. Mobile device management (MDM) is a common control.",
         "remote_work"),
        // GDPR relationship
        ("What is the relationship between NIS2 and GDPR?",
         "NIS2 covers network and information system security. GDPR covers personal data protection. They overlap on breach notification and security measures. NIS2 Art. 35 requires cooperation with data protection authorities when incidents involve personal data.",
         "legal"),
        ("Does NIS2 replace GDPR?",
         "No. NIS2 and GDPR are complementary. GDPR applies to personal data processing (72h breach notification to DPA). NIS2 applies to network security of essential/important entities (24h early warning to CSIRT). Both may apply simultaneously.",
         "legal"),
        // CISO / organizational
        ("Do we need a CISO under NIS2?",
         "NIS2 does not mandate a CISO title, but Art. 20(1) requires management body oversight of cybersecurity measures. A designated security officer or equivalent function is implied for effective governance.",
         "governance"),
        ("Who is responsible for NIS2 compliance?",
         "Art. 20(1): the management body (board of directors, executive management) approves cybersecurity measures, oversees implementation, and can be held personally responsible for violations.",
         "governance"),
        // Deadlines and transposition
        ("When does NIS2 come into effect?",
         "Directive (EU) 2022/2555 entered into force on 16 January 2023. Member states had until 17 October 2024 to transpose it into national law. Compliance obligations apply from the transposition date.",
         "legal"),
        ("Has NIS2 been transposed into national law?",
         "Transposition deadlines vary by member state. Art. 41 required transposition by 17 October 2024. Check your national competent authority for the specific implementing legislation.",
         "legal"),
        // National authorities
        ("Who is the NIS2 competent authority?",
         "Each EU member state designates competent authorities per Art. 8. Italy: ACN (Agenzia per la Cybersicurezza Nazionale). Germany: BSI. France: ANSSI. Each state also designates a CSIRT per Art. 10.",
         "legal"),
        ("What is a CSIRT?",
         "A Computer Security Incident Response Team designated per Art. 10 of NIS2. Each member state must have at least one CSIRT responsible for receiving incident notifications from essential and important entities.",
         "legal"),
        // Cross-border
        ("What if we operate in multiple EU countries?",
         "Art. 26 establishes jurisdiction rules. The primary competent authority is in the member state where the entity has its main establishment. Art. 37 enables mutual assistance between authorities.",
         "legal"),
        // Documentation
        ("What documentation does NIS2 require?",
         "Art. 21 requires documented policies for: (a) risk analysis, (c) business continuity, (d) supply chain, (h) cryptography, (i) access control. Art. 20 requires documented management approval. Art. 23 requires incident notification records.",
         "documentation"),
        ("Do we need a cybersecurity policy?",
         "Yes. Art. 21(2)(a) explicitly requires 'policies on risk analysis and information system security.' This is the foundational document from which all other Art. 21 measures derive.",
         "documentation"),
        // Change management
        ("Does NIS2 require change management?",
         "Art. 21(2)(e) covers security in system maintenance. Formal change management with security review, testing, and rollback procedures is an expected practice for maintaining system integrity.",
         "operations"),
        // DNS
        ("Does NIS2 apply to DNS providers?",
         "Yes. DNS service providers are explicitly listed in Annex I under digital infrastructure. They are classified as essential entities subject to all NIS2 obligations.",
         "applicability"),
        // Cloud security
        ("Does NIS2 apply to cloud services?",
         "Yes. Cloud computing service providers are listed in Annex I (digital infrastructure) and are classified as essential entities. They face the full set of Art. 20, 21, and 23 obligations.",
         "applicability"),
        ("Do we need cloud security posture management?",
         "Art. 21(2)(a) risk analysis and Art. 21(2)(e) system security apply to cloud environments. CSPM tools that detect misconfigurations are a recognized practice for cloud workloads.",
         "operations"),
        // API security
        ("Does NIS2 cover API security?",
         "Art. 21(2)(e) covers security in system development and maintenance. APIs are part of the system surface. Authentication, rate limiting, input validation, and encryption apply.",
         "development"),
        // Insurance
        ("Do we need cyber insurance for NIS2?",
         "NIS2 does not mandate cyber insurance. However, Art. 21(2)(a) risk analysis should evaluate residual risk. Insurance is a risk transfer mechanism that complements but does not replace technical measures.",
         "risk_assessment"),
        // Audit
        ("Can we be audited under NIS2?",
         "Art. 32 (essential) and Art. 33 (important) grant competent authorities powers to conduct audits, inspections, and on-site visits. Essential entities may face regular audits; important entities are audited ex-post.",
         "governance"),
        ("Do we need internal audits?",
         "Art. 21(2)(f) requires assessing the effectiveness of cybersecurity measures. Internal audits, security reviews, and maturity assessments are standard methods for this obligation.",
         "governance"),
        // Specific Art. 21 measures people ask about
        ("What is cyber hygiene under NIS2?",
         "Art. 21(2)(g) lists basic cyber hygiene: device configuration policies, software updates, secure account creation, password management. It also requires periodic cybersecurity training for all employees.",
         "operations"),
        ("What does NIS2 say about business continuity?",
         "Art. 21(2)(c) requires: business continuity management, backup procedures, disaster recovery, and crisis management. Plans must be documented, tested, and maintained.",
         "business_continuity"),
        ("Do we need a vulnerability disclosure policy?",
         "Art. 21(2)(e) covers coordinated vulnerability disclosure. Entities should have a process for receiving, triaging, and remediating reported vulnerabilities in their products and services.",
         "vulnerability_mgmt"),
        // Scope edge cases
        ("Are hospitals subject to NIS2?",
         "Yes. Health is listed in Annex I. Hospitals and healthcare providers meeting the size threshold (50+ employees or 10M+ EUR) are classified as essential entities.",
         "applicability"),
        ("Are universities subject to NIS2?",
         "Research organizations are listed in Annex II. Universities meeting the size threshold are classified as important entities. Those not meeting the threshold are out of scope.",
         "applicability"),
        ("Is a startup subject to NIS2?",
         "Only if it operates in an Annex I or II sector AND meets the size threshold (50+ employees or 10M+ EUR revenue). Most early-stage startups are below both thresholds and are out of scope.",
         "applicability"),
        ("Are government agencies subject to NIS2?",
         "Public administration entities are listed in Annex I and classified as essential regardless of size (Art. 2(2)). Central government bodies are in scope; local government treatment varies by member state.",
         "applicability"),
        // Penalties beyond fines
        ("Can NIS2 suspend our operations?",
         "Art. 32(5) allows competent authorities to temporarily suspend certifications or authorizations of essential entities for non-compliance. This goes beyond financial penalties.",
         "sanctions"),
        ("Can executives be banned under NIS2?",
         "Art. 32(5)(b) allows competent authorities to request a temporary prohibition of management functions for natural persons responsible for non-compliance in essential entities.",
         "sanctions"),
    ];

    for (q, a, cat) in extended {
        entries.push(CacheEntry {
            question: q.to_string(),
            answer: a.to_string(),
            category: cat.to_string(),
            follow_ups: vec![],
            embedding: vec![],
        });
    }

    // --- Short-form keyword entries (how people actually search) ---
    let keywords: &[(&str, &str, &str)] = &[
        // Password / access
        ("password rotation", "Art. 21(2)(i) requires access control policies. Password/secret rotation, prohibition of shared accounts, and credential lifecycle management are mandatory measures.", "access_control"),
        ("rotazione password", "Art. 21(2)(i) richiede politiche di controllo accessi. La rotazione delle password, il divieto di account condivisi e la gestione del ciclo di vita delle credenziali sono misure obbligatorie.", "access_control"),
        ("shared root password", "Art. 21(2)(i): shared root accounts violate individual accountability. Each administrator must have unique credentials. Root/admin access must use MFA per Art. 21(2)(j).", "access_control"),
        ("password condivise", "Art. 21(2)(i): gli account root condivisi violano il principio di responsabilita individuale. Ogni amministratore deve avere credenziali uniche.", "access_control"),
        ("ssh keys", "Art. 21(2)(j) mandates strong authentication. SSH key-based auth is a recognized implementation. Password-only access to critical systems is insufficient.", "access_control"),
        ("chiavi ssh", "Art. 21(2)(j) richiede autenticazione forte. L'autenticazione basata su chiavi SSH e' un'implementazione riconosciuta. L'accesso solo con password e' insufficiente.", "access_control"),
        ("MFA", "Art. 21(2)(j) requires multi-factor or continuous authentication. MFA is expected for all administrative access and critical systems.", "access_control"),
        ("autenticazione multifattore", "Art. 21(2)(j) richiede soluzioni di autenticazione a piu fattori. MFA e' richiesto per accessi amministrativi e sistemi critici.", "access_control"),
        // Encryption
        ("encryption at rest", "Art. 21(2)(h) requires cryptography policies. Encryption of data at rest (AES-256 recommended by ENISA) protects stored data, backups, and removable media.", "encryption"),
        ("cifratura dati a riposo", "Art. 21(2)(h) richiede politiche di crittografia. La cifratura dei dati a riposo (AES-256 raccomandato ENISA) protegge dati, backup e supporti rimovibili.", "encryption"),
        ("encryption in transit", "Art. 21(2)(h) cryptography + Art. 21(2)(j) secured communications. TLS 1.2+ for all data in transit. mTLS for service-to-service communication.", "encryption"),
        ("cifratura in transito", "Art. 21(2)(h) crittografia + Art. 21(2)(j) comunicazioni protette. TLS 1.2+ per dati in transito. mTLS per comunicazione tra servizi.", "encryption"),
        ("vault", "Art. 21(2)(h) and (i): proper key management and access control require a secrets management solution. HashiCorp Vault, AWS Secrets Manager, or equivalent.", "encryption"),
        ("gestione segreti", "Art. 21(2)(h) e (i): la gestione delle chiavi crittografiche e il controllo accessi richiedono una soluzione di gestione dei segreti (Vault, Secrets Manager).", "encryption"),
        ("TLS", "Art. 21(2)(h) and (j): TLS 1.2+ is the minimum for data in transit. TLS 1.3 is recommended. Certificate management and rotation are part of cryptography policies.", "encryption"),
        // Incident response
        ("breach notification", "Art. 23: early warning to CSIRT within 24h, full notification within 72h, final report within 30 days. Applies to significant incidents affecting service provision.", "incident_response"),
        ("notifica incidente", "Art. 23: preallarme al CSIRT entro 24h, notifica completa entro 72h, relazione finale entro 30 giorni. Si applica a incidenti significativi.", "incident_response"),
        ("24 ore", "Art. 23(4)(a): early warning to CSIRT within 24 hours of becoming aware of a significant incident. Must indicate if malicious and if cross-border.", "incident_response"),
        ("72 ore", "Art. 23(4)(b): full incident notification to CSIRT within 72 hours. Must include severity assessment, impact, and indicators of compromise.", "incident_response"),
        ("incident response", "Art. 21(2)(b) requires incident handling: detection, analysis, containment, eradication, recovery, post-incident review. Procedures must be documented and tested.", "incident_response"),
        ("risposta incidenti", "Art. 21(2)(b) richiede gestione incidenti: rilevamento, analisi, contenimento, eliminazione, ripristino, revisione post-incidente.", "incident_response"),
        // Backup / DR
        ("backup", "Art. 21(2)(c) explicitly requires backup management and disaster recovery. Regular tested backups with offsite/offline copies are a minimum.", "business_continuity"),
        ("disaster recovery", "Art. 21(2)(c) requires business continuity including disaster recovery. RTO/RPO must be defined, plans documented, and recovery tested regularly.", "business_continuity"),
        ("continuita operativa", "Art. 21(2)(c) richiede continuita operativa: gestione backup, ripristino in caso di disastro, gestione delle crisi. I piani devono essere documentati e testati.", "business_continuity"),
        // Supply chain
        ("supply chain", "Art. 21(2)(d) requires supply chain security: security clauses in contracts, vendor assessments, monitoring of supplier risk, and incident coordination.", "supply_chain"),
        ("catena di approvvigionamento", "Art. 21(2)(d) richiede sicurezza della catena di fornitura: clausole di sicurezza nei contratti, valutazione fornitori, monitoraggio rischi.", "supply_chain"),
        ("fornitori", "Art. 21(2)(d): la sicurezza dei rapporti con fornitori diretti e' un obbligo. Include valutazioni di sicurezza, SLA, e coordinamento incidenti.", "supply_chain"),
        // Vulnerability management
        ("patching", "Art. 21(2)(e) requires vulnerability management. Timely patching of known vulnerabilities is a core requirement. Prioritize by CVSS severity.", "vulnerability_mgmt"),
        ("vulnerability scanning", "Art. 21(2)(e) covers vulnerability management. Regular scanning (weekly for internet-facing, monthly for internal) and penetration testing are expected.", "vulnerability_mgmt"),
        ("penetration test", "Art. 21(2)(e) vulnerability management + Art. 21(2)(f) effectiveness assessment. Annual pentest minimum, more frequent for critical infrastructure.", "vulnerability_mgmt"),
        // Governance
        ("board liability", "Art. 20(1): management bodies approve and oversee cybersecurity measures. They can be held personally responsible for violations. Art. 32(5)(b) allows temporary management bans.", "governance"),
        ("responsabilita CDA", "Art. 20(1): gli organi direttivi approvano le misure di cybersecurity e ne sovraintendono l'attuazione. Possono essere ritenuti responsabili delle violazioni.", "governance"),
        ("CISO", "NIS2 does not mandate a CISO title, but Art. 20(1) requires management oversight. A designated security function reporting to the board is implied.", "governance"),
        ("formazione cybersecurity", "Art. 20(2) richiede formazione obbligatoria in cybersecurity per gli organi direttivi. Art. 21(2)(g) richiede formazione per tutti i dipendenti.", "governance"),
        // Risk
        ("risk assessment", "Art. 21(2)(a) requires risk analysis. This is the foundation: identify assets, threats, vulnerabilities, impacts. Annual minimum, plus event-driven reviews.", "risk_assessment"),
        ("analisi rischi", "Art. 21(2)(a) richiede analisi dei rischi: identificare asset, minacce, vulnerabilita, impatti. Almeno annuale, piu revisioni al verificarsi di eventi.", "risk_assessment"),
        // Network
        ("firewall", "Art. 21(2)(a) information system security. Firewalls, IDS/IPS, WAF, and network monitoring are baseline measures. Segmentation limits lateral movement.", "network_security"),
        ("segmentazione rete", "Art. 21(2)(a) sicurezza dei sistemi informatici. La segmentazione di rete limita il movimento laterale in caso di compromissione.", "network_security"),
        // Sanctions
        ("sanzioni", SANCTION_A, "sanctions"),
        ("multa NIS2", SANCTION_A, "sanctions"),
        ("NIS2 fine", SANCTION_A, "sanctions"),
        ("10 milioni", "Art. 34(4): 10,000,000 EUR is the floor sanction for essential entities. If 2% of worldwide turnover exceeds 10M, the higher amount applies.", "sanctions"),
    ];

    for (q, a, cat) in keywords {
        entries.push(CacheEntry {
            question: q.to_string(),
            answer: a.to_string(),
            category: cat.to_string(),
            follow_ups: vec![],
            embedding: vec![],
        });
    }

    // --- Level 2/3: implementation detail entries ---
    let deep: &[(&str, &str, &str)] = &[
        // Access control implementation
        ("How to implement password rotation",
         "Art. 21(2)(i): Implement via Active Directory GPO or IAM policies. Set maximum password age (90 days recommended), enforce history (24 passwords), minimum length (14+ chars), complexity requirements. Integrate with PAM for privileged accounts.",
         "access_control_impl"),
        ("What is a PAM solution",
         "Privileged Access Management enforces Art. 21(2)(i). Key features: credential vaulting, session recording, just-in-time access, approval workflows, automatic rotation of service accounts. Examples: CyberArk, BeyondTrust, Delinea.",
         "access_control_impl"),
        ("How to implement MFA",
         "Art. 21(2)(j): Deploy MFA via TOTP apps (Google/Microsoft Authenticator), hardware keys (YubiKey/FIDO2), or push notifications. Prioritize: admin accounts first, then VPN, then all users. Avoid SMS-based MFA (SIM swap risk).",
         "access_control_impl"),
        ("What is RBAC",
         "Role-Based Access Control implements Art. 21(2)(i). Define roles by job function, assign minimum required permissions per role. Review role assignments quarterly. Separate duties for critical operations (maker-checker).",
         "access_control_impl"),
        ("How to manage service accounts",
         "Art. 21(2)(i): Inventory all service accounts. Store credentials in vault (not in code/config). Rotate automatically (30-90 day cycle). Monitor for anomalous usage. Assign individual owners. Disable when decommissioned.",
         "access_control_impl"),
        // Encryption implementation
        ("How to implement encryption at rest",
         "Art. 21(2)(h): Use AES-256 for data at rest. Enable LUKS/BitLocker for disk encryption. Use database-level TDE for structured data. Encrypt backups before offsite transfer. Manage keys in HSM or KMS, never alongside data.",
         "encryption_impl"),
        ("How to implement TLS",
         "Art. 21(2)(h)(j): Enforce TLS 1.2 minimum, prefer TLS 1.3. Disable SSLv3, TLS 1.0, 1.1. Use strong cipher suites (ECDHE, AES-GCM). Automate certificate renewal (Let's Encrypt/ACME). Deploy HSTS headers. Monitor certificate expiry.",
         "encryption_impl"),
        ("What is an HSM",
         "Hardware Security Module provides tamper-resistant key storage per Art. 21(2)(h). Use for: CA private keys, database encryption master keys, code signing. Cloud alternatives: AWS CloudHSM, Azure Dedicated HSM, GCP Cloud HSM.",
         "encryption_impl"),
        ("How to manage encryption keys",
         "Art. 21(2)(h): Implement key lifecycle: generation (use CSPRNG), distribution (encrypted channels), storage (HSM/KMS), rotation (annual minimum), revocation, destruction. Separate key management from data management roles.",
         "encryption_impl"),
        // Incident response implementation
        ("How to build an incident response plan",
         "Art. 21(2)(b): Define: (1) classification criteria (P1-P4), (2) escalation matrix, (3) communication templates (internal/external), (4) containment procedures per attack type, (5) evidence preservation steps, (6) post-incident review process. Test via tabletop exercises quarterly.",
         "incident_impl"),
        ("What is an incident classification",
         "Art. 21(2)(b): Classify by impact: P1 (critical service disruption, data breach), P2 (significant degradation), P3 (minor impact), P4 (informational). P1/P2 trigger Art. 23 notification. Map to response times: P1=immediate, P2=4h, P3=24h, P4=72h.",
         "incident_impl"),
        ("How to do a post-incident review",
         "Art. 21(2)(b): Conduct blameless review within 5 business days. Document: timeline, root cause, detection lag, containment effectiveness, communication gaps. Produce action items with owners and deadlines. Feed into risk assessment (Art. 21(2)(a)).",
         "incident_impl"),
        ("What is a tabletop exercise",
         "Art. 21(2)(b)(f): Simulated incident scenario for decision-makers. No live systems affected. Test: detection, escalation, communication, decision-making. Run quarterly minimum. Document findings. Satisfies Art. 21(2)(f) effectiveness assessment.",
         "incident_impl"),
        // Business continuity implementation
        ("What is RTO and RPO",
         "Art. 21(2)(c): RTO (Recovery Time Objective) = max acceptable downtime. RPO (Recovery Point Objective) = max acceptable data loss. Define per system/service. Critical systems: RTO <4h, RPO <1h. Non-critical: RTO <24h, RPO <24h.",
         "bc_impl"),
        ("How to test disaster recovery",
         "Art. 21(2)(c)(f): Test types: (1) tabletop walkthrough (quarterly), (2) partial failover (semi-annual), (3) full failover (annual). Document: actual RTO/RPO vs target, issues found, remediation actions. Ensure backup restoration is verified.",
         "bc_impl"),
        ("How to implement backup strategy",
         "Art. 21(2)(c): Follow 3-2-1 rule: 3 copies, 2 different media, 1 offsite. Encrypt backups at rest. Test restoration monthly. Automate verification (checksum). Define retention per data type. Air-gap at least one backup copy from network.",
         "bc_impl"),
        // Supply chain implementation
        ("How to assess vendor security",
         "Art. 21(2)(d): Use standardized questionnaire (SIG/CAIQ). Require SOC 2 Type II or ISO 27001. Include security SLA in contracts. Right-to-audit clause. Annual reassessment. Monitor vendor breaches via threat intelligence.",
         "supply_chain_impl"),
        ("What security clauses for contracts",
         "Art. 21(2)(d): Include: data processing terms, breach notification (<24h to you), right to audit, subcontractor approval, encryption requirements, data residency, incident coordination, termination data return/destruction.",
         "supply_chain_impl"),
        ("How to monitor third-party risk",
         "Art. 21(2)(d): Continuous monitoring: security ratings (BitSight, SecurityScorecard), vendor breach alerts, dark web monitoring for vendor data. Tier vendors by criticality. Critical vendors: quarterly review. Others: annual.",
         "supply_chain_impl"),
        // Vulnerability management implementation
        ("How to implement patch management",
         "Art. 21(2)(e): Define SLA by severity: critical (48h), high (7d), medium (30d), low (90d). Use vulnerability scanner (Nessus, Qualys). Automate OS patches (WSUS, SCCM, unattended-upgrades). Test patches in staging before production.",
         "vuln_impl"),
        ("How to set up vulnerability scanning",
         "Art. 21(2)(e): Authenticated scans weekly for internet-facing. Monthly for internal. Use both network (Nessus) and web app (OWASP ZAP, Burp) scanners. Integrate results into ticketing. Track mean-time-to-remediate metrics.",
         "vuln_impl"),
        ("How to scope a penetration test",
         "Art. 21(2)(e)(f): Define: scope (IP ranges, apps, social engineering?), rules of engagement, testing window, emergency contacts. Black-box for external, grey-box for internal. Annual minimum. Retest after critical findings are fixed.",
         "vuln_impl"),
        // Detection implementation
        ("How to implement SIEM",
         "Art. 21(2)(b): Collect logs from: firewalls, endpoints, AD/IAM, DNS, proxy, email gateway, cloud control plane. Define correlation rules for: brute force, lateral movement, data exfil, privilege escalation. Set alert priorities. Tune false positives.",
         "detection_impl"),
        ("What logs to collect",
         "Art. 21(2)(b): Minimum: authentication events, privilege changes, firewall allow/deny, DNS queries, email gateway, VPN connections, file access on sensitive shares, cloud API calls, endpoint process creation. Retain 12+ months.",
         "detection_impl"),
        ("How to implement EDR",
         "Art. 21(2)(b)(e): Deploy agent on all endpoints. Enable: process monitoring, file integrity, network connections, memory analysis. Set detection rules for: ransomware behavior, credential dumping, living-off-the-land binaries. Integrate with SIEM.",
         "detection_impl"),
        // Governance implementation
        ("How to brief the board on cybersecurity",
         "Art. 20(1)(2): Present quarterly: (1) risk posture dashboard, (2) open critical vulnerabilities, (3) incident summary, (4) compliance status vs Art. 21 measures, (5) investment requests. Use business language, not technical jargon. Document board resolutions.",
         "governance_impl"),
        ("What training does the board need",
         "Art. 20(2): Annual minimum. Cover: NIS2 obligations and personal liability, current threat landscape, company risk posture, incident response roles, social engineering awareness. External instructors recommended for independence.",
         "governance_impl"),
        ("How to structure a security team",
         "Art. 20(1): Minimum functions: CISO (reports to board), security operations (monitoring), security engineering (architecture), GRC (compliance/risk). Small companies: outsource SOC, retain CISO function internally. Document reporting lines.",
         "governance_impl"),
        // Risk assessment implementation
        ("How to do a risk assessment",
         "Art. 21(2)(a): Framework: ISO 27005 or NIST CSF. Steps: (1) asset inventory, (2) threat identification, (3) vulnerability assessment, (4) impact analysis, (5) likelihood rating, (6) risk scoring (impact x likelihood), (7) treatment plan. Document residual risk.",
         "risk_impl"),
        ("What risk framework to use",
         "Art. 21(2)(a): Options: ISO 27005 (EU standard, aligned with NIS2), NIST CSF (US, widely adopted), EBIOS RM (French, endorsed by ANSSI), BSI IT-Grundschutz (German). Choose based on your member state's guidance and existing certifications.",
         "risk_impl"),
        ("How to do a business impact analysis",
         "Art. 21(2)(a)(c): Identify critical processes and supporting systems. For each: define impact of outage (financial, reputational, legal, safety). Determine RTO/RPO. Map dependencies. Output feeds both risk assessment and business continuity planning.",
         "risk_impl"),
        // Network implementation
        ("How to implement network segmentation",
         "Art. 21(2)(a): Segment by function and sensitivity: DMZ (internet-facing), corporate LAN, OT network (if applicable), management network, guest network. Use VLANs + firewall rules between zones. Micro-segment critical workloads. Log inter-zone traffic.",
         "network_impl"),
        ("How to implement zero trust",
         "Art. 21(2)(a)(i)(j): Principles: verify explicitly, least privilege, assume breach. Implementation: identity-based access (not network), micro-segmentation, continuous authentication, device posture checks, encrypted all traffic. Start with admin access, expand.",
         "network_impl"),
        ("What is network monitoring",
         "Art. 21(2)(b): Deploy: NetFlow/sFlow collection, IDS/IPS (Suricata, Snort), DNS monitoring, full packet capture on critical segments. Detect: C2 beaconing, data exfiltration, unauthorized protocols, ARP spoofing. Feed alerts to SIEM.",
         "network_impl"),
        // Email/phishing implementation
        ("How to implement email security",
         "Art. 21(2)(g)(j): Deploy: SPF (strict -all), DKIM (2048-bit), DMARC (p=reject). Add email filtering gateway. Enable sandboxing for attachments. User awareness training quarterly. Simulate phishing monthly. Report button in email client.",
         "email_impl"),
        ("How to run phishing simulations",
         "Art. 21(2)(g): Monthly simulated phishing campaigns. Vary templates: credential harvesting, malicious attachment, CEO fraud. Track: click rate, report rate, repeat offenders. Targeted training for users who fail. Goal: <5% click rate.",
         "email_impl"),
        // Documentation implementation
        ("What policies do we need to write",
         "Art. 21: Minimum policies: (1) information security policy, (2) access control policy, (3) incident response plan, (4) business continuity plan, (5) cryptography policy, (6) supply chain security policy, (7) vulnerability management policy, (8) acceptable use policy.",
         "doc_impl"),
        ("How to structure a security policy",
         "Art. 21(2)(a): Structure: (1) purpose and scope, (2) roles and responsibilities, (3) policy statements with Art. references, (4) standards and procedures, (5) exceptions process, (6) review cycle (annual minimum). Approved by management body per Art. 20(1).",
         "doc_impl"),
        // Legal implementation
        ("How to register with the competent authority",
         "Art. 27: Essential and important entities must register with their national competent authority. Provide: entity name, sector, contact details, IP ranges, member states of operation. Deadlines vary by member state.",
         "legal_impl"),
        ("How to prepare for a NIS2 audit",
         "Art. 32/33: Prepare: (1) evidence portfolio mapped to Art. 21(2)(a-j), (2) incident logs (Art. 23), (3) training records (Art. 20(2)), (4) risk assessment reports, (5) third-party audit reports (SOC2/ISO27001), (6) board meeting minutes showing cybersecurity oversight.",
         "legal_impl"),
        // Physical security
        ("How to implement physical access control",
         "Art. 21(2)(a): Measures: badge access for server rooms, visitor logs, CCTV at entry points, clean desk policy, device locking, secure disposal of media. Data center: mantrap, biometric access, environmental monitoring (temperature, water, fire).",
         "physical_impl"),
        // Remote work implementation
        ("How to secure remote access",
         "Art. 21(2)(i)(j): Deploy: always-on VPN or ZTNA solution, MFA for all remote access, endpoint compliance checks (patch level, AV status), split tunnel prohibition for high-security environments, DLP on endpoints, remote wipe capability.",
         "remote_impl"),
        ("How to implement MDM",
         "Art. 21(2)(a)(i): Mobile Device Management: enforce encryption, remote wipe, minimum OS version, app allowlist, certificate-based Wi-Fi, containerization for BYOD. Separate personal/corporate data. Compliance check before network access.",
         "remote_impl"),
    ];

    for (q, a, cat) in deep {
        entries.push(CacheEntry {
            question: q.to_string(),
            answer: a.to_string(),
            category: cat.to_string(),
            follow_ups: vec![],
            embedding: vec![],
        });
    }

    // --- Italian + EN paraphrase entries from dataset_it ---
    for (q, a, cat) in crate::dataset_it::italian_entries() {
        entries.push(CacheEntry {
            question: q.to_string(),
            answer: a.to_string(),
            category: cat.to_string(),
            follow_ups: vec![],
            embedding: vec![],
        });
    }

    // --- Italian expansion (2x coverage) ---
    for (q, a, cat) in crate::dataset_it2::italian_expansion() {
        entries.push(CacheEntry {
            question: q.to_string(),
            answer: a.to_string(),
            category: cat.to_string(),
            follow_ups: vec![],
            embedding: vec![],
        });
    }

    // --- Italian deep-dive entries (long-form detailed answers) ---
    for (q, a, cat) in crate::dataset_it3::italian_deep() {
        entries.push(CacheEntry {
            question: q.to_string(),
            answer: a.to_string(),
            category: cat.to_string(),
            follow_ups: vec![],
            embedding: vec![],
        });
    }

    // --- Scenario entries (panic questions EN+IT) ---
    for (q, a, cat) in crate::dataset_scenarios::scenario_entries() {
        entries.push(CacheEntry {
            question: q.to_string(),
            answer: a.to_string(),
            category: cat.to_string(),
            follow_ups: vec![],
            embedding: vec![],
        });
    }

    // --- Operational security entries EN+IT ---
    for (q, a, cat) in crate::dataset_ops::ops_entries() {
        entries.push(CacheEntry {
            question: q.to_string(),
            answer: a.to_string(),
            category: cat.to_string(),
            follow_ups: vec![],
            embedding: vec![],
        });
    }

    // --- Legal/compliance mapping entries EN+IT ---
    for (q, a, cat) in crate::dataset_legal::legal_entries() {
        entries.push(CacheEntry {
            question: q.to_string(),
            answer: a.to_string(),
            category: cat.to_string(),
            follow_ups: vec![],
            embedding: vec![],
        });
    }

    // --- Assign follow-ups by category (deterministic conversation tree) ---
    for entry in &mut entries {
        entry.follow_ups = follow_ups_for(&entry.category);
    }

    entries
}

/// Deterministic follow-up paths per category.
fn follow_ups_for(category: &str) -> Vec<String> {
    match category {
        "applicability" => vec![
            "What are the NIS2 obligations?".into(),
            "What are the NIS2 sanctions?".into(),
            "What is the NIS2 size threshold?".into(),
            "What is the difference between essential and important?".into(),
            "When does NIS2 come into effect?".into(),
            "Do we need a risk assessment?".into(),
        ],
        "access_control" => vec![
            "How to implement MFA".into(),
            "How to implement password rotation".into(),
            "What is a PAM solution".into(),
            "How to manage service accounts".into(),
            "Do we need a vault for secrets management?".into(),
            "What is RBAC".into(),
        ],
        "access_control_impl" => vec![
            "Do we need privileged access management?".into(),
            "Do we need a vault for secrets management?".into(),
            "Do we need centralized logging?".into(),
            "What documentation does NIS2 require?".into(),
            "Do we need a risk assessment?".into(),
        ],
        "encryption" => vec![
            "How to implement encryption at rest".into(),
            "How to implement TLS".into(),
            "How to manage encryption keys".into(),
            "What is an HSM".into(),
            "Do we need a vault for secrets management?".into(),
            "What encryption standards does NIS2 require?".into(),
        ],
        "encryption_impl" => vec![
            "Do we need encryption at rest?".into(),
            "Do we need encryption in transit?".into(),
            "How to manage encryption keys".into(),
            "What is an HSM".into(),
            "What policies do we need to write".into(),
        ],
        "incident_response" => vec![
            "How to build an incident response plan".into(),
            "What is an incident classification".into(),
            "How to do a post-incident review".into(),
            "What is a tabletop exercise".into(),
            "Who do we notify after a breach?".into(),
            "Do we need a SIEM under NIS2?".into(),
        ],
        "incident_impl" => vec![
            "What are the NIS2 incident reporting deadlines?".into(),
            "Do we need centralized logging?".into(),
            "How to implement SIEM".into(),
            "How to prepare for a NIS2 audit".into(),
            "Do we need a disaster recovery plan?".into(),
        ],
        "business_continuity" => vec![
            "What is RTO and RPO".into(),
            "How to implement backup strategy".into(),
            "How to test disaster recovery".into(),
            "Do we need a disaster recovery plan?".into(),
            "How to do a business impact analysis".into(),
            "What documentation does NIS2 require?".into(),
        ],
        "bc_impl" => vec![
            "How to test disaster recovery".into(),
            "How to implement backup strategy".into(),
            "How to do a business impact analysis".into(),
            "Do we need a risk assessment?".into(),
            "What policies do we need to write".into(),
        ],
        "supply_chain" => vec![
            "How to assess vendor security".into(),
            "What security clauses for contracts".into(),
            "How to monitor third-party risk".into(),
            "Are our cloud providers covered by NIS2?".into(),
            "Does NIS2 require change management?".into(),
            "Do we need a risk assessment?".into(),
        ],
        "supply_chain_impl" => vec![
            "How to assess vendor security".into(),
            "What security clauses for contracts".into(),
            "How to monitor third-party risk".into(),
            "What documentation does NIS2 require?".into(),
            "Can we be audited under NIS2?".into(),
        ],
        "vulnerability_mgmt" => vec![
            "How to implement patch management".into(),
            "How to set up vulnerability scanning".into(),
            "How to scope a penetration test".into(),
            "Does NIS2 require secure software development?".into(),
            "Do we need a vulnerability disclosure policy?".into(),
            "Do we need code reviews?".into(),
        ],
        "vuln_impl" => vec![
            "How to implement patch management".into(),
            "How to set up vulnerability scanning".into(),
            "How to scope a penetration test".into(),
            "Do we need endpoint detection and response?".into(),
            "What policies do we need to write".into(),
        ],
        "governance" => vec![
            "How to brief the board on cybersecurity".into(),
            "What training does the board need".into(),
            "How to structure a security team".into(),
            "Is the board personally liable under NIS2?".into(),
            "Can executives be banned under NIS2?".into(),
            "How to prepare for a NIS2 audit".into(),
        ],
        "governance_impl" => vec![
            "How to brief the board on cybersecurity".into(),
            "What training does the board need".into(),
            "How to structure a security team".into(),
            "What policies do we need to write".into(),
            "Can we be audited under NIS2?".into(),
        ],
        "risk_assessment" => vec![
            "How to do a risk assessment".into(),
            "What risk framework to use".into(),
            "How to do a business impact analysis".into(),
            "How often should we do risk assessments?".into(),
            "Do we need cyber insurance for NIS2?".into(),
            "What are the NIS2 obligations?".into(),
        ],
        "risk_impl" => vec![
            "How to do a risk assessment".into(),
            "What risk framework to use".into(),
            "How to do a business impact analysis".into(),
            "What documentation does NIS2 require?".into(),
            "How to prepare for a NIS2 audit".into(),
        ],
        "network_security" => vec![
            "How to implement network segmentation".into(),
            "How to implement zero trust".into(),
            "What is network monitoring".into(),
            "Do we need a firewall?".into(),
            "Do we need endpoint detection and response?".into(),
            "Do we need a VPN?".into(),
        ],
        "network_impl" => vec![
            "How to implement network segmentation".into(),
            "How to implement zero trust".into(),
            "What is network monitoring".into(),
            "Do we need a SIEM under NIS2?".into(),
            "How to implement EDR".into(),
        ],
        "detection" => vec![
            "How to implement SIEM".into(),
            "What logs to collect".into(),
            "How to implement EDR".into(),
            "Do we need centralized logging?".into(),
            "Do we need an incident response plan?".into(),
            "What is network monitoring".into(),
        ],
        "detection_impl" => vec![
            "How to implement SIEM".into(),
            "What logs to collect".into(),
            "How to implement EDR".into(),
            "How to build an incident response plan".into(),
            "What is network monitoring".into(),
        ],
        "sanctions" => vec![
            "Can NIS2 suspend our operations?".into(),
            "Can executives be banned under NIS2?".into(),
            "Is the board personally liable under NIS2?".into(),
            "How to prepare for a NIS2 audit".into(),
            "What are the NIS2 obligations?".into(),
        ],
        "legal" => vec![
            "When does NIS2 come into effect?".into(),
            "What is the relationship between NIS2 and GDPR?".into(),
            "Who is the NIS2 competent authority?".into(),
            "How to register with the competent authority".into(),
            "What if we operate in multiple EU countries?".into(),
            "How to prepare for a NIS2 audit".into(),
        ],
        "legal_impl" => vec![
            "How to register with the competent authority".into(),
            "How to prepare for a NIS2 audit".into(),
            "What documentation does NIS2 require?".into(),
            "Can we be audited under NIS2?".into(),
            "Who is the NIS2 competent authority?".into(),
        ],
        "documentation" | "doc_impl" => vec![
            "What policies do we need to write".into(),
            "How to structure a security policy".into(),
            "How to prepare for a NIS2 audit".into(),
            "What are the NIS2 governance requirements?".into(),
            "Do we need a cybersecurity policy?".into(),
        ],
        "email_security" | "email_impl" => vec![
            "How to implement email security".into(),
            "How to run phishing simulations".into(),
            "Does NIS2 require cybersecurity training?".into(),
            "What is cyber hygiene under NIS2?".into(),
            "Do we need a SIEM under NIS2?".into(),
        ],
        "physical" | "physical_impl" => vec![
            "How to implement physical access control".into(),
            "Do we need an asset inventory?".into(),
            "Do we need network segmentation?".into(),
            "What documentation does NIS2 require?".into(),
        ],
        "remote_work" | "remote_impl" => vec![
            "How to secure remote access".into(),
            "How to implement MDM".into(),
            "Do we need a VPN?".into(),
            "Is multi-factor authentication mandatory?".into(),
            "Do we need a BYOD policy?".into(),
        ],
        "architecture" => vec![
            "How to implement zero trust".into(),
            "How to implement network segmentation".into(),
            "Do we need endpoint detection and response?".into(),
            "Is multi-factor authentication mandatory?".into(),
            "What is network monitoring".into(),
        ],
        "asset_management" => vec![
            "Do we need an asset inventory?".into(),
            "Do we need to classify our data?".into(),
            "Do we need a risk assessment?".into(),
            "What policies do we need to write".into(),
        ],
        "operations" => vec![
            "Does NIS2 require change management?".into(),
            "What is cyber hygiene under NIS2?".into(),
            "Do we need cloud security posture management?".into(),
            "How to implement patch management".into(),
            "What policies do we need to write".into(),
        ],
        "development" => vec![
            "Does NIS2 require secure software development?".into(),
            "Do we need code reviews?".into(),
            "Do we need penetration testing?".into(),
            "Does NIS2 cover API security?".into(),
            "Do we need a vulnerability disclosure policy?".into(),
        ],
        "dora" => vec![
            "What is the relationship between NIS2 and GDPR?".into(),
            "What are the NIS2 obligations?".into(),
            "Does NIS2 apply to banking companies?".into(),
        ],
        "general" | "classification" => vec![
            "What are the NIS2 obligations?".into(),
            "What are the NIS2 sanctions?".into(),
            "What are the NIS2 incident reporting deadlines?".into(),
            "Do we need a risk assessment?".into(),
            "Is the board personally liable under NIS2?".into(),
            "When does NIS2 come into effect?".into(),
        ],
        _ => vec![
            "What are the NIS2 obligations?".into(),
            "What are the NIS2 sanctions?".into(),
            "What are the NIS2 incident reporting deadlines?".into(),
            "Do we need a risk assessment?".into(),
            "Is the board personally liable under NIS2?".into(),
            "When does NIS2 come into effect?".into(),
        ],
    }
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
