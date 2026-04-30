pub fn entries() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "Serve un SIEM per NIS2?",
            "Art. 21(2)(b) richiede gestione degli incidenti, che implica capacita di rilevamento. Un SIEM o sistema equivalente di correlazione log e' lo standard per rilevare e investigare incidenti.",
            "detection"
        ),
        (
            "Serve un SOC?",
            "NIS2 non impone un SOC per nome, ma Art. 21(2)(b) richiede capacita di rilevamento e gestione incidenti. Un SOC (interno o esterno) e' il modo comune per soddisfare questo obbligo.",
            "detection"
        ),
        (
            "Servono log centralizzati?",
            "Art. 21(2)(b) gestione incidenti e Art. 21(2)(f) valutazione efficacia richiedono tracce di audit. La centralizzazione dei log e' la base per rilevamento, forensics e prove di conformita.",
            "detection"
        ),
        (
            "Serve un EDR?",
            "Art. 21(2)(b) richiede gestione incidenti incluso il rilevamento. L'EDR e' una misura riconosciuta per la visibilita degli endpoint. Art. 21(2)(e) copre anche la gestione vulnerabilita sugli endpoint.",
            "detection"
        ),
        (
            "Come implementare il monitoraggio 24/7?",
            "Art. 21(2)(b): per entita essenziali, monitoraggio continuo e' atteso. Opzioni: SOC interno, SOC esterno (MSSP), modello ibrido. Minimo: alerting automatico + reperibilita analista. SIEM + SOAR per automazione.",
            "detection"
        ),
        (
            "Quanto conservare i log di sicurezza?",
            "Art. 21(2)(b): conservazione minima raccomandata 12 mesi online, 24 mesi in archivio. Per settori regolamentati (finanza, sanita) verificare requisiti aggiuntivi. Log immutabili per integrita forense.",
            "detection"
        ),
        (
            "Come implementare un sistema di rilevamento efficace per NIS2?",
            "Art. 21(2)(b) richiede capacita di rilevamento e gestione incidenti: (1) SIEM — piattaforma centralizzata per raccolta, correlazione e analisi log. Fonti minime: firewall, endpoint (EDR), Active Directory, DNS, email gateway, VPN, cloud (AWS CloudTrail/Azure Activity Log). Regole di correlazione per: brute force, movement laterale, esfiltrazione dati, escalation privilegi, accesso anomalo. (2) EDR — agente su tutti gli endpoint (server e workstation). Capacita: monitoraggio processi, integrita file, connessioni di rete, analisi comportamentale. Regole per: ransomware (cifratura massiva), credential dumping, living-off-the-land (PowerShell anomalo). (3) NDR — analisi traffico di rete per minacce che bypassano endpoint. Rilevamento: beaconing C2, tunnel DNS, esfiltrazione via protocolli legittimi. (4) Threat hunting — attivita proattiva settimanale: cercare IoC da feed di threat intelligence, analizzare anomalie nei log, verificare baseline. (5) Metriche — MTTD (tempo rilevamento) target: < 24h per incidenti critici. False positive rate: < 5%. Copertura log: 100% sistemi critici. Conservazione: 12 mesi online, 24 mesi archivio.",
            "detection"
        ),
        (
            "Serve un inventario degli asset?",
            "Art. 21(2)(i) richiede gestione degli asset. Non si puo proteggere cio che non si conosce. Un inventario completo di hardware, software, dati e asset di rete e' fondamentale.",
            "asset_management"
        ),
        (
            "Serve classificare i dati?",
            "Art. 21(2)(a) analisi dei rischi richiede comprensione di quali dati si possiedono e la loro criticita. La classificazione dei dati consente misure di sicurezza proporzionate.",
            "asset_management"
        ),
        (
            "NIS2 si applica ai dispositivi IoT?",
            "Art. 21(2)(e) copre sicurezza nell'acquisizione e manutenzione dei sistemi, inclusi IoT. Misure: inventario dispositivi, segmentazione rete dedicata, aggiornamenti firmware, credenziali uniche, monitoraggio traffico anomalo.",
            "asset_management"
        ),
        (
            "NIS2 richiede sviluppo software sicuro?",
            "Art. 21(2)(e) richiede esplicitamente sicurezza nell'acquisizione, sviluppo e manutenzione dei sistemi. Copre SDLC sicuro, code review e gestione vulnerabilita nel software custom.",
            "development"
        ),
        (
            "Servono code review?",
            "Art. 21(2)(e) copre sicurezza nello sviluppo e manutenzione. Code review, analisi statica e test di sicurezza sono pratiche riconosciute sotto questo obbligo.",
            "development"
        ),
        (
            "Come proteggere le API?",
            "Art. 21(2)(e): le API fanno parte della superficie di attacco. Misure: autenticazione (OAuth2/mTLS), rate limiting, validazione input, cifratura TLS, logging accessi, WAF, test OWASP API Security Top 10.",
            "development"
        ),
        (
            "Come implementare lo sviluppo sicuro?",
            "Art. 21(2)(e): SDLC sicuro con: threat modeling in design, code review, SAST/DAST in CI/CD, gestione dipendenze (SCA), test di sicurezza pre-rilascio, hardening configurazioni, vulnerability management.",
            "development"
        ),
        (
            "Servono code review di sicurezza?",
            "Art. 21(2)(e): si. Review del codice per vulnerabilita (OWASP Top 10), analisi statica automatizzata (SAST), peer review per codice critico. Integrazione in pipeline CI/CD per feedback rapido.",
            "development"
        ),
        (
            "Serve un penetration test?",
            "Art. 21(2)(e) richiede gestione vulnerabilita e Art. 21(2)(f) valutazione efficacia delle misure. Il penetration test e' un metodo standard per entrambi. Annuale minimo.",
            "vulnerability_mgmt"
        ),
        (
            "Serve fare patching?",
            "Art. 21(2)(e) richiede gestione vulnerabilita. Il patching tempestivo delle vulnerabilita note e' un requisito fondamentale. Prioritizzare per severita CVSS. SLA: critico 48h, alto 7gg, medio 30gg.",
            "vulnerability_mgmt"
        ),
        (
            "Serve la scansione vulnerabilita?",
            "Art. 21(2)(e): scansione vulnerabilita regolare. Settimanale per sistemi internet-facing, mensile per interni. Test di penetrazione annuale. Tracciare metriche di tempo medio di remediation.",
            "vulnerability_mgmt"
        ),
        (
            "Serve una policy di vulnerability disclosure?",
            "Art. 21(2)(e) copre la divulgazione coordinata delle vulnerabilita. Le entita dovrebbero avere un processo per ricevere, valutare e rimediare le vulnerabilita segnalate nei propri prodotti e servizi.",
            "vulnerability_mgmt"
        ),
        (
            "Ogni quanto fare vulnerability scanning?",
            "Art. 21(2)(e): settimanale per sistemi esposti a internet, mensile per rete interna. Scan dopo ogni cambio significativo. Prioritizzare rimedio per CVSS: critico entro 48h, alto entro 7gg, medio entro 30gg.",
            "vulnerability_mgmt"
        ),
        (
            "Serve un programma di bug bounty?",
            "Art. 21(2)(e) copre la divulgazione coordinata delle vulnerabilita. Un programma di bug bounty e' un'implementazione avanzata. Per la maggior parte delle entita, una policy di vulnerability disclosure e' sufficiente.",
            "vulnerability_mgmt"
        ),
        (
            "Come prioritizzare le patch?",
            "Art. 21(2)(e): (1) CVSS score come base, (2) esposizione (internet-facing = urgente), (3) criticita asset, (4) exploit disponibili (KEV CISA). Critico: 48h. Alto: 7gg. Medio: 30gg. Basso: 90gg.",
            "vulnerability_mgmt"
        ),
        (
            "Come strutturare un programma di gestione vulnerabilita per NIS2?",
            "Art. 21(2)(e) richiede gestione vulnerabilita e Art. 21(2)(f) valutazione efficacia: (1) Discovery — inventario completo di tutti gli asset: server, endpoint, dispositivi di rete, IoT, cloud, container. Aggiornamento automatico. (2) Scanning — settimanale per sistemi esposti a internet, mensile per rete interna, dopo ogni cambio significativo. Scanner autenticati per visibilita completa. (3) Prioritizzazione — CVSS come base, poi fattori contestuali: esposizione internet (+), criticita asset (+), exploit disponibile (++), nel catalogo KEV CISA (+++). SLA di rimedio: critico 48h, alto 7gg, medio 30gg, basso 90gg. (4) Remediation — processo strutturato: assegnazione al team responsabile, tracking in sistema di ticketing, verifica post-patch, documentazione. Per sistemi legacy senza patch: virtual patching via WAF/IPS, segmentazione, compensating controls. (5) Pentest — annuale minimo da team esterno qualificato. Ambito: rete esterna, rete interna, applicazioni web/mobile, social engineering. Red team per entita essenziali. (6) Disclosure — policy pubblica per ricevere segnalazioni di vulnerabilita. Processo di triage, comunicazione con il ricercatore, timeline di fix.",
            "vulnerability_mgmt"
        ),
        (
            "Come costruire un piano di risposta incidenti",
            "Art. 21(2)(b): Definire: (1) criteri classificazione (P1-P4), (2) matrice escalation, (3) template comunicazione, (4) procedure contenimento per tipo attacco, (5) preservazione prove, (6) revisione post-incidente. Test tramite tabletop trimestrali.",
            "incident_impl"
        ),
        (
            "Cos'e' la classificazione incidenti",
            "Art. 21(2)(b): Classificare per impatto: P1 (interruzione critica, data breach), P2 (degradazione significativa), P3 (impatto minore), P4 (informativo). P1/P2 attivano notifica Art. 23. Tempi: P1=immediato, P2=4h, P3=24h, P4=72h.",
            "incident_impl"
        ),
        (
            "Cos'e' una revisione post-incidente",
            "Art. 21(2)(b): Revisione blameless entro 5 giorni lavorativi. Documentare: timeline, causa radice, ritardo rilevamento, efficacia contenimento, lacune comunicazione. Produrre azioni con responsabili e scadenze.",
            "incident_impl"
        ),
        (
            "Cos'e' un esercizio tabletop",
            "Art. 21(2)(b)(f): Scenario incidente simulato per decisori. Nessun sistema live coinvolto. Testare: rilevamento, escalation, comunicazione, decisioni. Trimestrale minimo. Documentare risultati. Soddisfa Art. 21(2)(f).",
            "incident_impl"
        ),
        (
            "Cosa sono RTO e RPO",
            "Art. 21(2)(c): RTO (Recovery Time Objective) = downtime massimo accettabile. RPO (Recovery Point Objective) = perdita dati massima accettabile. Definire per sistema/servizio. Critici: RTO <4h, RPO <1h. Non critici: RTO <24h, RPO <24h.",
            "bc_impl"
        ),
        (
            "Come testare il disaster recovery",
            "Art. 21(2)(c)(f): Tipi test: (1) walkthrough tabletop (trimestrale), (2) failover parziale (semestrale), (3) failover completo (annuale). Documentare: RTO/RPO reale vs target, problemi trovati, azioni correttive.",
            "bc_impl"
        ),
        (
            "Come implementare una strategia di backup",
            "Art. 21(2)(c): Regola 3-2-1: 3 copie, 2 supporti diversi, 1 offsite. Cifrare backup a riposo. Testare ripristino mensilmente. Verifica automatica (checksum). Air-gap almeno una copia dalla rete.",
            "bc_impl"
        ),
        (
            "Come implementare il patch management",
            "Art. 21(2)(e): SLA per severita: critico (48h), alto (7gg), medio (30gg), basso (90gg). Scanner vulnerabilita (Nessus, Qualys). Automatizzare patch OS (WSUS, SCCM, unattended-upgrades). Testare in staging prima della produzione.",
            "vuln_impl"
        ),
        (
            "Come impostare la scansione vulnerabilita",
            "Art. 21(2)(e): Scansioni autenticate settimanali per sistemi internet-facing. Mensili per interni. Scanner sia di rete (Nessus) che web app (OWASP ZAP, Burp). Integrare risultati nel ticketing. Tracciare tempo medio di remediation.",
            "vuln_impl"
        ),
        (
            "Come definire lo scope di un penetration test",
            "Art. 21(2)(e)(f): Definire: scope (IP, applicazioni, social engineering?), regole di ingaggio, finestra di test, contatti emergenza. Black-box per esterno, grey-box per interno. Annuale minimo. Retest dopo fix critici.",
            "vuln_impl"
        ),
        (
            "Come fare un'analisi dei rischi",
            "Art. 21(2)(a): Framework: ISO 27005 o NIST CSF. Passi: (1) inventario asset, (2) identificazione minacce, (3) valutazione vulnerabilita, (4) analisi impatto, (5) rating probabilita, (6) scoring rischio (impatto x probabilita), (7) piano trattamento.",
            "risk_impl"
        ),
        (
            "Quale framework di rischio usare",
            "Art. 21(2)(a): Opzioni: ISO 27005 (standard EU, allineato NIS2), NIST CSF (US, ampiamente adottato), EBIOS RM (francese, endorsed ANSSI), BSI IT-Grundschutz (tedesco). Scegliere in base alla guida dell'autorita nazionale.",
            "risk_impl"
        ),
        (
            "Come fare una business impact analysis",
            "Art. 21(2)(a)(c): Identificare processi critici e sistemi di supporto. Per ciascuno: impatto dell'interruzione (finanziario, reputazionale, legale, sicurezza). Determinare RTO/RPO. Mappare dipendenze.",
            "risk_impl"
        ),
        (
            "Come faccio la gap analysis NIS2?",
            "Mappare ogni sub-lettera Art. 21(2)(a-j) vs stato attuale: (a) policy rischi, (b) incident handling, (c) continuita, (d) supply chain, (e) vulnerabilita, (f) assessment, (g) igiene, (h) crittografia, (i) accessi, (j) MFA/comunicazioni. Per ogni voce: conforme/parziale/assente.",
            "risk_impl"
        ),
        (
            "Come implementare la segmentazione di rete",
            "Art. 21(2)(a): Segmentare per funzione e sensibilita: DMZ, LAN aziendale, rete OT (se applicabile), rete management, rete guest. VLAN + regole firewall tra zone. Micro-segmentare carichi critici. Loggare traffico inter-zona.",
            "network_impl"
        ),
        (
            "Come implementare zero trust",
            "Art. 21(2)(a)(i)(j): Principi: verificare esplicitamente, privilegio minimo, assumere compromissione. Implementazione: accesso basato su identita, micro-segmentazione, autenticazione continua, verifica postura dispositivo.",
            "network_impl"
        ),
        (
            "Cos'e' il monitoraggio di rete",
            "Art. 21(2)(b): Implementare: raccolta NetFlow/sFlow, IDS/IPS (Suricata, Snort), monitoraggio DNS, cattura pacchetti su segmenti critici. Rilevare: beaconing C2, esfiltrazione dati, protocolli non autorizzati.",
            "network_impl"
        ),
        (
            "Come proteggere la rete OT?",
            "Art. 21(2)(a): segmentazione fisica/logica IT-OT (modello Purdue). Firewall industriali tra zone. Monitoraggio passivo traffico OT (no scan attivi). Patch management dedicato OT con finestre di manutenzione. Backup configurazioni PLC.",
            "network_impl"
        ),
        (
            "Come implementare la sicurezza email",
            "Art. 21(2)(g)(j): Implementare: SPF (strict -all), DKIM (2048-bit), DMARC (p=reject). Gateway filtraggio email. Sandboxing allegati. Formazione awareness trimestrale. Simulazione phishing mensile. Pulsante segnalazione in client email.",
            "email_impl"
        ),
        (
            "Come fare simulazioni di phishing",
            "Art. 21(2)(g): Campagne mensili di phishing simulato. Variare template: furto credenziali, allegato malevolo, CEO fraud. Tracciare: tasso click, tasso segnalazione, recidivi. Formazione mirata per chi fallisce. Obiettivo: <5% click rate.",
            "email_impl"
        ),
        (
            "Come fare formazione cybersecurity ai dipendenti?",
            "Art. 21(2)(g): programma strutturato: (1) onboarding sicurezza per nuovi assunti, (2) moduli trimestrali (phishing, password, social engineering), (3) simulazioni pratiche mensili, (4) test di verifica, (5) formazione ruolo-specifica per IT/admin. Tracciare completamento per audit.",
            "email_impl"
        ),
        (
            "Quali policy dobbiamo scrivere",
            "Art. 21: Policy minime: (1) sicurezza informazioni, (2) controllo accessi, (3) piano risposta incidenti, (4) piano continuita, (5) policy crittografia, (6) sicurezza supply chain, (7) gestione vulnerabilita, (8) uso accettabile.",
            "doc_impl"
        ),
        (
            "Come strutturare una policy di sicurezza",
            "Art. 21(2)(a): Struttura: (1) scopo e ambito, (2) ruoli e responsabilita, (3) dichiarazioni policy con riferimenti agli articoli, (4) standard e procedure, (5) processo eccezioni, (6) ciclo revisione (annuale minimo). Approvata dal CDA per Art. 20(1).",
            "doc_impl"
        ),
        (
            "Come scrivere una policy di sicurezza per NIS2?",
            "Art. 21(2)(a): (1) Titolo e versione, (2) Ambito di applicazione, (3) Riferimenti normativi (Art. specifici NIS2), (4) Ruoli e responsabilita, (5) Regole operative, (6) Eccezioni e deroghe, (7) Sanzioni interne, (8) Revisione annuale. Approvazione CDA obbligatoria (Art. 20).",
            "doc_impl"
        ),
        (
            "Quale documentazione tenere per l'audit?",
            "Art. 32/33: portfolio prove: (1) policy approvate dal CDA con data, (2) registri formazione firmati, (3) report analisi rischi, (4) log incidenti e notifiche, (5) report penetration test, (6) contratti fornitori con clausole di sicurezza, (7) verbali CDA, (8) rapporti audit interni.",
            "doc_impl"
        ),
        (
            "Come registrarsi presso l'autorita competente",
            "Art. 27: Entita essenziali e importanti devono registrarsi presso l'autorita nazionale competente. Fornire: nome entita, settore, contatti, range IP, stati membri di operazione. Scadenze variano per stato membro.",
            "legal_impl"
        ),
        (
            "Come prepararsi a un audit NIS2",
            "Art. 32/33: Preparare: (1) portfolio prove mappato su Art. 21(2)(a-j), (2) log incidenti (Art. 23), (3) registri formazione (Art. 20(2)), (4) report analisi rischi, (5) audit terze parti (SOC2/ISO27001), (6) verbali CDA con supervisione cybersecurity.",
            "legal_impl"
        ),
        (
            "Come ci registriamo presso l'ACN?",
            "Art. 27 + D.Lgs. 138/2024: registrazione tramite il portale dedicato dell'ACN. Fornire: denominazione, C.F./P.IVA, settore, range IP, contatti referente sicurezza, stati membri in cui si opera. Verificare le scadenze sul sito ACN.",
            "legal_impl"
        ),
        (
            "Come implementare il controllo accessi fisico",
            "Art. 21(2)(a): Misure: badge per server room, registro visitatori, CCTV agli ingressi, clean desk policy, blocco dispositivi, smaltimento sicuro supporti. Data center: mantrap, accesso biometrico, monitoraggio ambientale.",
            "physical_impl"
        ),
        (
            "Come proteggere l'accesso remoto",
            "Art. 21(2)(i)(j): Implementare: VPN always-on o ZTNA, MFA per ogni accesso remoto, verifica conformita endpoint (patch, AV), divieto split tunnel per ambienti high-security, DLP su endpoint, capacita remote wipe.",
            "remote_impl"
        ),
        (
            "Come implementare MDM",
            "Art. 21(2)(a)(i): Mobile Device Management: cifratura obbligatoria, remote wipe, versione OS minima, allowlist app, Wi-Fi basato su certificato, containerizzazione per BYOD. Separare dati personali/aziendali.",
            "remote_impl"
        ),
        (
            "What do I do if we share the admin password?",
            "Art. 21(2)(i): shared admin accounts violate individual accountability. Each administrator must have unique credentials. Root/admin access must use MFA per Art. 21(2)(j). Implement PAM for credential vaulting and session recording.",
            "access_control"
        ),
        (
            "We use the same root password everywhere",
            "Art. 21(2)(i): shared root accounts violate individual accountability. Each administrator must have unique credentials. Root/admin access must use MFA per Art. 21(2)(j). Immediate remediation: rotate all shared credentials, deploy PAM.",
            "access_control"
        ),
        (
            "Condividiamo la password di admin, e' un problema?",
            "Art. 21(2)(i): gli account admin condivisi violano la responsabilita individuale. Ogni amministratore deve avere credenziali uniche. L'accesso root/admin deve usare MFA per Art. 21(2)(j). Implementare PAM per vaulting credenziali.",
            "access_control"
        ),
        (
            "Usiamo la stessa password root ovunque",
            "Art. 21(2)(i): gli account root condivisi violano la responsabilita individuale. Azione immediata: ruotare tutte le credenziali condivise, implementare PAM, abilitare MFA su tutti gli accessi amministrativi.",
            "access_control"
        ),
        (
            "Serve il controllo accessi?",
            "Art. 21(2)(i) richiede esplicitamente policy di controllo accessi e gestione asset. Include: principio del minimo privilegio, separazione compiti, revisione periodica accessi, MFA per accessi critici.",
            "access_control"
        ),
        (
            "Serve gestire gli accessi privilegiati?",
            "Art. 21(2)(i) richiede controllo accessi e gestione asset. Soluzioni PAM che impongono privilegio minimo, registrazione sessioni e accesso just-in-time sono implementazioni riconosciute.",
            "access_control"
        ),
        (
            "Cos'e' il principio del minimo privilegio?",
            "Art. 21(2)(i) copre strategie di controllo accessi. Minimo privilegio significa che utenti e sistemi ricevono solo l'accesso minimo necessario. Si applica a tutti gli account, specialmente quelli amministrativi.",
            "access_control"
        ),
        (
            "SSH1 e' accettabile per NIS2?",
            "No. SSH1 ha vulnerabilita note e non e' piu considerato sicuro. Art. 21(2)(j) richiede comunicazioni protette. Usare SSH2 con chiavi Ed25519 o RSA 4096-bit. Disabilitare SSH1, password authentication e root login.",
            "access_control"
        ),
        (
            "Come implementare il principio del minimo privilegio?",
            "Art. 21(2)(i): (1) classificare utenti per ruolo, (2) assegnare solo permessi necessari, (3) rivedere trimestralmente, (4) account privilegiati via PAM, (5) accesso just-in-time per operazioni critiche.",
            "access_control"
        ),
        (
            "Come gestire il ciclo di vita delle identita?",
            "Art. 21(2)(i): provisioning automatizzato da HR. Processo di onboarding/offboarding documentato. Revoca immediata alla cessazione. Revisione accessi trimestrale. Disabilitazione account inattivi dopo 90 giorni.",
            "access_control"
        ),
        (
            "Serve la revisione periodica degli accessi?",
            "Art. 21(2)(i) e (f): revisione accessi almeno trimestrale per account privilegiati, semestrale per utenti standard. Documentare ogni revisione con approvazione del responsabile. Revocare accessi non necessari.",
            "access_control"
        ),
        (
            "Come implementare un sistema completo di controllo accessi per NIS2?",
            "Art. 21(2)(i) e (j) richiedono un sistema IAM completo: (1) Identity lifecycle — provisioning automatizzato basato su HR feed. Onboarding: account creato con permessi minimi del ruolo. Offboarding: revoca immediata tutti gli accessi alla cessazione del rapporto. Trasferimento: revisione e aggiornamento permessi. (2) Autenticazione — MFA obbligatorio per: accessi VPN, console amministrative, servizi cloud, email da dispositivi nuovi. FIDO2/passkey preferito a OTP. No SMS come secondo fattore (rischio SIM swap). (3) Autorizzazione — RBAC con ruoli definiti per funzione lavorativa. Principio del minimo privilegio. Segregation of duties per operazioni critiche. (4) PAM — accesso privilegiato via piattaforma PAM con: credential vaulting, sessioni registrate, just-in-time access con approvazione, rotazione automatica password di servizio. (5) Revisione — trimestrale per account privilegiati, semestrale per utenti standard. Account inattivi 90+ giorni: disabilitazione automatica. Report di accesso anomalo via SIEM.",
            "access_control"
        ),
        (
            "Our data is not encrypted, is that a problem?",
            "Art. 21(2)(h) requires cryptography policies. Unencrypted data at rest is a compliance gap. Implement AES-256 for storage, TLS 1.2+ for transit. Prioritize: databases, backups, removable media.",
            "encryption"
        ),
        (
            "I nostri dati non sono cifrati, e' un problema?",
            "Art. 21(2)(h) richiede politiche di crittografia. Dati non cifrati a riposo sono una lacuna di conformita. Implementare AES-256 per storage, TLS 1.2+ per transito. Priorita: database, backup, supporti rimovibili.",
            "encryption"
        ),
        (
            "Serve la crittografia?",
            "Art. 21(2)(h) richiede esplicitamente policy sull'uso della crittografia e della cifratura. Include cifratura dati a riposo (AES-256), in transito (TLS 1.2+) e gestione chiavi crittografiche.",
            "encryption"
        ),
        (
            "Che standard di cifratura servono?",
            "Art. 21(2)(h): ENISA raccomanda AES-256 per dati a riposo, TLS 1.2+ (preferire 1.3) per dati in transito. Algoritmi asimmetrici: RSA-4096 o ECDSA P-256+. Evitare: DES, 3DES, RC4, MD5, SHA-1.",
            "encryption"
        ),
        (
            "Possiamo usare FTP?",
            "No. FTP trasmette credenziali in chiaro, violando Art. 21(2)(h) crittografia e Art. 21(2)(j) comunicazioni protette. Usare SFTP (SSH) o FTPS (TLS). Verificare tutti i sistemi legacy che ancora usano FTP.",
            "encryption"
        ),
        (
            "Serve la firma digitale?",
            "Art. 21(2)(h) policy crittografiche: la firma digitale garantisce integrita e non-repudiabilita. Raccomandata per: approvazioni policy, contratti fornitori, notifiche al CSIRT, comunicazioni ufficiali. Usare firme qualificate eIDAS per valore legale.",
            "encryption"
        ),
        (
            "Come implementare la cifratura a riposo?",
            "Art. 21(2)(h): (1) cifratura disco completa (BitLocker/FileVault), (2) cifratura database (TDE), (3) cifratura backup, (4) cifratura storage cloud. AES-256 come standard minimo. Gestione chiavi separata.",
            "encryption"
        ),
        (
            "Servono certificati SSL/TLS?",
            "Art. 21(2)(h)(j): TLS 1.2+ obbligatorio per tutti i servizi esposti. Certificati da CA riconosciuta o PKI interna. Automazione rinnovo (ACME/Let's Encrypt). Monitoraggio scadenza. HSTS abilitato.",
            "encryption"
        ),
        (
            "Come gestire le chiavi crittografiche?",
            "Art. 21(2)(h): KMS centralizzato (HSM o cloud KMS). Rotazione chiavi annuale minimo. Separazione delle responsabilita (chi cifra non gestisce le chiavi). Backup sicuro delle chiavi master. Audit trail completo.",
            "encryption"
        ),
        (
            "Quali sono i requisiti di crittografia dettagliati della NIS2?",
            "Art. 21(2)(h) richiede politiche di crittografia e cifratura complete: (1) Dati a riposo — AES-256 per database, backup, storage cloud. Cifratura disco completa per laptop e dispositivi mobili (BitLocker, FileVault). Cifratura TDE per database in produzione. (2) Dati in transito — TLS 1.2+ obbligatorio, TLS 1.3 raccomandato. mTLS per comunicazione tra servizi. VPN con cifratura forte per accesso remoto. HSTS per tutti i servizi web. (3) Gestione chiavi — KMS centralizzato (HSM on-premise o cloud KMS). Rotazione chiavi annuale minimo, trimestrale per chiavi ad alto rischio. Separazione delle responsabilita: chi cifra non gestisce le chiavi. Backup sicuro chiavi master in location separata. (4) Certificati — PKI aziendale o CA esterna riconosciuta. Automazione rinnovo (ACME/Let's Encrypt). Monitoraggio scadenza con alert 30/14/7 giorni. Certificate pinning per app critiche. (5) Firma digitale — firma qualificata eIDAS per documenti ufficiali, notifiche al CSIRT, approvazioni CDA. Non-repudiabilita garantita.",
            "encryption"
        ),
        (
            "We got hacked, what do we do?",
            "Art. 23: (1) Contain the breach immediately. (2) Send early warning to CSIRT within 24h. (3) Full notification within 72h. (4) Preserve evidence. (5) Activate incident response plan. (6) Final report within 30 days.",
            "incident_response"
        ),
        (
            "Ci hanno hackerato, cosa facciamo?",
            "Art. 23: (1) Contenere la violazione immediatamente. (2) Preallarme al CSIRT entro 24h. (3) Notifica completa entro 72h. (4) Preservare le prove. (5) Attivare piano di risposta incidenti. (6) Relazione finale entro 30 giorni.",
            "incident_response"
        ),
        (
            "Entro quanto tempo devo notificare un incidente?",
            "Art. 23: preallarme al CSIRT entro 24h dal momento in cui si viene a conoscenza di un incidente significativo. Notifica completa entro 72h. Relazione finale entro 30 giorni.",
            "incident_response"
        ),
        (
            "A chi notifico un incidente?",
            "Art. 23: al CSIRT nazionale. In Italia: CSIRT Italia presso l'ACN. Preallarme entro 24h, notifica completa entro 72h. Se coinvolge dati personali, anche al Garante Privacy (GDPR, 72h).",
            "incident_response"
        ),
        (
            "Serve un piano di risposta incidenti?",
            "Art. 21(2)(b) richiede gestione incidenti: rilevamento, analisi, contenimento, eliminazione, ripristino, revisione post-incidente. Le procedure devono essere documentate e testate regolarmente.",
            "incident_response"
        ),
        (
            "Siamo stati colpiti da un ransomware, cosa fare?",
            "Art. 23 + Art. 21(2)(b): (1) Isolare immediatamente i sistemi infetti. (2) NON pagare il riscatto (alimenta il crimine). (3) Preallarme CSIRT entro 24h. (4) Preservare evidenze forensi. (5) Avviare ripristino da backup offline. (6) Notifica completa entro 72h.",
            "incident_response"
        ),
        (
            "Dobbiamo pagare il riscatto?",
            "Le autorita (ACN, ENISA, Europol) sconsigliano fortemente il pagamento. Non garantisce il ripristino dei dati, finanzia il crimine organizzato e puo comportare violazione delle sanzioni EU. Investire invece in backup e prevenzione per Art. 21(2)(c).",
            "incident_response"
        ),
        (
            "Come strutturare un team di risposta incidenti?",
            "Art. 21(2)(b): team CSIRT interno con ruoli definiti: incident manager, analista forense, comunicazione, legale, IT operations. Formazione periodica, esercitazioni simulate, reperibilita 24/7 per entita essenziali.",
            "incident_response"
        ),
        (
            "Cosa includere nella notifica al CSIRT?",
            "Art. 23(4): (a) preallarme 24h: natura incidente, sospetto malevolo, impatto transfrontaliero. (b) notifica 72h: valutazione severita, impatto, indicatori di compromissione. (d) relazione finale 30gg: root cause, misure adottate.",
            "incident_response"
        ),
        (
            "Come classificare la severita di un incidente?",
            "Art. 23(3): incidente significativo = impatto operativo sul servizio, perdita finanziaria, danno a persone fisiche. Criteri: numero utenti impattati, durata interruzione, estensione geografica, dati personali coinvolti.",
            "incident_response"
        ),
        (
            "Serve un'esercitazione di risposta incidenti?",
            "Art. 21(2)(b) e (f): le procedure devono essere testate. Esercitazioni tabletop (annuali), simulazioni tecniche (semestrali), red team (per entita essenziali). Documentare risultati e lezioni apprese.",
            "incident_response"
        ),
        (
            "Cosa fare in caso di ransomware?",
            "Art. 23 + Art. 21(2)(b)(c): (1) isolare sistemi compromessi, (2) notifica CSIRT entro 24h, (3) attivare piano DR, (4) analisi forense, (5) ripristino da backup puliti, (6) notifica GDPR se dati personali, (7) relazione finale.",
            "incident_response"
        ),
        (
            "Quali sono le tempistiche esatte per la notifica degli incidenti?",
            "Art. 23 NIS2 stabilisce tre fasi obbligatorie: (1) Preallarme entro 24 ore dalla conoscenza dell'incidente significativo — deve indicare se l'incidente e' sospettato di essere causato da atti illeciti o malevoli e se ha o puo' avere impatto transfrontaliero. (2) Notifica completa entro 72 ore — deve includere aggiornamento delle informazioni del preallarme, valutazione iniziale della severita e dell'impatto, e indicatori di compromissione se disponibili. (3) Relazione finale entro 30 giorni — deve contenere descrizione dettagliata dell'incidente, tipo di minaccia o causa radice, misure di mitigazione applicate, impatto transfrontaliero se applicabile. Per incidenti in corso, la relazione finale diventa una relazione intermedia con obbligo di relazione finale entro un mese dalla gestione.",
            "incident_response"
        ),
        (
            "Come prepararsi a un attacco ransomware secondo la NIS2?",
            "La preparazione anti-ransomware richiede misure da piu articoli: Art. 21(2)(a) analisi rischi — valutare la probabilita e l'impatto di un attacco ransomware sulla propria organizzazione. Art. 21(2)(b) gestione incidenti — avere un playbook specifico per ransomware con procedure di isolamento, comunicazione e decisione sul pagamento del riscatto. Art. 21(2)(c) continuita operativa — backup immutabili (air-gapped), testati regolarmente, con RTO/RPO definiti. Regola 3-2-1: tre copie, due supporti, una offsite. Art. 21(2)(e) gestione vulnerabilita — patching tempestivo, prioritizzando le vulnerabilita nel catalogo KEV di CISA. Art. 21(2)(g) igiene cyber — segmentazione rete, principio del minimo privilegio, disabilitazione macro, formazione anti-phishing. Art. 21(2)(i) controllo accessi — PAM per account privilegiati, MFA obbligatorio, monitoraggio accessi anomali.",
            "incident_response"
        ),
        (
            "Quando entra in vigore NIS2?",
            "La Direttiva (UE) 2022/2555 e' entrata in vigore il 16 gennaio 2023. Gli stati membri avevano tempo fino al 17 ottobre 2024 per il recepimento in legge nazionale. Gli obblighi si applicano dalla data di recepimento.",
            "legal"
        ),
        (
            "NIS2 e' gia legge in Italia?",
            "Il D.Lgs. 138/2024 ha recepito la NIS2 in Italia. L'ACN (Agenzia per la Cybersicurezza Nazionale) e' l'autorita competente. Le scadenze di registrazione e conformita sono definite nel decreto.",
            "legal"
        ),
        (
            "Chi e' l'autorita competente in Italia?",
            "L'ACN (Agenzia per la Cybersicurezza Nazionale) e' l'autorita competente per NIS2 in Italia. Il CSIRT Italia, operante presso l'ACN, riceve le notifiche di incidente per Art. 23.",
            "legal"
        ),
        (
            "Cos'e' il CSIRT?",
            "Computer Security Incident Response Team designato per Art. 10 di NIS2. In Italia e' il CSIRT Italia presso l'ACN. Riceve le notifiche di incidenti da entita essenziali e importanti.",
            "legal"
        ),
        (
            "Che rapporto c'e' tra NIS2 e GDPR?",
            "NIS2 copre sicurezza di reti e sistemi. GDPR copre protezione dati personali. Si sovrappongono su notifica breach e misure di sicurezza. Art. 35 NIS2 richiede cooperazione con autorita protezione dati quando gli incidenti coinvolgono dati personali.",
            "legal"
        ),
        (
            "NIS2 sostituisce il GDPR?",
            "No. NIS2 e GDPR sono complementari. GDPR si applica al trattamento dati personali (72h notifica al Garante). NIS2 si applica alla sicurezza di rete delle entita (24h preallarme al CSIRT). Entrambi possono applicarsi simultaneamente.",
            "legal"
        ),
        (
            "Operiamo in piu paesi UE, come funziona?",
            "Art. 26 stabilisce le regole di giurisdizione. L'autorita competente primaria e' nello stato membro dove l'entita ha il suo stabilimento principale. Art. 37 consente assistenza reciproca tra autorita.",
            "legal"
        ),
        (
            "Cosa prevede il D.Lgs. 138/2024?",
            "Recepisce la Direttiva NIS2 in Italia. Punti chiave: ACN come autorita competente, CSIRT Italia per notifiche, obbligo registrazione entita, sanzioni allineate (fino a 10M EUR / 2%), responsabilita organi direttivi, tempistiche conformita.",
            "legal"
        ),
        (
            "NIS2 e il Perimetro di Sicurezza Nazionale Cibernetica?",
            "Sono complementari. Il Perimetro (D.L. 105/2019) copre soggetti strategici nazionali con obblighi piu stringenti. NIS2 ha ambito piu ampio (tutti i settori Allegato I/II). Un soggetto puo essere soggetto a entrambi. L'ACN gestisce entrambi.",
            "legal"
        ),
        (
            "Qual e' il rapporto tra NIS2 e GDPR?",
            "NIS2 copre sicurezza reti e sistemi. GDPR copre protezione dati personali. Si sovrappongono su notifica breach e misure di sicurezza. Art. 35 NIS2 richiede cooperazione con autorita protezione dati quando gli incidenti coinvolgono dati personali.",
            "legal"
        ),
        (
            "La NIS2 sostituisce il GDPR?",
            "No. NIS2 e GDPR sono complementari. GDPR: notifica breach DPA entro 72h per dati personali. NIS2: preallarme CSIRT entro 24h per sicurezza reti. Entrambi possono applicarsi simultaneamente allo stesso incidente.",
            "legal"
        ),
        (
            "Chi e' l'autorita competente NIS2 in Italia?",
            "L'ACN (Agenzia per la Cybersicurezza Nazionale) e' l'autorita competente NIS2 in Italia (Art. 8). Il CSIRT Italia (presso ACN) riceve le notifiche di incidente (Art. 10). L'ACN supervisiona anche DORA per il settore finanziario.",
            "legal"
        ),
        (
            "La NIS2 e' gia stata recepita in Italia?",
            "Il D.Lgs. 138/2024 ha recepito la NIS2 in Italia. Entrato in vigore il 16 ottobre 2024. Gli obblighi di registrazione e notifica sono operativi. Verificare le scadenze specifiche presso l'ACN.",
            "legal"
        ),
        (
            "Cosa succede se operiamo in piu paesi UE?",
            "Art. 26: la giurisdizione principale e' nello stato membro dell'establishment principale. Art. 37 consente assistenza reciproca tra autorita. Per provider digitali, vale lo stato della sede principale.",
            "legal"
        ),
        (
            "Come si relazionano NIS2, GDPR e DORA?",
            "Le tre normative sono complementari: (1) NIS2 — copre la sicurezza delle reti e dei sistemi informativi per entita essenziali e importanti. Obbligo: misure di sicurezza Art. 21, notifica incidenti Art. 23 (24h/72h/30gg). Autorita: CSIRT/ACN. (2) GDPR — copre la protezione dei dati personali per qualsiasi titolare/responsabile del trattamento. Obbligo: misure tecniche e organizzative Art. 32, notifica breach DPA entro 72h (Art. 33), notifica interessati (Art. 34). Autorita: Garante Privacy. (3) DORA — copre la resilienza operativa digitale per il settore finanziario (banche, assicurazioni, fintech). Requisiti piu stringenti di NIS2 per ICT risk management, test di resilienza, gestione fornitori ICT. Art. 4 NIS2 stabilisce che DORA prevale come lex specialis per il settore finanziario. (4) Sovrapposizione — un incidente cyber in una banca con dati personali attiva: NIS2 (notifica CSIRT 24h), GDPR (notifica DPA 72h), DORA (notifica autorita di vigilanza finanziaria). Art. 35 NIS2 richiede cooperazione tra autorita NIS2 e autorita protezione dati.",
            "legal"
        ),
        (
            "Serve il change management?",
            "Art. 21(2)(e) copre sicurezza nella manutenzione dei sistemi. Change management formale con revisione di sicurezza, test e procedure di rollback e' una pratica attesa per mantenere l'integrita dei sistemi.",
            "operations"
        ),
        (
            "Cos'e' la cyber hygiene secondo NIS2?",
            "Art. 21(2)(g): include policy di configurazione dispositivi, aggiornamenti software, creazione account sicuri, gestione password. Richiede anche formazione periodica in cybersecurity per tutti i dipendenti.",
            "operations"
        ),
        (
            "Serve il cloud security posture management?",
            "Art. 21(2)(a) analisi rischi e Art. 21(2)(e) sicurezza sistemi si applicano agli ambienti cloud. Strumenti CSPM che rilevano misconfigurazioni sono una pratica riconosciuta per workload cloud.",
            "operations"
        ),
        (
            "Quali sono le prime 5 cose da fare per NIS2?",
            "1. Inventario asset e dati critici (Art. 21(2)(i)). 2. MFA su tutti gli accessi admin (Art. 21(2)(j)). 3. Backup 3-2-1 con test ripristino (Art. 21(2)(c)). 4. Piano risposta incidenti con contatti CSIRT (Art. 23). 5. Formazione base per tutti (Art. 21(2)(g)).",
            "operations"
        ),
        (
            "Cosa possiamo fare subito a costo zero?",
            "Azioni immediate: (1) abilitare MFA ovunque disponibile, (2) disabilitare account non utilizzati, (3) configurare SPF/DKIM/DMARC, (4) verificare che i backup funzionino (test ripristino), (5) documentare chi contattare in caso di incidente, (6) disabilitare protocolli obsoleti (TLS 1.0/1.1).",
            "operations"
        ),
        (
            "Serve il change management per NIS2?",
            "Art. 21(2)(e): la manutenzione dei sistemi richiede gestione dei cambiamenti formale. Review di sicurezza, test, approvazione, rollback documentati per ogni modifica a sistemi in scope.",
            "operations"
        ),
        (
            "Come gestire gli aggiornamenti software?",
            "Art. 21(2)(e)(g): aggiornamenti regolari fanno parte della igiene cyber e gestione vulnerabilita. Processo: test in staging, approvazione, deployment controllato, rollback plan, verifica post-deploy.",
            "operations"
        ),
        (
            "Cosa dice NIS2 sulla continuita operativa?",
            "Art. 21(2)(c) richiede: gestione continuita operativa, procedure di backup, disaster recovery e gestione delle crisi. I piani devono essere documentati, testati e manutenuti.",
            "business_continuity"
        ),
        (
            "Servono i backup?",
            "Art. 21(2)(c) richiede esplicitamente la gestione dei backup e il disaster recovery. Backup regolari e testati con copie offsite/offline sono il minimo. Regola 3-2-1.",
            "business_continuity"
        ),
        (
            "Serve un piano di disaster recovery?",
            "Art. 21(2)(c) richiede continuita operativa e gestione delle crisi, incluso il disaster recovery. Il piano deve definire RTO/RPO, essere documentato, testato e aggiornato regolarmente.",
            "business_continuity"
        ),
        (
            "Ogni quanto testare i backup?",
            "Art. 21(2)(c) richiede gestione backup e Art. 21(2)(f) valutazione efficacia. Test di ripristino regolari sono prassi attesa. Minimo trimestrale per sistemi critici, semestrale per gli altri.",
            "business_continuity"
        ),
        (
            "Cosa deve contenere un piano di continuita operativa?",
            "Art. 21(2)(c): analisi impatto operativo (BIA), procedure di ripristino, ruoli e responsabilita, contatti di emergenza, procedure di comunicazione, test periodici, aggiornamento post-incidente.",
            "business_continuity"
        ),
        (
            "Serve un sito di disaster recovery?",
            "Art. 21(2)(c) richiede disaster recovery. Un sito DR (fisico o cloud) e' necessario per entita essenziali con RTO stringenti. Per entita importanti, backup offsite con ripristino documentato puo bastare.",
            "business_continuity"
        ),
        (
            "Come definire RTO e RPO?",
            "Art. 21(2)(c): RTO (Recovery Time Objective) = tempo massimo di inattivita tollerabile. RPO (Recovery Point Objective) = perdita dati massima accettabile. Derivano dalla BIA e guidano la strategia di backup e DR.",
            "business_continuity"
        ),
        (
            "La NIS2 richiede un piano di gestione delle crisi?",
            "Si. Art. 21(2)(c) elenca esplicitamente la gestione delle crisi. Include: escalation, comunicazione interna/esterna, coordinamento con CSIRT, decisioni operative in emergenza.",
            "business_continuity"
        ),
        (
            "Serve un backup offsite?",
            "Art. 21(2)(c): i backup devono proteggere da ransomware e disastri fisici. Backup offsite (o air-gapped) e' una misura riconosciuta. La regola 3-2-1 (3 copie, 2 supporti, 1 offsite) e' best practice.",
            "business_continuity"
        ),
        (
            "Come strutturare un programma completo di continuita operativa?",
            "Art. 21(2)(c) richiede un approccio strutturato: (1) BIA — Business Impact Analysis per ogni servizio critico. Definire RTO (tempo di ripristino) e RPO (perdita dati accettabile). Identificare dipendenze tra sistemi. (2) Strategia di backup — regola 3-2-1-1-0: tre copie, due supporti diversi, una offsite, una immutabile, zero errori nei test di ripristino. Backup incrementali giornalieri, completi settimanali. (3) Piano DR — documentare procedure di ripristino per ogni scenario: guasto hardware, attacco ransomware, disastro fisico, compromissione cloud. Definire ruoli e responsabilita. Contact list aggiornata. (4) Piano di gestione crisi — escalation, comunicazione interna (dipendenti) e esterna (clienti, autorita, media), coordinamento con CSIRT. Decision tree per attivazione DR. (5) Test — test di ripristino backup: mensile per sistemi critici, trimestrale per gli altri. Esercitazione tabletop: semestrale. Failover test: annuale. Documentare risultati e lezioni apprese. (6) Manutenzione — revisione piani: annuale e dopo ogni incidente significativo. Aggiornamento contact list trimestrale.",
            "business_continuity"
        ),
        (
            "NIS2 richiede zero trust?",
            "NIS2 non impone zero trust per nome. Tuttavia Art. 21(2)(i) controllo accessi, Art. 21(2)(j) autenticazione continua e Art. 21(2)(a) policy basate sul rischio sono allineati con i principi zero trust.",
            "architecture"
        ),
        (
            "NIS2 copre la sicurezza fisica?",
            "Art. 21(2)(a) richiede policy di sicurezza dei sistemi informatici che includono sicurezza fisica e ambientale di reti e sistemi. I controlli di accesso fisico sono in ambito.",
            "physical"
        ),
        (
            "La NIS2 copre la sicurezza fisica?",
            "Art. 21(2)(a) richiede policy di sicurezza dei sistemi informatici che includono sicurezza fisica e ambientale. Controllo accessi fisici ai data center, sale server e infrastrutture critiche e' in scope.",
            "physical"
        ),
        (
            "Serve il controllo accessi fisico ai server?",
            "Art. 21(2)(a) e (i): accesso fisico ai sistemi critici deve essere controllato. Badge, biometria, registrazione accessi, videosorveglianza, accompagnamento visitatori sono misure attese.",
            "physical"
        ),
        (
            "Come proteggere fisicamente il data center?",
            "Art. 21(2)(a): controllo accessi multi-livello, rilevamento intrusione, videosorveglianza, protezione ambientale (antincendio, climatizzazione, UPS), ridondanza alimentazione, perimetro sicuro.",
            "physical"
        ),
        (
            "La videosorveglianza e' richiesta dalla NIS2?",
            "Non esplicitamente, ma Art. 21(2)(a) richiede sicurezza dei sistemi. La videosorveglianza delle aree critiche (sale server, accessi) e' una misura di sicurezza fisica riconosciuta e attesa.",
            "physical"
        ),
        (
            "Cosa dice NIS2 sul lavoro remoto?",
            "Art. 21(2)(j) richiede comunicazioni protette e Art. 21(2)(i) richiede controllo accessi. L'accesso remoto deve usare VPN o ZTNA con MFA. Le policy BYOD rientrano in Art. 21(2)(a).",
            "remote_work"
        ),
        (
            "Serve una VPN?",
            "Art. 21(2)(j) richiede comunicazioni protette. VPN o tunnel cifrati equivalenti sono standard per l'accesso remoto a risorse interne. Le policy di split tunneling vanno valutate per rischio.",
            "remote_work"
        ),
        (
            "Come gestire il lavoro remoto per NIS2?",
            "Art. 21(2)(j) comunicazioni protette e Art. 21(2)(i) controllo accessi. Accesso remoto via VPN o ZTNA con MFA. Policy BYOD sotto Art. 21(2)(a). Cifratura disco obbligatoria su dispositivi mobili.",
            "remote_work"
        ),
        (
            "Serve una VPN per NIS2?",
            "Art. 21(2)(j) richiede comunicazioni protette. VPN o tunnel cifrati equivalenti sono lo standard per accesso remoto. Valutare split tunneling rispetto ai rischi per Art. 21(2)(a).",
            "remote_work"
        ),
        (
            "Serve una policy BYOD?",
            "Art. 21(2)(a) analisi rischi e Art. 21(2)(i) gestione asset richiedono policy per dispositivi personali. MDM (Mobile Device Management) obbligatorio. Separazione dati aziendali/personali.",
            "remote_work"
        ),
        (
            "Come proteggere i dispositivi in smart working?",
            "Art. 21(2)(a)(i)(j): cifratura disco completa, MFA per ogni accesso, VPN always-on, EDR installato, aggiornamenti automatici, timeout sessione, divieto di salvataggio locale dati critici.",
            "remote_work"
        ),
        (
            "Il dipendente puo usare il PC personale?",
            "Art. 21(2)(a): consentito solo con policy BYOD approvata. Requisiti minimi: MDM, cifratura, antivirus gestito, VPN, separazione profilo, capacita di wipe remoto, sistema operativo supportato.",
            "remote_work"
        ),
        (
            "Serve un'assicurazione cyber?",
            "NIS2 non impone assicurazione cyber. Tuttavia Art. 21(2)(a) analisi rischi deve valutare il rischio residuo. L'assicurazione e' un meccanismo di trasferimento del rischio che complementa ma non sostituisce le misure tecniche.",
            "risk_assessment"
        ),
        (
            "Quanto costa adeguarsi a NIS2?",
            "Il costo dipende dalla maturita attuale. Voci principali: assessment iniziale (gap analysis), remediation tecnica (firewall, SIEM, PAM, cifratura), documentazione policy, formazione, e audit. Le PMI possono partire da 30-80K EUR; le grandi aziende molto di piu.",
            "risk_assessment"
        ),
        (
            "Quanto tempo serve per adeguarsi?",
            "Tipicamente 6-18 mesi dalla gap analysis all'adeguamento completo. Fasi: (1) assessment 1-2 mesi, (2) piano remediation 1 mese, (3) implementazione tecnica 3-9 mesi, (4) documentazione 2-3 mesi, (5) test e audit 1-2 mesi.",
            "risk_assessment"
        ),
        (
            "Da dove iniziare con NIS2?",
            "Passo 1: verificare se rientrate (settore + soglia). Passo 2: gap analysis vs Art. 21(2)(a-j). Passo 3: prioritizzare per rischio. Passo 4: implementare misure critiche (incident response, accessi, cifratura). Passo 5: documentare policy. Passo 6: registrarsi presso l'autorita.",
            "risk_assessment"
        ),
        (
            "Come fare un'analisi dei rischi per NIS2?",
            "Art. 21(2)(a): (1) inventario asset, (2) identificazione minacce, (3) valutazione vulnerabilita, (4) analisi impatto, (5) calcolo rischio residuo, (6) piano di trattamento. Metodologie: ISO 27005, NIST SP 800-30.",
            "risk_assessment"
        ),
        (
            "Ogni quanto aggiornare l'analisi dei rischi?",
            "Art. 21(2)(a) e (f): revisione annuale come minimo. Aggiornamento aggiuntivo in caso di: incidente significativo, cambio infrastruttura, nuova minaccia, cambio normativo, acquisizione aziendale.",
            "risk_assessment"
        ),
        (
            "Serve un registro dei rischi?",
            "Art. 21(2)(a): il registro dei rischi documenta rischi identificati, probabilita, impatto, misure di mitigazione, rischio residuo, responsabile. E' il prodotto principale dell'analisi rischi.",
            "risk_assessment"
        ),
        (
            "Come classificare i rischi per priorita?",
            "Art. 21(2)(a): matrice probabilita x impatto. Criteri: impatto operativo, finanziario, reputazionale, legale. Prioritizzare: rischi critici (trattamento immediato), alti (piano trimestrale), medi (annuale).",
            "risk_assessment"
        ),
        (
            "La NIS2 richiede un risk appetite statement?",
            "Art. 20(1): gli organi direttivi approvano le misure di sicurezza. Definire la propensione al rischio dell'organizzazione guida le decisioni di trattamento. Documento approvato dal CDA.",
            "risk_assessment"
        ),
        (
            "Come condurre un'analisi dei rischi completa per NIS2?",
            "Art. 21(2)(a) richiede una metodologia strutturata. Fasi raccomandate: (1) Inventario asset — catalogare tutti i sistemi informativi, dati, servizi, personale, fornitori in scope NIS2. Classificare per criticita. (2) Identificazione minacce — utilizzare il framework ENISA Threat Landscape come riferimento: ransomware, phishing, APT, DDoS, supply chain attack, insider threat. (3) Valutazione vulnerabilita — vulnerability scanning, pentest, review configurazioni. Incrociare con catalogo KEV CISA. (4) Analisi impatto — per ogni scenario: impatto operativo (RTO), finanziario (costo downtime + sanzione NIS2), reputazionale, legale. (5) Calcolo rischio — matrice probabilita x impatto. Classificare: critico, alto, medio, basso. (6) Piano trattamento — per ogni rischio critico/alto: misura di mitigazione, responsabile, scadenza, costo, rischio residuo accettato. Approvazione formale del CDA. Revisione almeno annuale, aggiornamento al verificarsi di incidenti, cambi infrastruttura, nuove minacce.",
            "risk_assessment"
        ),
        (
            "NIS2 copre il phishing?",
            "Art. 21(2)(g) richiede igiene informatica di base e formazione. Misure anti-phishing (SPF, DKIM, DMARC, formazione awareness, filtraggio email) rientrano nell'igiene e prevenzione incidenti.",
            "email_security"
        ),
        (
            "Serve la sicurezza email?",
            "Art. 21(2)(g) igiene informatica e Art. 21(2)(j) comunicazioni protette si applicano. Misure di sicurezza email incluse cifratura (S/MIME o TLS), anti-spam e anti-phishing sono attese.",
            "email_security"
        ),
        (
            "Serve formare tutti i dipendenti?",
            "Art. 21(2)(g) richiede igiene informatica di base e formazione periodica per TUTTI i dipendenti, non solo IT. Contenuti: riconoscere phishing, gestione password, segnalazione incidenti, policy uso accettabile, sicurezza dati. Frequenza: trimestrale raccomandata.",
            "email_security"
        ),
        (
            "Riceviamo molte email di phishing",
            "Art. 21(2)(g)(j): azioni immediate: (1) implementare SPF/DKIM/DMARC con p=reject, (2) gateway email con sandboxing, (3) formazione awareness mensile, (4) simulazioni phishing, (5) pulsante 'Segnala phishing' nel client email, (6) bloccare macro Office di default.",
            "email_security"
        ),
        (
            "Serve la cifratura delle email?",
            "Art. 21(2)(h) e (j): la cifratura email e' raccomandata per comunicazioni sensibili. Opzioni: S/MIME per email individuali, TLS obbligatorio tra gateway (MTA-STS), cifratura end-to-end per dati classificati. Come minimo TLS in transito.",
            "email_security"
        ),
        (
            "Come configurare SPF, DKIM e DMARC?",
            "Art. 21(2)(g) igiene cyber: SPF (record DNS con IP autorizzati), DKIM (firma crittografica email), DMARC (policy di enforcement). Implementare in ordine: SPF → DKIM → DMARC p=quarantine → p=reject.",
            "email_security"
        ),
        (
            "Come prevenire il phishing?",
            "Art. 21(2)(g): (1) filtri email gateway, (2) SPF/DKIM/DMARC, (3) formazione periodica utenti, (4) simulazioni phishing, (5) sandboxing allegati, (6) URL rewriting, (7) reporting button per segnalazione rapida.",
            "email_security"
        ),
        (
            "Serve un firewall?",
            "Art. 21(2)(a) sicurezza dei sistemi informatici. Firewall, IDS/IPS, WAF e monitoraggio di rete sono misure di base. La segmentazione limita il movimento laterale in caso di compromissione.",
            "network_security"
        ),
        (
            "NIS2 copre i sistemi OT e SCADA?",
            "Si. Art. 21(2)(a) copre la sicurezza di reti e sistemi informatici, inclusi i sistemi OT/SCADA. La segmentazione IT/OT e' una misura fondamentale. IEC 62443 e' lo standard di riferimento per la sicurezza OT.",
            "network_security"
        ),
        (
            "Il Wi-Fi aziendale e' in scope NIS2?",
            "Si. Art. 21(2)(a) copre la sicurezza dei sistemi informatici. Wi-Fi aziendale: WPA3 o WPA2-Enterprise con 802.1X. Rete guest separata. Segmentazione VLAN. Monitoraggio rogue AP. Certificati invece di PSK per accesso aziendale.",
            "network_security"
        ),
        (
            "Come segmentare la rete per NIS2?",
            "Art. 21(2)(a): (1) separare rete OT da IT, (2) DMZ per servizi esposti, (3) VLAN per funzione/criticita, (4) microsegmentazione per sistemi critici, (5) firewall tra segmenti. Zero trust tra zone.",
            "network_security"
        ),
        (
            "Serve un IDS/IPS?",
            "Art. 21(2)(a)(b): sistemi di rilevamento intrusione sono misure attese per sicurezza di rete e gestione incidenti. IDS per rilevamento, IPS per prevenzione attiva. Posizionare su perimetro e segmenti critici.",
            "network_security"
        ),
        (
            "Come proteggere il DNS?",
            "Art. 21(2)(a): DNSSEC per integrita, DNS filtering per blocco domini malevoli, DNS logging per forensics, DNS sinkhole per malware C2. Provider DNS in Allegato I come infrastruttura digitale.",
            "network_security"
        ),
        (
            "Serve il monitoraggio del traffico di rete?",
            "Art. 21(2)(b): il rilevamento incidenti richiede visibilita sul traffico. NetFlow/sFlow, deep packet inspection, analisi anomalie. Monitoraggio est-ovest (laterale) oltre che nord-sud (perimetrale).",
            "network_security"
        ),
        (
            "Come progettare un'architettura di rete sicura per NIS2?",
            "Art. 21(2)(a) richiede sicurezza dei sistemi informativi, che include l'architettura di rete: (1) Segmentazione — separare rete OT da IT con firewall dedicato. DMZ per servizi esposti. VLAN per funzione: server, utenti, gestione, IoT. Microsegmentazione per sistemi critici. (2) Perimetro — firewall next-generation con ispezione deep packet. WAF per applicazioni web. IDS/IPS in linea. DNS filtering. Reverse proxy per servizi esposti. (3) Monitoraggio — SIEM con raccolta log da tutti i segmenti. NDR per analisi traffico laterale (est-ovest). NetFlow per analisi comportamentale. Alert per anomalie. (4) Accesso remoto — VPN con MFA o ZTNA (Zero Trust Network Access). Split tunneling valutato e documentato. Timeout sessione. Logging completo. (5) Wireless — WPA3 o WPA2-Enterprise con 802.1X. Rete guest isolata. WIDS per rogue AP. Certificati invece di PSK. (6) Resilienza — link ridondanti per siti critici. Failover automatico. DDoS protection. BGP monitoring per hijack detection.",
            "network_security"
        ),
        (
            "Che differenza c'e' tra entita essenziale e importante?",
            "Art. 3: essenziali = settori Allegato I (energia, trasporti, banche, sanita, acqua, digitale, PA) sopra soglia. Importanti = settori Allegato II (postali, rifiuti, chimico, alimentare, manifattura, digitale, ricerca). Essenziali: audit proattivi, sanzioni piu alte (10M vs 7M).",
            "classification"
        ),
        (
            "Quali sono gli obblighi NIS2?",
            "La Direttiva NIS2 introduce **16 obblighi principali** strutturati in 3 domini critici:

### 1. Governance (Art. 20)
*   **Approvazione e Supervisione:** Gli organi direttivi devono approvare formalmente le misure di gestione dei rischi di sicurezza e supervisionarne l'attuazione (con responsabilita' personale).
*   **Formazione Obbligatoria:** I membri degli organi direttivi devono seguire regolarmente corsi di formazione sulla cybersecurity per comprendere e valutare i rischi.

### 2. Misure di Sicurezza (Art. 21)
Le entita' devono implementare misure minime, tra cui:
*   **Analisi dei Rischi:** Valutazione continua e politiche di sicurezza dei sistemi informatici.
*   **Gestione degli Incidenti:** Procedure per prevenzione, rilevamento e risposta agli attacchi.
*   **Continuita' Operativa:** Gestione dei backup, disaster recovery e gestione delle crisi.
*   **Supply Chain Security:** Valutazione della sicurezza dei fornitori diretti e partner.
*   **Sicurezza dei Sistemi:** Sicurezza nello sviluppo, acquisizione e manutenzione, inclusa la gestione delle vulnerabilita'.
*   **Valutazione dell'Efficacia:** Audit regolari e test delle misure adottate.
*   **Igiene Informatica:** Pratiche di cyber hygiene di base e formazione continua per tutto il personale.
*   **Crittografia:** Politiche sull'uso della crittografia e cifratura dei dati.
*   **Controllo Accessi:** Sicurezza delle risorse umane, politiche di accesso rigide e gestione degli asset.
*   **Comunicazioni Sicure:** Uso obbligatorio di autenticazione a piu' fattori (MFA) e sistemi di comunicazione sicuri.

### 3. Segnalazione Incidenti (Art. 23)
Tempistiche rigorose per notificare il CSIRT nazionale di qualsiasi incidente significativo:
*   **Preallarme 24h:** Notifica iniziale dell'incidente e potenziale impatto transfrontaliero.
*   **Notifica 72h:** Aggiornamento completo, valutazione della severita' e indicatori di compromissione.
*   **Relazione Intermedia:** Su specifica richiesta del CSIRT.
*   **Relazione Finale a 30 Giorni:** Report dettagliato con root cause, misure di mitigazione adottate e impatto residuo.",
            "general"
        ),
    ]
}
