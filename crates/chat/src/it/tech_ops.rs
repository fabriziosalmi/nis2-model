pub fn entries() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "Come implementare un SIEM",
            "Art. 21(2)(b): Raccogliere log da: firewall, endpoint, AD/IAM, DNS, proxy, email gateway, cloud. Definire regole di correlazione per: brute force, movimento laterale, esfiltrazione dati, escalation privilegi.",
            "detection_impl"
        ),
        (
            "Quali log raccogliere",
            "Art. 21(2)(b): Minimo: eventi autenticazione, modifiche privilegi, firewall allow/deny, query DNS, email gateway, connessioni VPN, accesso file sensibili, API cloud, creazione processi endpoint. Conservare 12+ mesi.",
            "detection_impl"
        ),
        (
            "Come implementare EDR",
            "Art. 21(2)(b)(e): Installare agente su tutti gli endpoint. Abilitare: monitoraggio processi, integrita file, connessioni di rete, analisi memoria. Regole per: ransomware, credential dumping, living-off-the-land. Integrare con SIEM.",
            "detection_impl"
        ),
        (
            "Quali strumenti open-source posso usare per un SIEM/SOC interno?",
            "Se non puoi esternalizzare a un SOC 24/7, puoi implementare soluzioni open-source o gratuite per il log management e il rilevamento (Art. 21(2)(f)).

**Stack consigliato:**
- **Wazuh:** Eccellente piattaforma open-source XDR/SIEM. Offre agenti per Windows/Linux/macOS, rileva malware, controlla la compliance e offre FIM (File Integrity Monitoring).
- **Elastic Stack (ELK):** Per la raccolta massiva di log di firewall e Active Directory.
- **TheHive / Cortex:** Piattaforme per l'Incident Response e l'automazione dei playbook.",
            "detection_impl"
        ),
        (
            "Come implementare la rotazione password",
            "Art. 21(2)(i): Implementare tramite GPO di Active Directory o policy IAM. Scadenza password massima (90 giorni raccomandati), cronologia (24 password), lunghezza minima (14+ caratteri), requisiti di complessita. Integrare con PAM per account privilegiati.",
            "access_control_impl"
        ),
        (
            "Cos'e' una soluzione PAM",
            "Privileged Access Management implementa Art. 21(2)(i). Funzionalita: vaulting credenziali, registrazione sessioni, accesso just-in-time, workflow approvazione, rotazione automatica account di servizio. Esempi: CyberArk, BeyondTrust, Delinea.",
            "access_control_impl"
        ),
        (
            "Come implementare MFA",
            "Art. 21(2)(j): MFA tramite app TOTP (Google/Microsoft Authenticator), chiavi hardware (YubiKey/FIDO2), o notifiche push. Priorita: account admin prima, poi VPN, poi tutti gli utenti. Evitare MFA via SMS (rischio SIM swap).",
            "access_control_impl"
        ),
        (
            "Cos'e' RBAC",
            "Role-Based Access Control implementa Art. 21(2)(i). Definire ruoli per funzione lavorativa, assegnare permessi minimi per ruolo. Revisione trimestrale. Separazione compiti per operazioni critiche (maker-checker).",
            "access_control_impl"
        ),
        (
            "Come gestire gli account di servizio",
            "Art. 21(2)(i): Inventariare tutti gli account di servizio. Credenziali in vault (non nel codice/config). Rotazione automatica (30-90 giorni). Monitorare uso anomalo. Assegnare proprietari individuali. Disabilitare quando dismessi.",
            "access_control_impl"
        ),
        (
            "L'MFA è obbligatoria ovunque? Che soluzioni consigliate?",
            "L'uso dell'autenticazione a più fattori è menzionato esplicitamente come obbligo tecnico di base.

<span class='ans-header'><span class='ans-art'>Art. 21(2)(j)</span> <span class='ans-intro'>MFA (Multi-Factor Authentication)</span></span>
<span class='ans-body'>Va applicata ovunque sia possibile, dando priorità ad accessi remoti, VPN e account amministrativi.</span>

**Soluzioni Consigliate:**
- Se usi Microsoft 365 / Entra ID (Azure AD), attiva subito le **Security Defaults** o le regole di Conditional Access per imporre l'MFA.
- Per le VPN (Fortinet, Cisco, OpenVPN), integra RADIUS o SAML con Google Workspace o Microsoft Authenticator.
- **Passkeys (FIDO2):** Per gli utenti privilegiati, adotta chiavette hardware (YubiKey) resistenti al phishing (standard FIDO2/WebAuthn). E' lo standard d'oro attuale.",
            "access_control_impl"
        ),
        (
            "Come implementare cifratura a riposo",
            "Art. 21(2)(h): Usare AES-256 per dati a riposo. Abilitare LUKS/BitLocker per cifratura disco. TDE per database. Cifrare backup prima del trasferimento offsite. Gestire chiavi in HSM o KMS, mai insieme ai dati.",
            "encryption_impl"
        ),
        (
            "Come implementare TLS",
            "Art. 21(2)(h)(j): TLS 1.2 minimo, preferire TLS 1.3. Disabilitare SSLv3, TLS 1.0, 1.1. Cipher suite forti (ECDHE, AES-GCM). Rinnovo automatico certificati (Let's Encrypt/ACME). Header HSTS. Monitorare scadenza certificati.",
            "encryption_impl"
        ),
        (
            "Cos'e' un HSM",
            "Hardware Security Module: storage tamper-resistant per chiavi crittografiche per Art. 21(2)(h). Usare per: chiavi private CA, master key cifratura database, code signing. Alternative cloud: AWS CloudHSM, Azure Dedicated HSM, GCP Cloud HSM.",
            "encryption_impl"
        ),
        (
            "Come gestire le chiavi crittografiche",
            "Art. 21(2)(h): Ciclo di vita chiavi: generazione (CSPRNG), distribuzione (canali cifrati), storage (HSM/KMS), rotazione (annuale minimo), revoca, distruzione. Separare gestione chiavi da gestione dati.",
            "encryption_impl"
        ),
        (
            "Cos'è la crittografia E2EE e quando è obbligatoria?",
            "L'Art. 21(2)(h) incoraggia l'uso della crittografia, inclusa quella end-to-end (E2EE), dove opportuno.

E2EE significa che solo il mittente e il destinatario hanno le chiavi; il server intermedio non può leggere i dati (es. Signal, WhatsApp, ProtonMail).
**Dove usarla in azienda:**
- Per le comunicazioni dei dirigenti apicali o team di R&D su progetti segreti.
- Nei canali di comunicazione di emergenza (out-of-band) da usare quando la rete aziendale (Exchange/Teams) è compromessa da un ransomware.",
            "encryption_impl"
        ),
        (
            "Come valutare la sicurezza dei fornitori",
            "Art. 21(2)(d): Questionario standardizzato (SIG/CAIQ). Richiedere SOC 2 Type II o ISO 27001. SLA di sicurezza nei contratti. Clausola right-to-audit. Rivalutazione annuale. Monitorare breach fornitori tramite threat intelligence.",
            "supply_chain_impl"
        ),
        (
            "Quali clausole di sicurezza nei contratti",
            "Art. 21(2)(d): Includere: trattamento dati, notifica breach (<24h), diritto di audit, approvazione subappaltatori, requisiti cifratura, residenza dati, coordinamento incidenti, restituzione/distruzione dati a fine contratto.",
            "supply_chain_impl"
        ),
        (
            "Come monitorare il rischio terze parti",
            "Art. 21(2)(d): Monitoraggio continuo: rating di sicurezza (BitSight, SecurityScorecard), alert breach fornitori, dark web monitoring. Classificare fornitori per criticita. Critici: revisione trimestrale. Altri: annuale.",
            "supply_chain_impl"
        ),
        (
            "Come gestisco la sicurezza dei miei fornitori senza impazzire?",
            "La Supply Chain è il vettore di attacco principale degli ultimi anni e il focus centrale della NIS2.

<span class='ans-header'><span class='ans-art'>Art. 21(2)(d)</span> <span class='ans-intro'>Sicurezza della Supply Chain</span></span>
<span class='ans-body'>Non puoi controllare direttamente l'IT dei fornitori, ma puoi governare il rischio.</span>

**Azioni Pratiche per Iniziare:**
1. **Mappatura:** Crea un elenco dei fornitori critici (quelli che hanno accesso ai tuoi dati o sistemi).
2. **Questionari di Sicurezza:** Invia un form standardizzato (es. CAIQ del Cloud Security Alliance) per valutare la loro postura.
3. **Clausole Contrattuali:** Inserisci nei contratti il diritto di audit (Right to Audit) e obblighi di notifica in caso di incidente entro 12 ore (per permetterti di rispettare le tue 24h).
4. **Accessi:** Limita l'accesso remoto dei fornitori ai soli sistemi necessari, tramite VPN dedicate e obbligo di MFA.",
            "supply_chain_impl"
        ),
        (
            "Cosa faccio se un mio piccolo fornitore si rifiuta di adeguarsi ai miei requisiti di sicurezza NIS2?",
            "Questo è il problema centrale della gestione della supply chain. L'azienda più grande deve far ricadere i requisiti a cascata.

**Soluzioni per il dipartimento acquisti:**
1. **Gradualità:** Richiedi standard minimi iniziali (MFA obbligatoria per le email, backup, antivirus) e dai 12 mesi per adeguarsi.
2. **Supporto:** Metti a disposizione dei fornitori più piccoli (che non hanno budget IT) dei template gratuiti o piattaforme condivise in cloud.
3. **Sostituzione:** Se il fornitore ha un ruolo critico (es. sviluppa il software core) e rifiuta di garantire la sicurezza, sarai costretto a cambiare fornitore, altrimenti la responsabilità di un suo breach ricadrà su di te.",
            "supply_chain_impl"
        ),
        (
            "Serve auditare i fornitori?",
            "Art. 21(2)(d) richiede sicurezza della supply chain. Include: valutazioni di sicurezza dei fornitori, clausole contrattuali, SLA, monitoraggio rischi e coordinamento incidenti con fornitori diretti.",
            "supply_chain"
        ),
        (
            "I nostri fornitori cloud sono conformi?",
            "Art. 21(2)(d): la sicurezza della supply chain e' un obbligo. Per fornitori cloud verificare: certificazioni (ISO 27001, SOC 2), clausole contrattuali di sicurezza, localizzazione dati, diritto di audit.",
            "supply_chain"
        ),
        (
            "Un nostro fornitore e' stato hackerato",
            "Art. 21(2)(d) supply chain + Art. 23: (1) Valutare immediatamente l'impatto sui vostri dati/sistemi. (2) Isolare le connessioni con il fornitore. (3) Se impatto significativo, notifica al CSIRT (24h). (4) Richiedere al fornitore il report dell'incidente. (5) Rivalutare il rischio fornitore.",
            "supply_chain"
        ),
        (
            "Come valutare la sicurezza dei fornitori?",
            "Art. 21(2)(d): questionari di sicurezza, certificazioni (ISO 27001, SOC 2), audit on-site, SLA con clausole di sicurezza, diritto di audit, notifica incidenti, revisione periodica del rischio fornitore.",
            "supply_chain"
        ),
        (
            "Servono clausole di sicurezza nei contratti?",
            "Art. 21(2)(d): i contratti con fornitori devono includere: requisiti di sicurezza, obblighi di notifica incidenti, diritto di audit, penali per non conformita, SLA di patching, gestione subappaltatori.",
            "supply_chain"
        ),
        (
            "Come gestire il rischio dei subappaltatori?",
            "Art. 21(2)(d): il rischio della catena di fornitura include i subappaltatori. Richiedere ai fornitori diretti di applicare gli stessi standard di sicurezza ai loro subfornitori. Clausola di flusso verso il basso.",
            "supply_chain"
        ),
        (
            "I fornitori cloud rientrano nella NIS2?",
            "I provider cloud (IaaS, PaaS, SaaS) sono in Allegato I come infrastruttura digitale. Inoltre, Art. 21(2)(d) richiede di gestire il rischio della catena di fornitura inclusi i provider cloud.",
            "supply_chain"
        ),
        (
            "Come monitorare i fornitori nel tempo?",
            "Art. 21(2)(d): non basta la valutazione iniziale. Serve monitoraggio continuo: revisione annuale, monitoraggio breach pubblici, aggiornamento risk scoring, verifica certificazioni, esercitazioni congiunte.",
            "supply_chain"
        ),
        (
            "Serve un registro dei fornitori critici?",
            "Art. 21(2)(d) e Art. 21(2)(i) asset management: identificare fornitori critici (dipendenza operativa, accesso a dati/sistemi), classificarli per rischio, documentare misure di mitigazione per ciascuno.",
            "supply_chain"
        ),
        (
            "Come implementare la sicurezza della catena di fornitura per NIS2?",
            "Art. 21(2)(d) richiede un programma strutturato: (1) Classificazione fornitori — identificare fornitori critici (accesso a sistemi/dati, dipendenza operativa). Tier 1: accesso diretto. Tier 2: servizi essenziali. Tier 3: altri. (2) Due diligence — questionario di sicurezza, verifica certificazioni (ISO 27001, SOC 2), analisi bilancio per stabilita finanziaria, controllo breach storici. (3) Contratti — clausole obbligatorie: requisiti di sicurezza specifici, obbligo di notifica incidenti entro 24h, diritto di audit, SLA per patching critico, gestione subappaltatori, penali per non conformita, piano di exit. (4) Monitoraggio continuo — revisione annuale del risk scoring, monitoraggio breach pubblici (have i been pwned, press), verifica mantenimento certificazioni, esercitazioni congiunte di risposta incidenti. (5) SBOM — richiedere Software Bill of Materials per software acquistato. Monitorare vulnerabilita nelle dipendenze open source.",
            "supply_chain"
        ),
        (
            "Cosa prevede la regola 3-2-1 per i backup in ottica NIS2?",
            "La regola 3-2-1 è la best practice raccomandata per garantire la business continuity (Art. 21).

<span class='ans-header'><span class='ans-art'>Art. 21(2)(c)</span> <span class='ans-intro'>Regola 3-2-1 Backup</span></span>
<span class='ans-body'>Per proteggersi dai ransomware, l'architettura di backup deve rispettare:</span>
<div class='ans-steps'>
  <div class='ans-step'><div class='step-num'>3</div><div class='step-text'>Mantenere **tre copie** dei dati (una in produzione, due di backup).</div></div>
  <div class='ans-step'><div class='step-num'>2</div><div class='step-text'>Utilizzare **due supporti** di archiviazione differenti (es. NAS locale e Object Storage in cloud).</div></div>
  <div class='ans-step'><div class='step-num'>1</div><div class='step-text'>Conservare almeno **una copia off-site** o offline (air-gapped / immutabile).</div></div>
</div>

**Soluzione Tecnica Semplice:** Configurare i backup su un NAS locale con replica automatica su AWS S3 con Object Lock abilitato (immutabilità), in modo che i dati non possano essere criptati da un ransomware.",
            "business_continuity_impl"
        ),
        (
            "Come posso testare i backup in modo efficace?",
            "I backup sono inutili se non si possono ripristinare. La Direttiva impone test regolari.

<span class='ans-header'><span class='ans-art'>Art. 21(2)(c)</span> <span class='ans-intro'>Test di Ripristino</span></span>
<span class='ans-body'>Non basta avere i log verdi sui job di backup. Occorre documentare la capacità di ripristino.</span>

**Best Practice:**
1. Definire RTO (Recovery Time Objective) e RPO (Recovery Point Objective).
2. Schedulare un test di ripristino bare-metal trimestrale.
3. Utilizzare funzionalità come 'SureBackup' (Veeam) o test automatizzati su ambienti sandbox isolati.

**Riferimenti:** NIST SP 800-34 Rev. 1 (Contingency Planning Guide for Federal Information Systems).",
            "business_continuity_impl"
        ),
        (
            "Cos'è un Playbook di Incident Response e da dove posso partire?",
            "L'Art. 21 richiede procedure per la gestione degli incidenti. Il playbook è il manuale operativo.

<span class='ans-header'><span class='ans-art'>Art. 21(2)(a)</span> <span class='ans-intro'>Playbook Operativi</span></span>
<span class='ans-body'>Il playbook definisce chi fa cosa, quando e come durante una crisi.</span>

**Soluzione Reale e Risorse:**
Non è necessario scriverlo da zero. Sfrutta risorse open-source standard di settore:
1. Scarica i playbook gratuiti del **SANS Institute** (es. Ransomware, Data Breach).
2. Adatta il playbook del **NIST SP 800-61 Rev. 2** (Computer Security Incident Handling Guide).
3. Definisci chiaramente: ruoli (Incident Commander), canali di comunicazione di emergenza (Out-of-band), e procedure di isolamento (es. staccare la rete dal dominio).",
            "incident_response_impl"
        ),
        (
            "Cosa vuol dire 'Notifica di preallarme entro 24 ore' esattamente? Da quando parte il timer?",
            "I tempi di notifica della NIS2 sono draconiani.

<span class='ans-header'><span class='ans-art'>Art. 23</span> <span class='ans-intro'>La regola del 24-72-30</span></span>
<span class='ans-body'>Il timer parte dal momento in cui l'azienda *viene a conoscenza* (awareness) dell'incidente significativo.</span>

- **Preallarme (24h):** Non devi avere la perizia forense. Devi solo dire allo CSIRT nazionale: 'Siamo sotto attacco, sospettiamo un ransomware, stiamo investigando'.
- **Notifica Incidente (72h):** Un rapporto più strutturato con dettagli sull'impatto e gli indicatori di compromissione (IoC).
- **Rapporto Finale (1 mese):** Un'analisi post-mortem: cause alla radice (Root Cause Analysis), impatto completo, danni stimati e le misure strutturali prese per evitare che succeda di nuovo.",
            "incident_response_impl"
        ),
        (
            "Cos'è la Root Cause Analysis (Analisi delle Cause alla Radice) in caso di incidente?",
            "È una richiesta obbligatoria per il Report Finale (a 1 mese dall'incidente).

Non basta ripristinare i server e ripartire (remediation). L'azienda deve scoprire *come* l'attaccante è entrato.
**Esempi di Root Cause:**
- Un dipendente ha scaricato un malware da una mail (Vettore: Email Phishing).
- C'era una vulnerabilità RCE sul server VPN non patchato (Vettore: Exploit perimetrale).
- Password di un utente VPN indovinata e nessuna MFA attiva (Vettore: Credential Stuffing).
Spesso, per eseguire questa analisi, è necessario coinvolgere aziende esterne esperte in Digital Forensics and Incident Response (DFIR).",
            "incident_response_impl"
        ),
        (
            "Come faccio a fare un inventario degli asset IT se non ho budget per tool costosi?",
            "La sicurezza inizia dalla visibilità. Non puoi proteggere ciò che non sai di avere.

<span class='ans-header'><span class='ans-art'>Art. 21(2)(e)</span> <span class='ans-intro'>Gestione degli Asset IT</span></span>
<span class='ans-body'>L'inventario deve coprire hardware, software, servizi cloud e dati.</span>

**Soluzioni Tecniche Economiche (Open Source):**
- **Snipe-IT:** Eccellente piattaforma gratuita per l'IT Asset Management (hardware/software/licenze).
- **GLPI:** Strumento potente per Helpdesk e ITAM con funzionalità di auto-discovery (tramite agent FusionInventory).
- **Lansweeper:** (Versione freeware fino a 100 asset) per la scansione automatizzata della rete.

L'importante è avere un registro costantemente aggiornato e associarvi un livello di criticità.",
            "asset_management_impl"
        ),
        (
            "Cosa si intende per gestione delle vulnerabilità in pratica?",
            "Non è solo applicare le patch di Windows, ma avere un processo sistematico.

<span class='ans-header'><span class='ans-art'>Art. 21(2)(e)</span> <span class='ans-intro'>Gestione Vulnerabilità</span></span>
<span class='ans-body'>Bisogna identificare proattivamente i punti deboli prima che lo facciano gli attaccanti.</span>

**Processo Operativo (Soluzioni Reali):**
1. **Scansione Attiva:** Utilizza tool come Nessus, Qualys o soluzioni Open Source come **OpenVAS / Greenbone** per scansionare gli IP pubblici e interni regolarmente.
2. **Prioritizzazione:** Correggi prima le vulnerabilità con punteggio CVSS alto/critico esposte su Internet (es. KEV - Known Exploited Vulnerabilities del CISA).
3. **Patch Management:** Automatizza l'installazione delle patch per OS e software di terze parti (tramite WSUS, Intune o PDQ Deploy).",
            "vulnerability_mgmt_impl"
        ),
        (
            "Devo fare Penetration Test o basta un Vulnerability Assessment?",
            "L'Art. 21 impone la 'valutazione dell'efficacia delle misure di gestione dei rischi'.

Un Vulnerability Assessment (VA) è automatizzato e individua i problemi (es. tramite Nessus). Va fatto regolarmente (es. mensilmente).
Un Penetration Test (PT) è manuale: un ethical hacker prova a sfruttare i problemi per vedere fin dove arriva.

**Per la conformità NIS2:**
Un VA mensile/trimestrale e un PT annuale sull'infrastruttura esterna e interna (Active Directory) sono considerati lo standard d'oro (e richiesti dalla maggior parte delle certificazioni ISO 27001).",
            "vulnerability_mgmt_impl"
        ),
        (
            "Cosa significa implementare la 'Microsegmentazione' o un'architettura Zero Trust per la NIS2?",
            "La direttiva richiede politiche di sicurezza delle reti avanzate (Art. 21(2)(e)).

**Zero Trust in pratica:**
- Mai fidarsi di un dispositivo solo perché è connesso alla rete Wi-Fi dell'ufficio.
- Ogni richiesta di accesso a un server o applicativo deve essere verificata (Identità + Stato del dispositivo).
**Microsegmentazione:**
- Usare VLAN o firewall software (es. NSX, Illumio, o regole host-based) per fare in modo che i server non possano parlarsi se non strettamente necessario (es. il server HR non deve comunicare col server Produzione).",
            "network_security_impl"
        ),
        (
            "Endpoint Detection and Response (EDR) è obbligatorio? Basta l'antivirus?",
            "Il tradizionale antivirus (che lavora a firme) è oggi considerato insufficiente contro attacchi mirati o ransomware zero-day.

La NIS2 non nomina esplicitamente l'EDR, ma richiede 'sicurezza delle reti e dei sistemi informativi' allo stato dell'arte.
**Perché serve un EDR (es. CrowdStrike, Microsoft Defender for Endpoint, SentinelOne):**
- Monitora i comportamenti (es. un file word che lancia PowerShell).
- Permette di isolare con un clic un computer infetto dalla rete aziendale prima che il ransomware si propaghi lateralmente. È vitale per soddisfare i tempi strettissimi di risposta agli incidenti.",
            "endpoint_security_impl"
        ),
        (
            "Sviluppiamo software custom internamente. La NIS2 ci impone regole su questo?",
            "Assolutamente sì. L'Art. 21(2)(i) menziona 'pratiche di base di igiene informatica e formazione nel campo della cibersicurezza'. La sicurezza nel ciclo di vita dello sviluppo (SDLC) è implicita.

**Pratiche di Sviluppo Sicuro (DevSecOps):**
- **Analisi Statica (SAST):** Scansionare il codice sorgente per trovare vulnerabilità (es. SQL Injection) prima del rilascio (usando tool come SonarQube).
- **Analisi delle Dipendenze (SCA):** Controllare le librerie open-source utilizzate nel progetto per capire se hanno CVE note (es. tramite Dependabot o OWASP Dependency-Check).
- **Gestione Segreti:** Non hardcodare MAI password o chiavi API nel codice sorgente (usare Azure Key Vault o HashiCorp Vault).",
            "development_impl"
        ),
    ]
}
