pub fn entries() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "How much is the NIS2 fine?",
            "Art. 34: Essential entities face up to 10,000,000 EUR or 2% of worldwide turnover (whichever is higher). Important entities face up to 7,000,000 EUR or 1.4% of turnover.",
            "sanctions"
        ),
        (
            "What happens if we ignore NIS2?",
            "Art. 32-34: Competent authorities can impose fines (up to 10M EUR / 2% turnover), suspend certifications, temporarily ban executives from management roles, and mandate corrective measures with deadlines.",
            "sanctions"
        ),
        (
            "Quanto e' la multa NIS2?",
            "Art. 34: Entita essenziali fino a 10.000.000 EUR o 2% del fatturato mondiale (il piu alto). Entita importanti fino a 7.000.000 EUR o 1,4% del fatturato.",
            "sanctions"
        ),
        (
            "Cosa succede se ignoriamo NIS2?",
            "Art. 32-34: Le autorita competenti possono imporre sanzioni (fino a 10M EUR / 2% fatturato), sospendere certificazioni, vietare temporaneamente funzioni dirigenziali e imporre misure correttive con scadenze.",
            "sanctions"
        ),
        (
            "Possono sospenderci le operazioni?",
            "Art. 32(5) consente alle autorita competenti di sospendere temporaneamente certificazioni o autorizzazioni delle entita essenziali per non conformita. Va oltre le sanzioni pecuniarie.",
            "sanctions"
        ),
        (
            "I dirigenti possono essere sospesi?",
            "Art. 32(5)(b) consente alle autorita competenti di richiedere il divieto temporaneo di funzioni dirigenziali per le persone fisiche responsabili della non conformita nelle entita essenziali.",
            "sanctions"
        ),
        (
            "Rischio la galera per NIS2?",
            "NIS2 non prevede direttamente la detenzione, ma Art. 20(1) stabilisce responsabilita personale. La responsabilita penale dipende dal recepimento nazionale. Art. 32(5)(b) consente il divieto di funzioni dirigenziali e sanzioni fino a 10M EUR.",
            "sanctions"
        ),
        (
            "Quanto costa la non conformita NIS2?",
            "Art. 34: entita essenziali fino a 10M EUR o 2% fatturato mondiale. Entita importanti fino a 7M EUR o 1.4% fatturato. Si applica l'importo maggiore. Piu sanzioni accessorie (sospensione, divieto dirigenziale).",
            "sanctions"
        ),
        (
            "Possono sospendere le nostre operazioni?",
            "Art. 32(5): le autorita competenti possono sospendere temporaneamente certificazioni o autorizzazioni di entita essenziali in caso di non conformita. Va oltre le sanzioni finanziarie.",
            "sanctions"
        ),
        (
            "I dirigenti possono essere sanzionati personalmente?",
            "Art. 32(5)(b): le autorita possono richiedere il divieto temporaneo di funzioni dirigenziali per le persone fisiche responsabili della non conformita nelle entita essenziali.",
            "sanctions"
        ),
        (
            "Quali sono le differenze sanzionatorie tra essenziali e importanti?",
            "Essenziali (Art. 32): audit proattivi, sanzioni fino a 10M/2%. Importanti (Art. 33): audit ex-post, sanzioni fino a 7M/1.4%. Le entita essenziali hanno obblighi di vigilanza piu stringenti.",
            "sanctions"
        ),
        (
            "Le sanzioni NIS2 sono gia applicabili?",
            "Le sanzioni si applicano dalla data di recepimento nazionale. Art. 41 richiedeva recepimento entro 17 ottobre 2024. Verificare lo stato della legislazione nazionale presso l'autorita competente (ACN in Italia).",
            "sanctions"
        ),
        (
            "Qual e' il regime sanzionatorio completo della NIS2?",
            "Il regime sanzionatorio NIS2 e' articolato su piu livelli: (1) Sanzioni pecuniarie — Entita essenziali (Art. 34(4)): massimo 10.000.000 EUR o 2% del fatturato mondiale annuo, il maggiore dei due. Entita importanti (Art. 34(5)): massimo 7.000.000 EUR o 1.4% del fatturato mondiale annuo. (2) Misure di enforcement Art. 32 (essenziali) — le autorita possono: emettere avvertimenti, adottare istruzioni vincolanti, ordinare audit di sicurezza a spese dell'entita, ordinare la notifica ai clienti, sospendere temporaneamente certificazioni o autorizzazioni. (3) Responsabilita personale Art. 32(5)(b) — le autorita possono richiedere il divieto temporaneo di esercitare funzioni dirigenziali per le persone fisiche ritenute responsabili. (4) Differenze tra essenziali e importanti — le entita essenziali sono soggette a vigilanza proattiva (audit preventivi), le importanti a vigilanza ex-post (solo dopo segnalazione o incidente). Le sanzioni accessorie (sospensione, divieto dirigenziale) si applicano solo alle essenziali. (5) Principio di proporzionalita — le sanzioni devono essere effettive, proporzionate e dissuasive, tenendo conto della gravita della violazione, della durata, degli incidenti precedenti.",
            "sanctions"
        ),
        (
            "Do small companies need to comply with NIS2?",
            "Art. 2: NIS2 applies to entities in Annex I/II sectors with 50+ employees OR 10M+ EUR revenue. Micro and small enterprises are generally exempt unless they operate critical infrastructure (e.g., DNS, TLD registries).",
            "applicability"
        ),
        (
            "Le piccole aziende devono rispettare NIS2?",
            "Art. 2: NIS2 si applica a entita nei settori Allegato I/II con 50+ dipendenti O 10M+ EUR di ricavi. Micro e piccole imprese sono generalmente esenti salvo infrastrutture critiche (es. DNS, registri TLD).",
            "applicability"
        ),
        (
            "La mia azienda rientra nella NIS2?",
            "Dipende dal settore (Allegato I o II della Direttiva 2022/2555) e dalla dimensione (50+ dipendenti o 10M+ EUR fatturato). Verifica il tuo codice ATECO rispetto ai settori elencati. Art. 2 definisce i criteri.",
            "applicability"
        ),
        (
            "Siamo un ospedale, NIS2 ci riguarda?",
            "Si. La sanita e' elencata nell'Allegato I. Ospedali e fornitori sanitari che superano la soglia dimensionale (50+ dipendenti o 10M+ EUR) sono classificati come entita essenziali.",
            "applicability"
        ),
        (
            "Siamo una PMI, dobbiamo preoccuparci?",
            "Solo se operate in un settore Allegato I o II E superate la soglia (50+ dipendenti O 10M+ EUR ricavi). La maggior parte delle PMI sotto entrambe le soglie e' fuori ambito, salvo infrastrutture critiche.",
            "applicability"
        ),
        (
            "Siamo una startup, NIS2 si applica?",
            "Solo se opera in un settore Allegato I o II E supera la soglia (50+ dipendenti O 10M+ EUR fatturato). La maggior parte delle startup in fase iniziale e' sotto entrambe le soglie ed e' fuori ambito.",
            "applicability"
        ),
        (
            "Siamo una PA, NIS2 si applica?",
            "Si. Le pubbliche amministrazioni sono elencate nell'Allegato I e classificate come entita essenziali indipendentemente dalla dimensione (Art. 2(2)). Gli enti centrali sono in ambito; il trattamento degli enti locali varia per stato membro.",
            "applicability"
        ),
        (
            "Siamo un'universita, rientiramo?",
            "Le organizzazioni di ricerca sono nell'Allegato II. Le universita che superano la soglia dimensionale sono entita importanti. Quelle sotto soglia sono fuori ambito.",
            "applicability"
        ),
        (
            "NIS2 si applica al cloud?",
            "Si. I fornitori di servizi cloud sono elencati nell'Allegato I (infrastruttura digitale) e classificati come entita essenziali. Sono soggetti a tutti gli obblighi Art. 20, 21 e 23.",
            "applicability"
        ),
        (
            "NIS2 si applica al DNS?",
            "Si. I fornitori di servizi DNS sono esplicitamente elencati nell'Allegato I sotto infrastruttura digitale. Sono classificati come entita essenziali soggette a tutti gli obblighi NIS2.",
            "applicability"
        ),
        (
            "Quali settori rientrano nell'Allegato I?",
            "Allegato I (entita essenziali): energia, trasporti, banche, infrastrutture finanziarie, sanita, acqua potabile, acque reflue, infrastruttura digitale, gestione servizi ICT B2B, pubblica amministrazione, spazio.",
            "applicability"
        ),
        (
            "Quali settori rientrano nell'Allegato II?",
            "Allegato II (entita importanti): servizi postali, gestione rifiuti, fabbricazione/distribuzione prodotti chimici, produzione/distribuzione alimenti, fabbricazione dispositivi medici/elettronici/macchinari/veicoli, fornitori servizi digitali, ricerca.",
            "applicability"
        ),
        (
            "Siamo una PMI, dobbiamo preoccuparci della NIS2?",
            "Solo se operate in un settore Allegato I o II E superate la soglia (50+ dipendenti O 10M+ EUR ricavi). La maggior parte delle PMI sono sotto entrambe le soglie e fuori ambito, salvo infrastrutture critiche.",
            "applicability"
        ),
        (
            "Gli ospedali sono soggetti alla NIS2?",
            "Si. La sanita e' in Allegato I. Ospedali e strutture sanitarie che superano la soglia dimensionale (50+ dipendenti o 10M+ EUR) sono classificati come entita essenziali con tutti gli obblighi.",
            "applicability"
        ),
        (
            "Le universita sono soggette alla NIS2?",
            "Le organizzazioni di ricerca sono in Allegato II. Universita che superano la soglia dimensionale sono classificate come entita importanti. Quelle sotto soglia sono fuori ambito.",
            "applicability"
        ),
        (
            "Una startup e' soggetta alla NIS2?",
            "Solo se opera in un settore Allegato I o II E supera la soglia dimensionale (50+ dipendenti o 10M+ EUR fatturato). La maggior parte delle startup in fase iniziale sono sotto soglia e fuori ambito.",
            "applicability"
        ),
        (
            "La pubblica amministrazione rientra nella NIS2?",
            "Si. La PA e' in Allegato I ed e' classificata come essenziale indipendentemente dalla dimensione (Art. 2(2)). Enti centrali sono in scope; il trattamento degli enti locali varia per stato membro.",
            "applicability"
        ),
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
    ]
}
