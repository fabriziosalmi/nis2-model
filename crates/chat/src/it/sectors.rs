pub fn entries() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "Cosa prevede la NIS2 per il settore della Sanità (Ospedali, Cliniche)?",
            "Il settore sanitario è considerato ad Altissima Criticità (Allegato I).

<span class='ans-header'><span class='ans-art'>Settore Sanità</span> <span class='ans-intro'>Soggetti Essenziali</span></span>
<span class='ans-body'>Ospedali, cliniche, laboratori di analisi e produttori di dispositivi medici critici sono in prima linea.</span>

**Sfide Principali e Soluzioni:**
- **Apparati Elettromedicali (IoMT):** Spesso usano OS obsoleti non patchabili (es. Windows XP o 7 su macchine per la TAC). Devono essere isolati in VLAN dedicate e protetti tramite firewall interni (Microsegmentazione).
- **Dati Sanitari:** La crittografia a riposo e in transito è obbligatoria. Un data breach qui viola contemporaneamente NIS2 e GDPR (con multe combinate astronomiche).
- **Ransomware:** Gli ospedali sono bersagli frequenti. È vitale avere backup immutabili (air-gapped) e piani di continuità operativa clinica (carta e penna pronti).",
            "applicability_impl"
        ),
        (
            "Quali sono gli obblighi per il settore dell'Energia?",
            "L'Energia (produzione, distribuzione, trasmissione) è un settore di importanza vitale (Allegato I). Un blackout causato da un cyber-attacco ha ripercussioni su scala nazionale.

**Focus Operativi:**
- **Sistemi OT/ICS/SCADA:** La priorità assoluta è la separazione netta (Air Gap logico o DMZ forte) tra reti IT aziendali (dove gli utenti aprono le email) e reti OT (che gestiscono le turbine).
- **Monitoraggio Anomalie OT:** Implementare sensori passivi di rete (es. Nozomi Networks, Claroty) per rilevare comandi anomali inviati ai PLC senza impattare la latenza della produzione.
- **Micro-Grid e Rinnovabili:** I sistemi fotovoltaici o eolici distribuiti (IoT) creano una superficie d'attacco enorme e necessitano di autenticazione forte (Zero Trust) a livello di singolo inverter.",
            "applicability_impl"
        ),
        (
            "Come impatta la NIS2 sulle aziende di Trasporti e Logistica?",
            "I trasporti (aereo, ferroviario, per via d'acqua e stradale) sono nell'Allegato I.

La logistica integrata è il sistema nervoso dell'economia moderna.
- **Porti e Aeroporti:** I sistemi di smistamento bagagli, controllo radar e gestione cargo devono avere piani di disaster recovery rapidissimi (RTO di poche ore).
- **Aziende di Autotrasporto e Flotte:** Le flotte connesse (GPS, telemetria) pongono enormi rischi. Se il software di routing viene compromesso o alterato, le merci si fermano. È richiesta l'MFA obbligatoria per tutti gli accessi remoti ai portali logistici da parte degli autisti e dei dispatcher.",
            "applicability_impl"
        ),
        (
            "Banche e settore Finanziario: devono seguire la NIS2 o il DORA?",
            "Per il settore finanziario vige la Lex Specialis: la **Direttiva DORA** (Digital Operational Resilience Act).

Il DORA è una regolamentazione molto più prescrittiva della NIS2 (scende fino ai test di penetrazione threat-led - TLPT - e ai contratti con i cloud provider).
Tuttavia, le banche e le istituzioni finanziarie rimangono nel perimetro dell'Allegato I della NIS2. Gli Stati Membri, in fase di recepimento, garantiranno che gli adempimenti DORA coprano e soddisfino automaticamente i requisiti NIS2 per evitare doppie rendicontazioni agli CSIRT.",
            "applicability_impl"
        ),
        (
            "I provider di servizi Cloud (CSP) e i Data Center come si posizionano?",
            "Rientrano nelle **Infrastrutture Digitali** (Allegato I) e sono automaticamente **Soggetti Essenziali**, indipendentemente dalle dimensioni se forniscono servizi a livello di sistema critico.

Le aziende clienti scaricheranno su di loro la responsabilità tramite la supply chain (Art. 21(2)(d)). I CSP e i Data Center devono:
- Fornire reportistica standardizzata dettagliata (SOC 2 Type II, ISO 27001).
- Garantire la segregazione logica rigorosa (Tenant Isolation) negli ambienti multitenant.
- Assicurare ridondanza energetica e di rete (sistemi UPS/Generatori).
- Offrire servizi di crittografia Bring-Your-Own-Key (BYOK) per tranquillizzare i clienti sull'accesso ai dati.",
            "applicability_impl"
        ),
        (
            "E per le Pubbliche Amministrazioni? I Comuni sono soggetti alla direttiva?",
            "Sì. A differenza della NIS1, la NIS2 copre la Pubblica Amministrazione a livello centrale e regionale (Allegato I, Enti della PA).

Gli Stati Membri hanno la facoltà di estenderla anche ai governi locali (Comuni) se gestiscono servizi critici.
- **Transizione Digitale:** I comuni e gli enti locali devono adottare servizi cloud qualificati, imporre l'MFA a tutti i dipendenti e formare il personale sul phishing.
- **Rischio Reale:** Le PA locali sono oggi i bersagli più frequenti e vulnerabili di ransomware a causa dei budget IT limitati. La priorità assoluta per le PA è la **messa in sicurezza dei backup (offline/immutabili)** e la migrazione da vecchi server on-premise verso infrastrutture SaaS gestite a livello nazionale.",
            "applicability_impl"
        ),
        (
            "Le aziende dell'Acqua Potabile e gestione Rifiuti sono coperte?",
            "Assolutamente sì. L'acqua potabile è nell'Allegato I (Settori Altamente Critici), mentre la gestione dei rifiuti è nell'Allegato II (Altri settori critici).

Un attacco a un acquedotto (es. per alterare i livelli chimici nei depuratori) può avere conseguenze mortali, come visto nell'incidente di Oldsmar in Florida nel 2021.
- **Controlli Obbligatori:** Accesso remoto ai PLC e sistemi SCADA solo tramite VPN fortemente cifrate, con MFA obbligatoria e restrizione degli IP sorgenti. È vitale il cambio delle credenziali di default (che storicamente nei sistemi idrici sono lasciate su admin/admin).",
            "applicability_impl"
        ),
        (
            "Le aziende manifatturiere (Manufacturing / Industria 4.0) rientrano nella NIS2?",
            "Sì, la Fabbricazione di prodotti chimici, veicoli, computer, macchinari, apparecchiature elettriche e dispositivi medici rientra nell'Allegato II.

L'Industria 4.0 si basa sulla connessione dei macchinari di produzione a Internet (IoT).
- **Rischio di Interruzione:** Un blocco della catena di fornitura costa milioni al giorno (es. settore Automotive).
- **Adeguamento Tecnico:** Oltre al backup, queste aziende devono mappare rigorosamente l'inventario hardware industriale, eseguire segmentazione di rete (IT vs OT), e obbligare i fornitori che fanno teleassistenza remota ai torni/frese a usare portali Zero Trust e sessioni registrate (Privileged Access Management).",
            "applicability_impl"
        ),
        (
            "Cosa c'entra lo Spazio con la NIS2?",
            "Il settore spaziale è stato inserito nell'Allegato I come Infrastruttura di importanza vitale.

Copre gli operatori di infrastrutture terrestri e i gestori di reti satellitari (comunicazioni, navigazione GPS/Galileo, osservazione terrestre).
- **Vulnerabilità:** I satelliti si affidano a link radio che possono essere hackerati (spoofing, jamming) e centri di controllo terrestri che sono spesso sistemi IT tradizionali.
- **Requisiti:** Gli operatori devono garantire la resilienza crittografica delle comunicazioni terra-spazio, protezione estrema contro attacchi DDoS e controllo rigoroso della supply chain (es. componenti hardware cinesi o non certificati).",
            "applicability_impl"
        )
    ]
}
