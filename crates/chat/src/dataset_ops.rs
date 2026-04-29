//! Operational security entries — containers, CI/CD, DB, cloud, hardening.
//! Bilingual EN+IT paired entries.

pub fn ops_entries() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        // === LOG RETENTION ===
        ("How long must we retain logs under NIS2?",
         "Art. 21(2)(b) requires incident detection capability. ENISA recommends 12-month minimum log retention. Some national transpositions require longer. Balance with GDPR data minimization. Retain: auth, firewall, DNS, endpoint, email gateway logs.",
         "detection"),
        ("Per quanto tempo conservare i log per NIS2?",
         "Art. 21(2)(b) richiede capacita di rilevamento incidenti. ENISA raccomanda un minimo di 12 mesi di conservazione log. Alcuni recepimenti nazionali richiedono periodi piu lunghi. Bilanciare con la minimizzazione dati GDPR. Conservare: log autenticazione, firewall, DNS, endpoint, email gateway.",
         "detection"),

        // === DATA CLASSIFICATION ===
        ("How to classify data under NIS2",
         "Art. 21(2)(a): Implement a 4-tier scheme: C1 (Public), C2 (Internal), C3 (Confidential), C4 (Restricted). Map security controls to each tier: encryption requirements, access controls, handling procedures, disposal methods. All assets must be classified and labeled.",
         "asset_management"),
        ("Come classificare i dati per NIS2",
         "Art. 21(2)(a): Implementare uno schema a 4 livelli: C1 (Pubblico), C2 (Interno), C3 (Confidenziale), C4 (Riservato). Mappare i controlli di sicurezza per ogni livello: requisiti cifratura, controlli accesso, procedure di trattamento, metodi di smaltimento. Tutti gli asset devono essere classificati ed etichettati.",
         "asset_management"),

        // === SECURE DISPOSAL ===
        ("How to securely dispose of data",
         "Art. 21(2)(a)(h): Secure disposal is part of asset lifecycle. Methods: NIST SP 800-88 guidelines. HDDs: degaussing or physical destruction. SSDs: crypto-erase (ATA Secure Erase). Paper: cross-cut shredding. Certificates of destruction required for audit.",
         "asset_management"),
        ("Come smaltire i dati in modo sicuro",
         "Art. 21(2)(a)(h): Lo smaltimento sicuro fa parte del ciclo di vita degli asset. Metodi: linee guida NIST SP 800-88. HDD: degaussing o distruzione fisica. SSD: crypto-erase (ATA Secure Erase). Carta: distruzione a frammenti. Certificati di distruzione richiesti per audit.",
         "asset_management"),

        // === CIS HARDENING ===
        ("What are CIS Benchmarks?",
         "Art. 21(2)(a)(e): CIS Benchmarks are industry-standard hardening guides for OS, databases, cloud, and applications. Implementing CIS Level 1 baselines satisfies Art. 21(2)(e) system security requirements. Automate compliance checks with tools like OpenSCAP or CIS-CAT.",
         "vulnerability_mgmt"),
        ("Cosa sono i CIS Benchmark?",
         "Art. 21(2)(a)(e): I CIS Benchmark sono guide di hardening standard per OS, database, cloud e applicazioni. Implementare le baseline CIS Livello 1 soddisfa i requisiti di sicurezza dei sistemi Art. 21(2)(e). Automatizzare i controlli di conformita con strumenti come OpenSCAP o CIS-CAT.",
         "vulnerability_mgmt"),

        ("How to harden servers",
         "Art. 21(2)(e): (1) Apply CIS Benchmark for the OS. (2) Disable unnecessary services and ports. (3) Remove default accounts. (4) Enable audit logging. (5) Configure host-based firewall. (6) Enable file integrity monitoring. (7) Apply latest patches. (8) Restrict SSH to key-based auth only.",
         "vulnerability_mgmt"),
        ("Come fare hardening dei server",
         "Art. 21(2)(e): (1) Applicare CIS Benchmark per il sistema operativo. (2) Disabilitare servizi e porte non necessari. (3) Rimuovere account predefiniti. (4) Abilitare logging di audit. (5) Configurare firewall host-based. (6) Abilitare monitoraggio integrita file. (7) Applicare patch aggiornate. (8) Limitare SSH ad autenticazione con chiave.",
         "vulnerability_mgmt"),

        // === DNS SECURITY ===
        ("How to implement DNS security",
         "Art. 21(2)(a)(j): (1) Deploy DNSSEC for domain integrity. (2) Use DNS-over-HTTPS (DoH) or DNS-over-TLS (DoT) for privacy. (3) Monitor DNS queries for C2 beaconing and data exfiltration. (4) Block known malicious domains via DNS filtering. (5) Log all DNS queries for forensics.",
         "network_security"),
        ("Come implementare la sicurezza DNS",
         "Art. 21(2)(a)(j): (1) Implementare DNSSEC per l'integrita del dominio. (2) Usare DNS-over-HTTPS (DoH) o DNS-over-TLS (DoT) per la privacy. (3) Monitorare query DNS per beaconing C2 ed esfiltrazione dati. (4) Bloccare domini malevoli noti tramite filtraggio DNS. (5) Registrare tutte le query DNS per indagini forensi.",
         "network_security"),

        // === CONTAINER SECURITY ===
        ("How to secure containers and Kubernetes",
         "Art. 21(2)(e): (1) Use minimal base images (distroless/Alpine). (2) Scan images for vulnerabilities (Trivy, Grype). (3) Never run as root. (4) Implement network policies for pod isolation. (5) Use admission controllers (OPA/Gatekeeper). (6) Enable audit logging. (7) Rotate secrets via external secrets operator.",
         "development"),
        ("Come proteggere container e Kubernetes",
         "Art. 21(2)(e): (1) Usare immagini base minimali (distroless/Alpine). (2) Scansionare immagini per vulnerabilita (Trivy, Grype). (3) Mai eseguire come root. (4) Implementare network policy per isolamento pod. (5) Usare admission controller (OPA/Gatekeeper). (6) Abilitare audit logging. (7) Ruotare segreti tramite external secrets operator.",
         "development"),

        // === CI/CD SECURITY ===
        ("How to secure CI/CD pipelines",
         "Art. 21(2)(e): (1) Sign all commits (GPG). (2) Require code review approvals. (3) Scan for secrets in code (gitleaks). (4) SAST/DAST in pipeline. (5) Container image scanning. (6) Dependency vulnerability scanning (Dependabot, Renovate). (7) SBOM generation. (8) Immutable build artifacts. (9) Least-privilege service accounts for deploy.",
         "development"),
        ("Come proteggere le pipeline CI/CD",
         "Art. 21(2)(e): (1) Firmare tutti i commit (GPG). (2) Richiedere approvazione code review. (3) Scansionare segreti nel codice (gitleaks). (4) SAST/DAST nella pipeline. (5) Scansione immagini container. (6) Scansione vulnerabilita dipendenze (Dependabot, Renovate). (7) Generazione SBOM. (8) Artefatti di build immutabili. (9) Account di servizio con minimo privilegio per il deploy.",
         "development"),

        // === DATABASE SECURITY ===
        ("How to secure databases",
         "Art. 21(2)(e)(h)(i): (1) Enable TDE (Transparent Data Encryption). (2) Encrypt connections (TLS). (3) Implement row-level security where applicable. (4) Audit all privileged operations. (5) Restrict network access to application tier only. (6) Regular backups with encryption. (7) Separate admin and application accounts.",
         "encryption"),
        ("Come proteggere i database",
         "Art. 21(2)(e)(h)(i): (1) Abilitare TDE (Transparent Data Encryption). (2) Cifrare le connessioni (TLS). (3) Implementare sicurezza a livello di riga se applicabile. (4) Auditare tutte le operazioni privilegiate. (5) Limitare l'accesso di rete al solo tier applicativo. (6) Backup regolari con cifratura. (7) Separare account admin e applicativi.",
         "encryption"),

        // === CLOUD IAM ===
        ("How to implement cloud IAM for NIS2",
         "Art. 21(2)(i)(j): (1) Enforce MFA on all cloud console access. (2) Use IAM roles, not long-lived access keys. (3) Implement least privilege with policy boundaries. (4) Enable CloudTrail/Activity Log for audit. (5) Review access quarterly. (6) Use SSO with corporate IdP. (7) Separate production and development accounts.",
         "access_control"),
        ("Come implementare IAM cloud per NIS2",
         "Art. 21(2)(i)(j): (1) Imporre MFA su tutti gli accessi alla console cloud. (2) Usare ruoli IAM, non chiavi di accesso permanenti. (3) Implementare minimo privilegio con policy boundary. (4) Abilitare CloudTrail/Activity Log per audit. (5) Rivedere accessi trimestralmente. (6) Usare SSO con IdP aziendale. (7) Separare account produzione e sviluppo.",
         "access_control"),

        // === CONFIGURATION MANAGEMENT ===
        ("How to implement configuration management",
         "Art. 21(2)(e): (1) Define golden images for each system type. (2) Use IaC tools (Ansible, Terraform, Puppet). (3) Drift detection with automated remediation. (4) Version control all configurations. (5) Baseline against CIS Benchmarks. (6) Change approval workflow with security review.",
         "vulnerability_mgmt"),
        ("Come implementare il configuration management",
         "Art. 21(2)(e): (1) Definire immagini golden per ogni tipo di sistema. (2) Usare strumenti IaC (Ansible, Terraform, Puppet). (3) Rilevamento drift con remediation automatica. (4) Versionare tutte le configurazioni. (5) Baseline rispetto ai CIS Benchmark. (6) Workflow di approvazione modifiche con revisione sicurezza.",
         "vulnerability_mgmt"),

        // === NETWORK MONITORING ===
        ("How to implement network monitoring",
         "Art. 21(2)(b): (1) Deploy NetFlow/sFlow collection on core switches. (2) IDS/IPS at network boundaries (Suricata, Snort). (3) Full packet capture on critical segments. (4) DNS query logging and analysis. (5) TLS inspection where legally permitted. (6) Automated alerting on anomalies. (7) Integration with SIEM.",
         "detection"),
        ("Come implementare il monitoraggio di rete",
         "Art. 21(2)(b): (1) Implementare raccolta NetFlow/sFlow su switch core. (2) IDS/IPS ai confini di rete (Suricata, Snort). (3) Cattura pacchetti completa su segmenti critici. (4) Logging e analisi query DNS. (5) Ispezione TLS dove legalmente consentito. (6) Alerting automatizzato su anomalie. (7) Integrazione con SIEM.",
         "detection"),

        // === WIRELESS SECURITY ===
        ("How to secure enterprise Wi-Fi",
         "Art. 21(2)(a)(j): (1) WPA3-Enterprise or WPA2-Enterprise with 802.1X. (2) Certificate-based authentication (EAP-TLS). (3) Separate guest network (no access to internal resources). (4) Rogue AP detection. (5) Disable WPS. (6) VLAN segmentation per SSID. (7) Monitor for deauth attacks.",
         "network_security"),
        ("Come proteggere il Wi-Fi aziendale",
         "Art. 21(2)(a)(j): (1) WPA3-Enterprise o WPA2-Enterprise con 802.1X. (2) Autenticazione basata su certificati (EAP-TLS). (3) Rete guest separata (nessun accesso a risorse interne). (4) Rilevamento rogue AP. (5) Disabilitare WPS. (6) Segmentazione VLAN per SSID. (7) Monitorare attacchi di deautenticazione.",
         "network_security"),

        // === ENDPOINT SECURITY ===
        ("What endpoint security is required?",
         "Art. 21(2)(b)(e): (1) Next-gen antivirus/EDR on all endpoints. (2) Host-based firewall enabled. (3) Full-disk encryption (BitLocker/FileVault/LUKS). (4) Application whitelisting for critical systems. (5) USB device control. (6) Automated patching. (7) Screen lock policy (5 min). (8) Remote wipe capability.",
         "detection"),
        ("Quale sicurezza endpoint serve?",
         "Art. 21(2)(b)(e): (1) Antivirus next-gen/EDR su tutti gli endpoint. (2) Firewall host-based abilitato. (3) Cifratura disco completa (BitLocker/FileVault/LUKS). (4) Application whitelisting per sistemi critici. (5) Controllo dispositivi USB. (6) Patching automatizzato. (7) Blocco schermo (5 min). (8) Capacita remote wipe.",
         "detection"),

        // === IDENTITY MANAGEMENT ===
        ("How to implement identity lifecycle management",
         "Art. 21(2)(i): (1) Automated provisioning/deprovisioning via HR system integration. (2) Joiner-mover-leaver process documented. (3) Access review quarterly for all users, monthly for privileged. (4) Disable accounts on same day of termination. (5) Orphan account detection and cleanup.",
         "access_control"),
        ("Come gestire il ciclo di vita delle identita",
         "Art. 21(2)(i): (1) Provisioning/deprovisioning automatizzato tramite integrazione con sistema HR. (2) Processo joiner-mover-leaver documentato. (3) Revisione accessi trimestrale per tutti, mensile per privilegiati. (4) Disabilitare account lo stesso giorno di cessazione. (5) Rilevamento e pulizia account orfani.",
         "access_control"),

        // === AWARENESS METRICS ===
        ("How to measure security awareness effectiveness",
         "Art. 21(2)(f)(g): (1) Track phishing simulation click rates (target: under 5%). (2) Measure training completion rates (target: 100%). (3) Time to report suspicious emails. (4) Number of security incidents caused by human error. (5) Pre/post training knowledge assessments. (6) Report to board quarterly per Art. 20.",
         "email_security"),
        ("Come misurare l'efficacia della formazione sicurezza",
         "Art. 21(2)(f)(g): (1) Tracciare tassi di click simulazioni phishing (obiettivo: sotto 5%). (2) Misurare tassi di completamento formazione (obiettivo: 100%). (3) Tempo di segnalazione email sospette. (4) Numero di incidenti causati da errore umano. (5) Valutazioni pre/post formazione. (6) Report al CDA trimestralmente per Art. 20.",
         "email_security"),

        // === INCIDENT FORENSICS ===
        ("How to preserve forensic evidence",
         "Art. 21(2)(b): (1) Do not power off compromised systems. (2) Capture volatile memory (RAM dump). (3) Create forensic disk image (bit-for-bit copy). (4) Document chain of custody. (5) Use write blockers for disk analysis. (6) Hash all evidence (SHA-256). (7) Store evidence in tamper-proof location. (8) Engage qualified forensic analysts.",
         "incident_response"),
        ("Come preservare le prove forensi",
         "Art. 21(2)(b): (1) Non spegnere i sistemi compromessi. (2) Acquisire memoria volatile (dump RAM). (3) Creare immagine forense del disco (copia bit-for-bit). (4) Documentare la catena di custodia. (5) Usare write blocker per l'analisi disco. (6) Calcolare hash di tutte le prove (SHA-256). (7) Conservare prove in luogo tamper-proof. (8) Coinvolgere analisti forensi qualificati.",
         "incident_response"),

        // === SBOM ===
        ("What is an SBOM and does NIS2 require it?",
         "Art. 21(2)(e): Software Bill of Materials lists all components in software. While NIS2 does not explicitly mandate SBOM, it is a recognized best practice for supply chain security (Art. 21(2)(d)) and vulnerability management. Tools: Syft, CycloneDX, SPDX. Recommended for all custom and third-party software.",
         "supply_chain"),
        ("Cos'e' un SBOM e NIS2 lo richiede?",
         "Art. 21(2)(e): Il Software Bill of Materials elenca tutti i componenti del software. NIS2 non lo impone esplicitamente, ma e' una pratica riconosciuta per la sicurezza della supply chain (Art. 21(2)(d)) e la gestione vulnerabilita. Strumenti: Syft, CycloneDX, SPDX. Raccomandato per tutto il software custom e di terze parti.",
         "supply_chain"),
    ]
}
