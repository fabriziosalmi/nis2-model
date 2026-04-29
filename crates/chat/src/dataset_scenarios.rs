//! Real-world panic scenarios — what people actually type when things go wrong.
//! Bilingual EN+IT paired entries.

pub fn scenario_entries() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        // === BREACH / ATTACK SCENARIOS ===
        ("Our website was defaced",
         "Art. 23 + Art. 21(2)(b): (1) Take the site offline or restore from clean backup. (2) Preserve logs and forensic evidence. (3) Early warning to CSIRT within 24h if the entity is in scope. (4) Check for deeper compromise (web shell, lateral movement). (5) Full notification within 72h.",
         "incident_response"),
        ("Il nostro sito web e' stato defacciato",
         "Art. 23 + Art. 21(2)(b): (1) Mettere il sito offline o ripristinare da backup pulito. (2) Preservare log ed evidenze forensi. (3) Preallarme al CSIRT entro 24h se l'entita e' in ambito. (4) Verificare compromissione piu profonda (web shell, movimento laterale). (5) Notifica completa entro 72h.",
         "incident_response"),

        ("We found malware on a server",
         "Art. 21(2)(b): (1) Isolate the server from the network immediately. (2) Do NOT power off — preserve volatile memory. (3) Capture forensic image. (4) Identify malware type and IOCs. (5) Check lateral movement. (6) If data breach or service impact, notify CSIRT per Art. 23 within 24h.",
         "incident_response"),
        ("Abbiamo trovato malware su un server",
         "Art. 21(2)(b): (1) Isolare immediatamente il server dalla rete. (2) NON spegnere — preservare la memoria volatile. (3) Acquisire immagine forense. (4) Identificare tipo malware e IOC. (5) Verificare movimento laterale. (6) Se data breach o impatto sui servizi, notificare CSIRT entro 24h (Art. 23).",
         "incident_response"),

        ("An employee lost a laptop",
         "Art. 21(2)(a)(h)(i): (1) Remote wipe immediately if MDM is deployed. (2) Revoke all credentials stored on the device. (3) If full-disk encryption was NOT active, treat as data breach. (4) If personal data was on the device, GDPR 72h notification may apply. (5) NIS2 Art. 23 if significant impact.",
         "incident_response"),
        ("Un dipendente ha perso il portatile",
         "Art. 21(2)(a)(h)(i): (1) Remote wipe immediato se MDM e' attivo. (2) Revocare tutte le credenziali salvate sul dispositivo. (3) Se la cifratura disco NON era attiva, trattare come data breach. (4) Se dati personali presenti, notifica GDPR 72h al Garante. (5) NIS2 Art. 23 se impatto significativo.",
         "incident_response"),

        ("Our backup doesn't restore",
         "Art. 21(2)(c): Critical failure. (1) Attempt alternative recovery methods (snapshots, replicas). (2) Engage disaster recovery procedures. (3) If this results in significant service disruption, CSIRT notification required under Art. 23. (4) Post-incident: implement regular backup restore tests (monthly minimum).",
         "business_continuity"),
        ("Il nostro backup non si ripristina",
         "Art. 21(2)(c): Guasto critico. (1) Tentare metodi di recovery alternativi (snapshot, repliche). (2) Attivare procedure di disaster recovery. (3) Se causa interruzione significativa del servizio, notifica CSIRT obbligatoria per Art. 23. (4) Post-incidente: implementare test di ripristino regolari (mensili minimo).",
         "business_continuity"),

        ("We have no logging",
         "Art. 21(2)(b)(f): Major compliance gap. Without logging you cannot detect, investigate, or prove response to incidents. (1) Deploy centralized logging immediately (syslog, ELK, Splunk). (2) Prioritize: auth events, firewall, DNS, endpoint. (3) This alone could trigger supervisory action under Art. 32.",
         "detection"),
        ("Non abbiamo logging",
         "Art. 21(2)(b)(f): Lacuna critica di conformita. Senza logging non si possono rilevare, investigare o dimostrare la gestione degli incidenti. (1) Implementare logging centralizzato immediatamente (syslog, ELK, Splunk). (2) Priorita: eventi autenticazione, firewall, DNS, endpoint. (3) Questo da solo puo attivare azione di supervisione per Art. 32.",
         "detection"),

        ("We use Windows XP on some machines",
         "Art. 21(2)(e): Unsupported operating systems receive no security patches, violating vulnerability management requirements. (1) Isolate these machines on a dedicated network segment. (2) Plan immediate migration or replacement. (3) If isolation is impossible, compensating controls: application whitelisting, no internet access, enhanced monitoring.",
         "vulnerability_mgmt"),
        ("Usiamo ancora Windows XP su alcune macchine",
         "Art. 21(2)(e): Sistemi operativi fuori supporto non ricevono patch di sicurezza, violando i requisiti di gestione vulnerabilita. (1) Isolare queste macchine su un segmento di rete dedicato. (2) Pianificare migrazione o sostituzione immediata. (3) Se l'isolamento e' impossibile: application whitelisting, nessun accesso internet, monitoraggio rafforzato.",
         "vulnerability_mgmt"),

        ("Our CEO uses personal email for work",
         "Art. 21(2)(g)(j): Personal email lacks enterprise security controls (DLP, archiving, e-discovery, phishing protection). (1) Mandate corporate email for all business communication. (2) Implement email security gateway. (3) Board training under Art. 20(2) must cover this. (4) Risk: data exfiltration, compliance evidence gaps.",
         "governance"),
        ("Il nostro CEO usa l'email personale per lavoro",
         "Art. 21(2)(g)(j): L'email personale non ha i controlli di sicurezza aziendali (DLP, archiviazione, protezione phishing). (1) Imporre email aziendale per tutte le comunicazioni lavorative. (2) Implementare gateway di sicurezza email. (3) La formazione del CDA per Art. 20(2) deve coprire questo. (4) Rischio: esfiltrazione dati, lacune nelle prove di conformita.",
         "governance"),

        ("We have no IT department",
         "Art. 21(2): NIS2 obligations apply regardless of internal IT capability. Options: (1) Outsource security operations to a managed SOC/MSSP. (2) Designate an internal security coordinator. (3) Engage a vCISO (virtual CISO) for governance. (4) Use managed services for patching, backup, monitoring. Art. 20(1) still requires board oversight.",
         "governance"),
        ("Non abbiamo un reparto IT",
         "Art. 21(2): Gli obblighi NIS2 si applicano indipendentemente dalle capacita IT interne. Opzioni: (1) Esternalizzare le operazioni di sicurezza a un SOC/MSSP gestito. (2) Designare un coordinatore sicurezza interno. (3) Ingaggiare un vCISO (CISO virtuale) per la governance. (4) Usare servizi gestiti per patching, backup, monitoraggio. Art. 20(1) richiede comunque supervisione del CDA.",
         "governance"),

        ("A competitor was fined under NIS2",
         "Art. 34: Fines are public. (1) Use this as leverage to secure board investment (Art. 20 personal liability). (2) Conduct immediate gap analysis vs Art. 21(2)(a-j). (3) Document your current posture to demonstrate due diligence. (4) Prioritize the areas where the competitor was found non-compliant.",
         "sanctions"),
        ("Un concorrente e' stato multato per NIS2",
         "Art. 34: Le sanzioni sono pubbliche. (1) Usare come leva per ottenere investimento dal CDA (Art. 20 responsabilita personale). (2) Condurre gap analysis immediata vs Art. 21(2)(a-j). (3) Documentare la postura attuale per dimostrare diligenza. (4) Prioritizzare le aree in cui il concorrente e' stato trovato non conforme.",
         "sanctions"),

        ("We received a data subject access request after a breach",
         "Art. 21 + GDPR: (1) Respond to DSAR within GDPR timelines (30 days). (2) For NIS2: if the breach is a significant incident, Art. 23 notification applies independently. (3) Coordinate between DPO (GDPR) and CISO (NIS2). (4) Document all actions for both regulators. (5) Inform affected users per Art. 23(2) if applicable.",
         "legal"),
        ("Abbiamo ricevuto una richiesta di accesso ai dati dopo un breach",
         "Art. 21 + GDPR: (1) Rispondere alla richiesta entro i termini GDPR (30 giorni). (2) Per NIS2: se il breach e' un incidente significativo, la notifica Art. 23 si applica indipendentemente. (3) Coordinare DPO (GDPR) e CISO (NIS2). (4) Documentare tutte le azioni per entrambi i regolatori. (5) Informare gli utenti interessati per Art. 23(2) se applicabile.",
         "legal"),

        // === PRACTICAL DAILY CONCERNS ===
        ("Our passwords are on a shared spreadsheet",
         "Art. 21(2)(i)(h): Critical violation. Credentials in plaintext violate access control and cryptography requirements. (1) Deploy a password manager (Bitwarden, 1Password, KeePass) immediately. (2) Rotate ALL credentials on that spreadsheet. (3) Delete the spreadsheet securely. (4) Implement PAM for privileged accounts.",
         "access_control"),
        ("Le nostre password sono su un foglio Excel condiviso",
         "Art. 21(2)(i)(h): Violazione critica. Credenziali in chiaro violano i requisiti di controllo accessi e crittografia. (1) Implementare un password manager (Bitwarden, 1Password, KeePass) immediatamente. (2) Ruotare TUTTE le credenziali presenti nel foglio. (3) Eliminare il foglio in modo sicuro. (4) Implementare PAM per account privilegiati.",
         "access_control"),

        ("We don't know what servers we have",
         "Art. 21(2)(i): No asset inventory means you cannot implement any other Art. 21 control effectively. (1) Run network discovery (nmap, Lansweeper, Rumble). (2) Catalogue: IP, OS, owner, criticality, data classification. (3) This is the foundation for risk assessment (Art. 21(2)(a)), patching (Art. 21(2)(e)), and access control.",
         "asset_management"),
        ("Non sappiamo quali server abbiamo",
         "Art. 21(2)(i): Senza inventario asset non si puo implementare nessun altro controllo Art. 21 in modo efficace. (1) Eseguire network discovery (nmap, Lansweeper, Rumble). (2) Catalogare: IP, OS, proprietario, criticita, classificazione dati. (3) E' la base per analisi rischi (Art. 21(2)(a)), patching (Art. 21(2)(e)) e controllo accessi.",
         "asset_management"),

        ("Our firewall rules haven't been reviewed in years",
         "Art. 21(2)(a)(f): Stale firewall rules accumulate unnecessary access paths. (1) Export and audit all rules. (2) Remove any-any rules. (3) Validate business justification for each rule. (4) Implement rule review process (quarterly minimum). (5) Enable logging on all rules. Art. 21(2)(f) requires effectiveness assessment.",
         "network_security"),
        ("Le regole del firewall non sono state riviste da anni",
         "Art. 21(2)(a)(f): Regole firewall obsolete accumulano percorsi di accesso non necessari. (1) Esportare e auditare tutte le regole. (2) Rimuovere regole any-any. (3) Validare la giustificazione business per ogni regola. (4) Implementare processo di revisione regole (trimestrale minimo). (5) Abilitare logging su tutte le regole. Art. 21(2)(f) richiede valutazione efficacia.",
         "network_security"),

        ("We store credentials in our source code",
         "Art. 21(2)(e)(h)(i): Secrets in code are a critical vulnerability. (1) Scan entire codebase with tools like gitleaks, trufflehog. (2) Rotate ALL exposed credentials immediately. (3) Implement secrets management (HashiCorp Vault, AWS Secrets Manager). (4) Add pre-commit hooks to block secrets. (5) CI/CD pipeline scanning.",
         "access_control"),
        ("Abbiamo credenziali nel codice sorgente",
         "Art. 21(2)(e)(h)(i): Segreti nel codice sono una vulnerabilita critica. (1) Scansionare tutto il codebase con strumenti come gitleaks, trufflehog. (2) Ruotare TUTTE le credenziali esposte immediatamente. (3) Implementare gestione segreti (HashiCorp Vault, AWS Secrets Manager). (4) Aggiungere pre-commit hook per bloccare segreti. (5) Scansione pipeline CI/CD.",
         "access_control"),

        ("We never did a risk assessment",
         "Art. 21(2)(a): Mandatory obligation. A risk assessment is the foundation of all NIS2 measures. (1) Choose a framework (ISO 27005, NIST CSF). (2) Inventory assets. (3) Identify threats per asset. (4) Evaluate likelihood and impact. (5) Produce a risk treatment plan with priorities. (6) Board approval required (Art. 20).",
         "risk_assessment"),
        ("Non abbiamo mai fatto un'analisi dei rischi",
         "Art. 21(2)(a): Obbligo obbligatorio. L'analisi dei rischi e' la base di tutte le misure NIS2. (1) Scegliere un framework (ISO 27005, NIST CSF). (2) Inventariare gli asset. (3) Identificare le minacce per asset. (4) Valutare probabilita e impatto. (5) Produrre un piano di trattamento rischi con priorita. (6) Approvazione CDA richiesta (Art. 20).",
         "risk_assessment"),

        ("Our third-party SaaS had a data breach",
         "Art. 21(2)(d) + Art. 23: (1) Assess impact on your data/users immediately. (2) Request incident report from the vendor. (3) If your entity's services are affected, Art. 23 notification may apply. (4) Activate supply chain incident procedures. (5) Document for audit trail. (6) Review vendor contract for SLA and breach notification clauses.",
         "supply_chain"),
        ("Il nostro fornitore SaaS ha subito un data breach",
         "Art. 21(2)(d) + Art. 23: (1) Valutare immediatamente l'impatto sui vostri dati/utenti. (2) Richiedere report dell'incidente al fornitore. (3) Se i servizi della vostra entita sono impattati, la notifica Art. 23 si applica. (4) Attivare procedure di incidente supply chain. (5) Documentare per la traccia di audit. (6) Rivedere il contratto per SLA e clausole di notifica breach.",
         "supply_chain"),
    ]
}
