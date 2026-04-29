//! Legal, compliance mapping, and sector-specific entries.
//! Bilingual EN+IT paired entries.

pub fn legal_entries() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        // === NIS2 vs ISO 27001 ===
        ("How does NIS2 map to ISO 27001?",
         "Art. 21 aligns closely with ISO 27001 Annex A controls. Key gaps: NIS2 adds explicit incident notification timelines (Art. 23: 24h/72h), board personal liability (Art. 20), and supply chain obligations (Art. 21(2)(d)). ISO 27001 certification covers approximately 70-80% of NIS2 requirements.",
         "governance"),
        ("Come si mappa NIS2 su ISO 27001?",
         "Art. 21 si allinea con i controlli Annex A di ISO 27001. Lacune principali: NIS2 aggiunge tempistiche esplicite per notifica incidenti (Art. 23: 24h/72h), responsabilita personale del CDA (Art. 20) e obblighi supply chain (Art. 21(2)(d)). La certificazione ISO 27001 copre circa il 70-80% dei requisiti NIS2.",
         "governance"),

        // === NIS2 vs NIST CSF ===
        ("How does NIS2 map to NIST CSF?",
         "NIST CSF 2.0 functions map to NIS2: Govern=Art.20, Identify=Art.21(2)(a)(i), Protect=Art.21(2)(e)(h)(i)(j), Detect=Art.21(2)(b), Respond=Art.21(2)(b)+Art.23, Recover=Art.21(2)(c). NIST CSF is a framework, NIS2 is law with sanctions. They are complementary.",
         "governance"),
        ("Come si mappa NIS2 su NIST CSF?",
         "Le funzioni NIST CSF 2.0 si mappano su NIS2: Govern=Art.20, Identify=Art.21(2)(a)(i), Protect=Art.21(2)(e)(h)(i)(j), Detect=Art.21(2)(b), Respond=Art.21(2)(b)+Art.23, Recover=Art.21(2)(c). NIST CSF e' un framework, NIS2 e' legge con sanzioni. Sono complementari.",
         "governance"),

        // === NIS2 vs CIS Controls ===
        ("How does NIS2 map to CIS Controls?",
         "CIS Controls v8 map well to NIS2 Art. 21: CIS 1-2 (Inventory)=Art.21(2)(i), CIS 3 (Data Protection)=Art.21(2)(h), CIS 4 (Secure Config)=Art.21(2)(e), CIS 6 (Access)=Art.21(2)(i)(j), CIS 7 (Vulnerability)=Art.21(2)(e), CIS 8 (Audit Log)=Art.21(2)(b), CIS 17 (Incident)=Art.21(2)(b)+Art.23.",
         "governance"),
        ("Come si mappa NIS2 sui CIS Controls?",
         "I CIS Controls v8 si mappano bene su NIS2 Art. 21: CIS 1-2 (Inventario)=Art.21(2)(i), CIS 3 (Protezione Dati)=Art.21(2)(h), CIS 4 (Config Sicura)=Art.21(2)(e), CIS 6 (Accesso)=Art.21(2)(i)(j), CIS 7 (Vulnerabilita)=Art.21(2)(e), CIS 8 (Log Audit)=Art.21(2)(b), CIS 17 (Incidenti)=Art.21(2)(b)+Art.23.",
         "governance"),

        // === SECTOR: ENERGY ===
        ("NIS2 requirements for the energy sector",
         "Energy is Annex I (essential entity). Sector-specific: (1) IEC 62443 for OT/SCADA systems. (2) NERC CIP equivalent obligations. (3) Cross-border incident coordination. (4) Real-time monitoring of grid infrastructure. (5) Physical security of generation and distribution assets. (6) Enhanced supply chain scrutiny for critical components.",
         "applicability"),
        ("Requisiti NIS2 per il settore energia",
         "L'energia e' nell'Allegato I (entita essenziale). Specifiche settoriali: (1) IEC 62443 per sistemi OT/SCADA. (2) Obblighi equivalenti NERC CIP. (3) Coordinamento incidenti transfrontaliero. (4) Monitoraggio in tempo reale dell'infrastruttura di rete. (5) Sicurezza fisica degli impianti di generazione e distribuzione. (6) Controllo supply chain rafforzato per componenti critici.",
         "applicability"),

        // === SECTOR: HEALTHCARE ===
        ("NIS2 requirements for healthcare",
         "Healthcare is Annex I (essential entity). Sector-specific: (1) Medical device security (MDR/IVDR intersection). (2) Patient data protection (NIS2+GDPR dual compliance). (3) HL7/FHIR API security. (4) Clinical system availability (life-critical RTO). (5) Interoperability security between facilities. (6) Ransomware resilience for critical care systems.",
         "applicability"),
        ("Requisiti NIS2 per la sanita",
         "La sanita e' nell'Allegato I (entita essenziale). Specifiche settoriali: (1) Sicurezza dispositivi medici (intersezione MDR/IVDR). (2) Protezione dati pazienti (doppia conformita NIS2+GDPR). (3) Sicurezza API HL7/FHIR. (4) Disponibilita sistemi clinici (RTO life-critical). (5) Sicurezza interoperabilita tra strutture. (6) Resilienza ransomware per sistemi di cura critici.",
         "applicability"),

        // === SECTOR: FINANCE ===
        ("NIS2 requirements for financial services",
         "Banking/financial infrastructure is Annex I. Additional: DORA (EU 2022/2554) applies specifically to financial entities with stricter ICT risk management. DORA is lex specialis to NIS2 for finance. Key additions: ICT third-party risk, digital operational resilience testing, threat-led penetration testing (TLPT).",
         "applicability"),
        ("Requisiti NIS2 per i servizi finanziari",
         "Banche/infrastrutture finanziarie sono nell'Allegato I. Aggiuntivo: DORA (UE 2022/2554) si applica specificamente alle entita finanziarie con requisiti di gestione rischio ICT piu stringenti. DORA e' lex specialis rispetto a NIS2 per la finanza. Aggiunte chiave: rischio terze parti ICT, test resilienza operativa digitale, penetration test guidato dalle minacce (TLPT).",
         "applicability"),

        // === SECTOR: TRANSPORT ===
        ("NIS2 requirements for transport",
         "Transport is Annex I (essential entity). Covers: air, rail, water, road transport. Sector-specific: (1) Safety-critical system protection. (2) GPS/GNSS spoofing resilience. (3) ICS/SCADA for traffic management. (4) Cross-border coordination with other EU member states. (5) Passenger data protection overlap with GDPR.",
         "applicability"),
        ("Requisiti NIS2 per i trasporti",
         "I trasporti sono nell'Allegato I (entita essenziale). Copre: aereo, ferroviario, marittimo, stradale. Specifiche settoriali: (1) Protezione sistemi safety-critical. (2) Resilienza a GPS/GNSS spoofing. (3) ICS/SCADA per gestione traffico. (4) Coordinamento transfrontaliero con altri stati UE. (5) Sovrapposizione protezione dati passeggeri con GDPR.",
         "applicability"),

        // === CROSS-BORDER ===
        ("How does cross-border incident reporting work?",
         "Art. 23(3): If an incident affects services in multiple EU member states, the CSIRT of the member state of establishment coordinates. Art. 23(4): CSIRT informs other affected member states' CSIRTs. Art. 13: EU-CyCLONe network coordinates large-scale cross-border incidents.",
         "incident_response"),
        ("Come funziona la notifica incidenti transfrontaliera?",
         "Art. 23(3): Se un incidente impatta servizi in piu stati membri UE, il CSIRT dello stato membro di stabilimento coordina. Art. 23(4): Il CSIRT informa i CSIRT degli altri stati membri interessati. Art. 13: La rete EU-CyCLONe coordina incidenti transfrontalieri su larga scala.",
         "incident_response"),

        // === ISACs ===
        ("What are ISACs and are they required?",
         "Art. 29: NIS2 encourages (but does not mandate) participation in information sharing groups (ISACs). ISACs facilitate threat intelligence exchange between entities in the same sector. Benefits: early warning of sector-specific threats, shared IOCs, collective defense. Recommended for all essential entities.",
         "governance"),
        ("Cosa sono gli ISAC e sono obbligatori?",
         "Art. 29: NIS2 incoraggia (ma non impone) la partecipazione a gruppi di condivisione informazioni (ISAC). Gli ISAC facilitano lo scambio di intelligence sulle minacce tra entita dello stesso settore. Benefici: allarme preventivo su minacce settoriali, IOC condivisi, difesa collettiva. Raccomandato per tutte le entita essenziali.",
         "governance"),

        // === KEY DATES ===
        ("What are the key NIS2 dates and deadlines?",
         "Directive published: 27 Dec 2022. Entry into force: 16 Jan 2023. Transposition deadline: 17 Oct 2024. Entity registration: varies by member state (check national authority). First supervisory reviews: from 2025. Essential entities subject to proactive supervision; important entities ex-post.",
         "legal"),
        ("Quali sono le date chiave di NIS2?",
         "Direttiva pubblicata: 27 dic 2022. Entrata in vigore: 16 gen 2023. Scadenza recepimento: 17 ott 2024. Registrazione entita: varia per stato membro (verificare con autorita nazionale). Prime revisioni di supervisione: dal 2025. Entita essenziali soggette a supervisione proattiva; importanti ex-post.",
         "legal"),

        // === SUPERVISORY POWERS ===
        ("What supervisory powers do authorities have?",
         "Art. 32 (essential): on-site inspections, security audits, evidence requests, access to data/documents, policy implementation evidence. Art. 33 (important): same powers but exercised ex-post (after incident or evidence of non-compliance). Art. 32(5): temporary suspension of certifications and management ban.",
         "sanctions"),
        ("Quali poteri hanno le autorita di supervisione?",
         "Art. 32 (essenziali): ispezioni in loco, audit di sicurezza, richieste di prove, accesso a dati/documenti, evidenze di implementazione policy. Art. 33 (importanti): stessi poteri ma esercitati ex-post (dopo incidente o evidenza di non conformita). Art. 32(5): sospensione temporanea certificazioni e divieto funzioni dirigenziali.",
         "sanctions"),

        // === NOTIFICATION DETAILS ===
        ("What must be included in an Art. 23 notification?",
         "Art. 23(4): Early warning (24h): initial assessment of whether the incident is suspected to be caused by unlawful acts, and cross-border impact. Notification (72h): severity, impact, technical indicators, affected services, mitigation measures. Final report (30 days): root cause, remediation actions, cross-border impact.",
         "incident_response"),
        ("Cosa deve contenere una notifica Art. 23?",
         "Art. 23(4): Preallarme (24h): valutazione iniziale se l'incidente e' sospettato essere causato da atti illeciti, e impatto transfrontaliero. Notifica (72h): severita, impatto, indicatori tecnici, servizi interessati, misure di mitigazione. Relazione finale (30gg): causa radice, azioni di remediation, impatto transfrontaliero.",
         "incident_response"),

        // === PENALTIES DETAIL ===
        ("How are NIS2 fines calculated?",
         "Art. 34(4-5): Essential entities: maximum of 10,000,000 EUR or 2% of total worldwide annual turnover of the preceding fiscal year, whichever is higher. Important entities: maximum of 7,000,000 EUR or 1.4% of turnover. Periodic penalty payments allowed for ongoing non-compliance. Member states define the exact calculation methodology.",
         "sanctions"),
        ("Come vengono calcolate le sanzioni NIS2?",
         "Art. 34(4-5): Entita essenziali: massimo 10.000.000 EUR o 2% del fatturato mondiale annuo totale dell'esercizio precedente, il piu alto. Entita importanti: massimo 7.000.000 EUR o 1,4% del fatturato. Sanzioni periodiche consentite per non conformita continua. Gli stati membri definiscono la metodologia esatta di calcolo.",
         "sanctions"),

        // === ENISA ROLE ===
        ("What is ENISA's role in NIS2?",
         "Art. 6, 14, 24: ENISA maintains the European vulnerability database, provides technical guidance, publishes threat landscape reports, supports CSIRTs, develops certification schemes, and coordinates large-scale incident exercises. ENISA guidelines are authoritative for interpreting Art. 21 requirements.",
         "legal"),
        ("Qual e' il ruolo di ENISA in NIS2?",
         "Art. 6, 14, 24: ENISA gestisce il database europeo delle vulnerabilita, fornisce guida tecnica, pubblica report sul panorama delle minacce, supporta i CSIRT, sviluppa schemi di certificazione e coordina esercitazioni incidenti su larga scala. Le linee guida ENISA sono autorevoli per l'interpretazione dei requisiti Art. 21.",
         "legal"),

        // === NIS1 vs NIS2 ===
        ("What changed from NIS1 to NIS2?",
         "Key changes: (1) Expanded scope: more sectors, size-based threshold (50+/10M+). (2) Harmonized sanctions (10M EUR / 2%). (3) Board personal liability (Art. 20). (4) Stricter notification timelines (24h/72h/30d). (5) Supply chain security mandatory (Art. 21(2)(d)). (6) Stronger supervisory powers. (7) Peer review mechanism between member states.",
         "legal"),
        ("Cosa e' cambiato da NIS1 a NIS2?",
         "Cambiamenti chiave: (1) Ambito ampliato: piu settori, soglia dimensionale (50+/10M+). (2) Sanzioni armonizzate (10M EUR / 2%). (3) Responsabilita personale del CDA (Art. 20). (4) Tempistiche di notifica piu strette (24h/72h/30gg). (5) Sicurezza supply chain obbligatoria (Art. 21(2)(d)). (6) Poteri di supervisione rafforzati. (7) Meccanismo di peer review tra stati membri.",
         "legal"),
    ]
}
