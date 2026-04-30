pub fn italian_massive() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        // SETTORI SPECIFICI
        (
            "Cosa prevede la NIS2 per il settore della Sanità (Ospedali, Cliniche)?",
            "Il settore sanitario è considerato ad Altissima Criticità (Allegato I).

<span class='ans-header'><span class='ans-art'>Settore Sanità</span> <span class='ans-intro'>Soggetti Essenziali</span></span>
<span class='ans-body'>Gli ospedali, le cliniche private (se superano le soglie dimensionali) e i laboratori di riferimento sono in prima linea.</span>

**Sfide Principali e Soluzioni:**
- **Apparati Elettromedicali (IoMT):** Spesso usano OS obsoleti (es. Windows XP). Devono essere isolati in VLAN dedicate e protetti tramite firewall interni (Microsegmentazione).
- **Dati Sanitari:** La crittografia a riposo e in transito è obbligatoria. Un data breach qui viola contemporaneamente NIS2 e GDPR.
- **Ransomware:** Gli ospedali sono bersagli frequenti. È vitale avere backup immutabili (air-gapped) e piani di continuità operativa clinica (carta e penna pronti).",
            "applicability_impl"
        ),
        (
            "Come impatta la NIS2 sulle aziende del settore Energia?",
            "L'Energia è un settore di importanza vitale (Allegato I). Un blackout causato da un attacco informatico ha ripercussioni nazionali.

**Focus Operativi:**
- **Sistemi OT/ICS/SCADA:** La priorità è la separazione netta (air gap o DMZ forte) tra reti IT aziendali e reti OT (Operational Technology).
- **Monitoraggio Anomalie:** Implementare sensori passivi di rete (es. Nozomi Networks, Claroty) per rilevare comandi anomali inviati ai PLC senza impattare la produzione.
- **Supply Chain:** I fornitori di turbine, inverter o software di controllo remoto sono vettori di attacco critici e vanno soggetti ad audit rigorosi.",
            "applicability_impl"
        ),
        (
            "Le aziende di Trasporti e Logistica sono coperte dalla NIS2?",
            "Sì, i trasporti (aereo, ferroviario, per via d'acqua e stradale) sono nell'Allegato I.

La logistica integrata è il sistema nervoso dell'economia.
- **Porti e Aeroporti:** I sistemi di smistamento bagagli e gestione cargo devono avere piani di disaster recovery rapidissimi (RTO di poche ore).
- **Aziende di Autotrasporto:** Flotte connesse (GPS, telemetria) pongono rischi. Se il software di routing viene compromesso, le merci si fermano. È richiesta l'MFA obbligatoria per tutti gli accessi remoti ai portali logistici.",
            "applicability_impl"
        ),
        (
            "I provider di servizi Cloud (CSP) e i Data Center come si posizionano?",
            "Rientrano nelle **Infrastrutture Digitali** (Allegato I) e sono automaticamente **Soggetti Essenziali**, indipendentemente dal loro fatturato o numero di dipendenti se forniscono servizi critici.

Le aziende clienti scaricheranno su di loro la responsabilità tramite la supply chain (Art. 21(2)(d)). I CSP devono:
- Fornire reportistica dettagliata (SOC 2 Type II, ISO 27001).
- Garantire la segregazione logica dei tenant.
- Offrire crittografia Bring-Your-Own-Key (BYOK).",
            "applicability_impl"
        ),
        (
            "E per le Pubbliche Amministrazioni? I Comuni sono soggetti alla direttiva?",
            "Sì. A differenza della NIS1, la NIS2 copre la Pubblica Amministrazione a livello centrale e regionale.

Gli Stati Membri possono estenderla anche ai governi locali (Comuni).
- **Transizione Digitale:** I comuni devono adottare servizi cloud qualificati, imporre l'MFA ai dipendenti e formare il personale sul phishing.
- **Rischio:** Le PA sono bersagli frequenti di ransomware a causa di budget limitati. La priorità assoluta sono i **backup offline** e la dismissione dei server on-premise obsoleti.",
            "applicability_impl"
        ),

        // APPROFONDIMENTI TECNICI (ART 21)
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
            "Quali strumenti open-source posso usare per un SIEM/SOC interno?",
            "Se non puoi esternalizzare a un SOC 24/7, puoi implementare soluzioni open-source o gratuite per il log management e il rilevamento (Art. 21(2)(f)).

**Stack consigliato:**
- **Wazuh:** Eccellente piattaforma open-source XDR/SIEM. Offre agenti per Windows/Linux/macOS, rileva malware, controlla la compliance e offre FIM (File Integrity Monitoring).
- **Elastic Stack (ELK):** Per la raccolta massiva di log di firewall e Active Directory.
- **TheHive / Cortex:** Piattaforme per l'Incident Response e l'automazione dei playbook.",
            "detection_impl"
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
            "Cos'è la crittografia E2EE e quando è obbligatoria?",
            "L'Art. 21(2)(h) incoraggia l'uso della crittografia, inclusa quella end-to-end (E2EE), dove opportuno.

E2EE significa che solo il mittente e il destinatario hanno le chiavi; il server intermedio non può leggere i dati (es. Signal, WhatsApp, ProtonMail).
**Dove usarla in azienda:**
- Per le comunicazioni dei dirigenti apicali o team di R&D su progetti segreti.
- Nei canali di comunicazione di emergenza (out-of-band) da usare quando la rete aziendale (Exchange/Teams) è compromessa da un ransomware.",
            "encryption_impl"
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

        // GOVERNANCE E RESPONSABILITA'
        (
            "Cosa rischia il CEO o il Consiglio di Amministrazione (CDA) con la NIS2?",
            "La NIS2 introduce una responsabilità personale per gli organi di vertice (Art. 20).

<span class='ans-header'><span class='ans-art'>Art. 20</span> <span class='ans-intro'>Responsabilità Organi di Vertice</span></span>
<span class='ans-body'>Il management non può più delegare tutto al reparto IT nascondendo la testa sotto la sabbia.</span>

**Conseguenze:**
- Il management **deve approvare** le misure di sicurezza.
- Deve **seguire una formazione** specifica sui rischi cyber.
- In caso di colpa grave, le autorità competenti possono sospendere temporaneamente il CEO o i dirigenti (nei Soggetti Essenziali) dal loro incarico e ritenerli personalmente responsabili per le sanzioni imposte alla società.",
            "governance_impl"
        ),
        (
            "Quanto deve durare la formazione cyber per i dipendenti e chi la deve fare?",
            "L'igiene informatica e la formazione sono pilastri della direttiva. L'anello debole è spesso l'utente (Phishing).

**Best Practices Operative:**
- La formazione deve essere **continua** (es. video brevi da 5 minuti ogni mese), non un noioso corso di 4 ore all'anno.
- **Simulazioni Phishing:** Eseguire campagne di finto phishing a sorpresa per testare l'attenzione dei dipendenti (usando tool come GoPhish o KnowBe4).
- Tutti devono partecipare, ma il **management e gli amministratori IT** devono seguire moduli avanzati (su social engineering, whaling e sicurezza degli account privilegiati).",
            "governance_impl"
        ),
        (
            "Serve assumere un CISO (Chief Information Security Officer) dedicato?",
            "La NIS2 non richiede per forza un ruolo a tempo pieno nominato CISO, ma richiede che ci sia un chiaro governo della sicurezza.

Nelle aziende medie, questo ruolo può essere assunto dall'IT Manager o, meglio ancora, può essere esternalizzato (CISO-as-a-Service o vCISO).
Tuttavia, deve esistere una figura che fa da ponte tra l'operatività tecnica e il board aziendale per riportare i livelli di rischio in modo comprensibile al management.",
            "governance_impl"
        ),

        // INCIDENT RESPONSE AVANZATO
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

        // CASI MISTI & SUPPLY CHAIN
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
            "Sviluppiamo software custom internamente. La NIS2 ci impone regole su questo?",
            "Assolutamente sì. L'Art. 21(2)(i) menziona 'pratiche di base di igiene informatica e formazione nel campo della cibersicurezza'. La sicurezza nel ciclo di vita dello sviluppo (SDLC) è implicita.

**Pratiche di Sviluppo Sicuro (DevSecOps):**
- **Analisi Statica (SAST):** Scansionare il codice sorgente per trovare vulnerabilità (es. SQL Injection) prima del rilascio (usando tool come SonarQube).
- **Analisi delle Dipendenze (SCA):** Controllare le librerie open-source utilizzate nel progetto per capire se hanno CVE note (es. tramite Dependabot o OWASP Dependency-Check).
- **Gestione Segreti:** Non hardcodare MAI password o chiavi API nel codice sorgente (usare Azure Key Vault o HashiCorp Vault).",
            "development_impl"
        ),
        (
            "La ISO 27001 basta per essere conformi alla NIS2?",
            "La ISO 27001 è un'ottima base di partenza (circa l'80% della copertura dei processi), ma non garantisce automaticamente la conformità NIS2.

**Differenze chiave:**
- **Ambito (Scope):** Nella ISO puoi definire un perimetro ristretto (es. solo un reparto). La NIS2 guarda all'azienda nel suo complesso se eroga il servizio essenziale.
- **Segnalazione Incidenti:** La ISO richiede di gestirli; la NIS2 impone leggi stringenti con il governo (entro 24h).
- **Supply Chain:** La NIS2 è molto più rigorosa sulle garanzie da richiedere ai fornitori diretti.
- **Sanzioni:** La ISO al massimo ti toglie il certificato. La NIS2 impone multe milionarie e responsabilità penali per il board.",
            "governance_impl"
        )
    ]
}
