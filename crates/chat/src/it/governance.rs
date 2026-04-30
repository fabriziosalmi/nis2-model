pub fn entries() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "Come presentare la cybersecurity al CDA",
            "Art. 20(1)(2): Presentare trimestralmente: (1) dashboard postura rischio, (2) vulnerabilita critiche aperte, (3) riepilogo incidenti, (4) stato conformita vs Art. 21, (5) richieste investimento. Linguaggio business, non tecnico.",
            "governance_impl"
        ),
        (
            "Quale formazione serve al CDA",
            "Art. 20(2): Annuale minimo. Coprire: obblighi NIS2 e responsabilita personale, panorama minacce, postura rischio aziendale, ruoli nella risposta incidenti, consapevolezza social engineering. Istruttori esterni raccomandati.",
            "governance_impl"
        ),
        (
            "Come strutturare un team di sicurezza",
            "Art. 20(1): Funzioni minime: CISO (riporta al CDA), security operations (monitoraggio), security engineering (architettura), GRC (compliance/rischio). Piccole aziende: outsourcing SOC, mantenere funzione CISO interna.",
            "governance_impl"
        ),
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
        (
            "La ISO 27001 basta per essere conformi alla NIS2?",
            "La ISO 27001 è un'ottima base di partenza (circa l'80% della copertura dei processi), ma non garantisce automaticamente la conformità NIS2.

**Differenze chiave:**
- **Ambito (Scope):** Nella ISO puoi definire un perimetro ristretto (es. solo un reparto). La NIS2 guarda all'azienda nel suo complesso se eroga il servizio essenziale.
- **Segnalazione Incidenti:** La ISO richiede di gestirli; la NIS2 impone leggi stringenti con il governo (entro 24h).
- **Supply Chain:** La NIS2 è molto più rigorosa sulle garanzie da richiedere ai fornitori diretti.
- **Sanzioni:** La ISO al massimo ti toglie il certificato. La NIS2 impone multe milionarie e responsabilità penali per il board.",
            "governance_impl"
        ),
        (
            "Can the CEO go to jail for NIS2?",
            "Art. 20(1) establishes personal responsibility of management bodies. Art. 32(5)(b) allows temporary prohibition of management functions. Criminal liability depends on national transposition. Civil liability and fines are explicit.",
            "governance"
        ),
        (
            "Il CEO rischia per NIS2?",
            "Art. 20(1) stabilisce la responsabilita personale degli organi direttivi. Art. 32(5)(b) consente il divieto temporaneo di funzioni dirigenziali. La responsabilita penale dipende dal recepimento nazionale.",
            "governance"
        ),
        (
            "Serve un CISO?",
            "NIS2 non impone il titolo CISO, ma Art. 20(1) richiede supervisione delle misure di cybersecurity da parte degli organi direttivi. Una funzione di sicurezza designata che riporta al CDA e' implicita.",
            "governance"
        ),
        (
            "Chi e' responsabile della conformita NIS2?",
            "Art. 20(1): l'organo direttivo (CDA, management esecutivo) approva le misure di cybersecurity, sovraintende l'attuazione e puo essere ritenuto personalmente responsabile delle violazioni.",
            "governance"
        ),
        (
            "Serve un DPO per NIS2?",
            "NIS2 non richiede un DPO (quello e' GDPR). Richiede pero che gli organi direttivi supervisionino la cybersecurity (Art. 20). Un CISO o funzione equivalente e' la figura di riferimento per NIS2.",
            "governance"
        ),
        (
            "Il CDA deve fare formazione?",
            "Art. 20(2): si, formazione obbligatoria in cybersecurity per gli organi direttivi. Annuale minimo. Copre: obblighi NIS2, responsabilita personale, panorama minacce, postura rischio aziendale.",
            "governance"
        ),
        (
            "Possono auditarci?",
            "Art. 32 (essenziali) e Art. 33 (importanti): le autorita competenti possono condurre audit, ispezioni e visite in loco. Le entita essenziali possono essere soggette ad audit regolari; le importanti ex-post.",
            "governance"
        ),
        (
            "Servono audit interni?",
            "Art. 21(2)(f) richiede la valutazione dell'efficacia delle misure di cybersecurity. Audit interni, revisioni di sicurezza e valutazioni di maturita sono metodi standard per questo obbligo.",
            "governance"
        ),
        (
            "Basta ISO 27001 per NIS2?",
            "ISO 27001 copre gran parte degli obblighi Art. 21 ma non tutto. Mancano: soglie di notifica incidente specifiche (Art. 23, 24h/72h), obblighi di governance CDA (Art. 20), registrazione autorita (Art. 27). ISO 27001 e' un ottimo punto di partenza ma non sufficiente.",
            "governance"
        ),
        (
            "Serve un consulente per NIS2?",
            "Non obbligatorio ma raccomandato per gap analysis e piano di remediation. Criteri di scelta: esperienza NIS2/ISO27001, conoscenza del settore, referenze, capacita di supporto audit. Diffidare da chi promette conformita istantanea.",
            "governance"
        ),
        (
            "Come convinco il management a investire in NIS2?",
            "Leve: (1) sanzioni fino a 10M EUR o 2% fatturato (Art. 34), (2) responsabilita personale CDA (Art. 20), (3) possibile sospensione dirigenti (Art. 32(5)(b)), (4) obbligo legale (D.Lgs. 138/2024), (5) reputazione, (6) requisiti clienti/fornitori, (7) assicurazione cyber piu favorevole.",
            "governance"
        ),
        (
            "Quale formazione serve al CDA?",
            "Art. 20(2): formazione specifica in cybersecurity per tutti i membri del CDA. Contenuti: panoramica NIS2, responsabilita personali, gestione rischi cyber, risposta incidenti, supervisione misure di sicurezza.",
            "governance"
        ),
        (
            "Chi nomina il responsabile della sicurezza?",
            "Art. 20(1): gli organi direttivi sono responsabili. La nomina del CISO o responsabile sicurezza e' una decisione del CDA. Il ruolo deve avere accesso diretto al CDA e risorse adeguate.",
            "governance"
        ),
        (
            "Il CDA puo delegare la responsabilita NIS2?",
            "Art. 20(1): il CDA puo delegare l'attuazione ma non la responsabilita. Resta responsabile dell'approvazione, supervisione e formazione. La delega deve essere documentata con reporting periodico.",
            "governance"
        ),
        (
            "Serve un comitato di sicurezza informatica?",
            "Art. 20(1): per entita complesse, un comitato di sicurezza che riporta al CDA e' prassi raccomandata. Composizione: CISO, IT, legale, compliance, operations. Riunioni almeno trimestrali.",
            "governance"
        ),
        (
            "Quali sono le responsabilita esatte del CDA per la NIS2?",
            "Art. 20 NIS2 definisce tre obblighi specifici per gli organi direttivi: (1) Approvazione — il CDA deve formalmente approvare le misure di gestione del rischio di cybersecurity adottate dall'entita ai sensi dell'Art. 21. Non basta una delega al CISO. (2) Supervisione — il CDA deve sovraintendere all'attuazione delle misure approvate e puo' essere ritenuto responsabile in caso di violazioni. La responsabilita e' personale. (3) Formazione — Art. 20(2) richiede che i membri degli organi direttivi seguano formazione specifica in cybersecurity per acquisire conoscenze e competenze sufficienti a comprendere i rischi e valutare le pratiche di sicurezza. La formazione deve essere periodica, non una tantum. Art. 20(1) stabilisce inoltre che la responsabilita non puo' essere delegata: il CDA puo' delegare l'attuazione operativa ma mantiene la responsabilita della supervisione.",
            "governance"
        ),
        (
            "Come strutturare la governance della sicurezza informatica?",
            "Una governance efficace per NIS2 richiede: (1) Livello strategico — comitato di sicurezza che riporta direttamente al CDA, composto da CISO, CTO, legale, compliance, HR. Riunioni almeno trimestrali con reporting su rischi, incidenti, conformita. (2) Livello tattico — CISO con accesso diretto al CDA, risorse adeguate, autorita decisionale su investimenti di sicurezza. Team dedicato di sicurezza con competenze in governance, operations, incident response. (3) Livello operativo — procedure documentate per ogni ambito dell'Art. 21, KPI misurabili (MTTD, MTTR, copertura patching), audit interni semestrali, penetration test annuali. (4) Documentazione — policy framework allineato a ISO 27001, registro rischi aggiornato, verbali delle riunioni del comitato, evidenze di formazione del CDA.",
            "governance"
        ),
        (
            "Quale documentazione richiede NIS2?",
            "Art. 21 richiede policy documentate per: (a) analisi rischi, (c) continuita operativa, (d) supply chain, (h) crittografia, (i) controllo accessi. Art. 20 richiede approvazione documentata del CDA. Art. 23 richiede registri notifiche incidenti.",
            "documentation"
        ),
        (
            "Serve una policy di cybersecurity?",
            "Si. Art. 21(2)(a) richiede esplicitamente 'politiche di analisi dei rischi e sicurezza dei sistemi informatici'. E' il documento fondamentale da cui derivano tutte le altre misure Art. 21.",
            "documentation"
        ),
        (
            "Serve un registro dei trattamenti per NIS2?",
            "NIS2 non richiede un registro dei trattamenti (quello e' GDPR Art. 30). NIS2 richiede pero: registri notifiche incidenti (Art. 23), documentazione policy (Art. 21), e registri di formazione (Art. 20(2)). Mantenerli tutti per audit.",
            "documentation"
        ),
        (
            "Quali documenti richiede la NIS2?",
            "Art. 21 richiede policy documentate per: (a) analisi rischi, (c) continuita operativa, (d) supply chain, (h) crittografia, (i) controllo accessi. Art. 20 richiede approvazione documentata del CDA. Art. 23 richiede registri notifiche.",
            "documentation"
        ),
        (
            "Serve una policy di sicurezza informatica?",
            "Si. Art. 21(2)(a) richiede esplicitamente policy di analisi rischi e sicurezza dei sistemi. E' il documento fondamentale da cui derivano tutte le altre misure dell'Art. 21.",
            "documentation"
        ),
        (
            "Come documentare la conformita NIS2?",
            "Mantenere: policy di sicurezza, registro rischi, piano di risposta incidenti, piano BCP/DR, registro fornitori, procedure di controllo accessi, log di formazione, verbali CDA, risultati audit.",
            "documentation"
        ),
        (
            "Serve un registro degli incidenti?",
            "Art. 23 e Art. 21(2)(b): documentare ogni incidente con timeline, impatto, azioni di contenimento, root cause analysis, lezioni apprese. Conservare per almeno 5 anni per fini di audit.",
            "documentation"
        ),
        (
            "Come strutturare le policy di sicurezza?",
            "Struttura raccomandata: ambito, responsabilita, classificazione asset, gestione rischi, misure tecniche, procedure operative, formazione, audit, revisione. Allineare alla ISO 27001 Annex A per completezza.",
            "documentation"
        ),
        (
            "Quale documentazione specifica serve per la conformita NIS2?",
            "Documentazione richiesta da Art. 20, 21, 23: (1) Policy di sicurezza — documento approvato dal CDA che definisce ambito, principi, ruoli e responsabilita della sicurezza. Allineato a ISO 27001. (2) Registro dei rischi — catalogo di tutti i rischi identificati con: probabilita, impatto, misure di mitigazione, rischio residuo, responsabile, data revisione. (3) Piani operativi — piano di risposta incidenti, piano di continuita operativa, piano di disaster recovery, piano di gestione crisi. Tutti documentati, approvati, testati. (4) Policy specifiche Art. 21 — controllo accessi, crittografia, supply chain, vulnerability management, change management, BYOD, classificazione dati, backup. (5) Registri — registro incidenti (timeline, impatto, azioni), registro notifiche CSIRT, registro audit e pentest, registro formazione CDA e dipendenti. (6) Contratti — clausole di sicurezza con fornitori, accordi di condivisione informazioni (Art. 29), SLA con KPI di sicurezza. (7) Evidenze — verbali riunioni comitato sicurezza, verbali approvazione CDA, certificati di formazione, report di audit, risultati pentest, log di revisione accessi. Conservare per almeno 5 anni.",
            "documentation"
        ),
        (
            "Come calcolare e giustificare il budget di cybersecurity per il board in ottica NIS2?",
            "Spesso il management considera la sicurezza un costo puro. La NIS2 cambia il paradigma (Art. 20): l'organo di vertice è responsabile.

**Strategia di giustificazione (CISO/IT Manager):**
1. **Analisi del Rischio Finanziario (FAIR):** Quantifica il rischio in Euro. Un fermo di 3 giorni quanto costa in mancato fatturato? E se si aggiunge la multa NIS2 (fino a 10M o 2% del fatturato)?
2. **Comparazione vs Costo di Mitigazione:** 'Spendere 50.000€ l'anno per EDR, Backup e MFA riduce l'esposizione di rischio da 5 Milioni a 100.000€'.
3. **Responsabilità Personale:** Ricordare educatamente al CDA che l'Art. 20 prevede sanzioni personali e sospensioni dall'incarico per negligenza. Di solito sblocca i budget molto velocemente.",
            "governance_impl"
        ),
        (
            "L'assicurazione Cyber (Cyber Insurance) sostituisce le misure della NIS2?",
            "Assolutamente no. Trasferire il rischio tramite una polizza assicurativa non elimina l'obbligo legale di applicare le misure (Art. 21).

Anzi, oggi le assicurazioni **pretendono** la conformità de facto (MFA su tutto, EDR, backup testati regolarmente) prima ancora di emettere la polizza. Se si verifica un incidente e l'assicurazione scopre che hai dichiarato il falso sui questionari di sicurezza (es. non avevi l'MFA sulla VPN), non paga il riscatto o i danni e annulla la polizza.",
            "risk_management"
        ),
        (
            "Come gestire il turn-over dei dipendenti per evitare rischi insider o data exfiltration?",
            "Il processo di off-boarding è fondamentale per la sicurezza (Art. 21(2)(i)). Molti incidenti nascono da ex-dipendenti arrabbiati con credenziali ancora attive.

**Procedure Operative Standard:**
1. Sospensione immediata (entro 1 ora dall'uscita) degli account Active Directory, Google Workspace, VPN.
2. Recupero documentato di tutti i dispositivi fisici (laptop, token RSA, smartphone, badge fisici).
3. Reset delle password condivise (es. account generici di amministrazione IT) a cui la persona aveva accesso.
L'automazione (Identity Lifecycle Management) che disabilita l'utente quando viene flaggato come 'Dismesso' nel software HR è il traguardo ideale.",
            "governance_impl"
        ),
        (
            "Auditing interno vs esterno: quante volte all'anno va fatto per essere in regola?",
            "La NIS2 richiede la 'valutazione dell'efficacia' (Art. 21(2)(f)). Non bastano controlli autoreferenziali.

**Programma Consigliato:**
- **Auditing Interno:** Continuo o trimestrale. Il team IT verifica lo stato dei backup, le metriche di patch (SLA) e gli account orfani.
- **Auditing Esterno (Indipendente):** Almeno annuale. Assumere una terza parte per eseguire un Penetration Test completo e un assessment ISO 27001/NIS2 garantisce che l'organo di vertice abbia una visione non viziata dello stato della sicurezza (evita l'effetto 'L'oste che dice che il vino è buono').",
            "governance_impl"
        ),
        (
            "Responsabilità dell'RSPP rispetto al CISO: come si intersecano sicurezza fisica e cyber?",
            "Un attacco cyber a un sistema OT industriale può fermare i robot e causare danni fisici ai lavoratori. Questo unisce le responsabilità del CISO (Cyber) e dell'RSPP (Sicurezza sul Lavoro).

Inoltre, la 'Sicurezza fisica e ambientale' (Art. 21(2)(e)) richiede che i server e i cavi di rete siano protetti da furti, incendi o manomissioni.
**Azione Pratica:** Istruire la guardia giurata/receptionist a non far entrare nessuno in sala macchine (tailgating) e mettere sistemi di videosorveglianza e controllo accessi sui rack IT. Un cavo staccato a mano è un incidente NIS2 tanto quanto un ransomware.",
            "governance_impl"
        )
    ]
}
