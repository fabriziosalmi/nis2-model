const fs = require('fs');
const path = require('path');

const datasetPath = path.join(__dirname, 'docs', 'public', 'dataset.json');
let dataset = JSON.parse(fs.readFileSync(datasetPath, 'utf8'));

// Nuovi record conversazionali in Italiano
const newITEntries = [
  {
    "q": "Ho un'azienda con 30 dipendenti, cosa devo fare per la NIS2?",
    "a": "Se la tua azienda ha 30 dipendenti e meno di 10 milioni di euro di fatturato, è classificata come micro o piccola impresa. Come regola generale (Art. 2), la NIS2 non si applica, a meno che non operiate in settori iper-critici (es. servizi fiduciari, TLD) o siate l'unico fornitore di un servizio essenziale. Tuttavia, ti consiglio di verificare se siete parte della supply chain di un'entità più grande, poiché potrebbero richiedervi garanzie di sicurezza tramite contratti.",
    "c": "applicability",
    "d": "",
    "f": ["Cos'è un soggetto essenziale?", "Quali sono i settori critici?", "Cosa dice la NIS2 sulla supply chain?"],
    "r": [],
    "s": "info"
  },
  {
    "q": "Siamo un'azienda manifatturiera, la NIS2 ci riguarda?",
    "a": "Sì, la manifattura (fabbricazione di dispositivi medici, computer, elettronica, macchinari, autoveicoli, ecc.) rientra nell'Allegato II (Soggetti Importanti). Se avete più di 50 dipendenti o superate i 10 milioni di euro di fatturato annuo, ricadete nel campo di applicazione della direttiva e dovrete adeguarvi implementando le misure di gestione del rischio (Art. 21) e le procedure di notifica (Art. 23).",
    "c": "applicability",
    "d": "",
    "f": ["Quali sono le misure di sicurezza obbligatorie?", "Cos'è un soggetto importante nella NIS2?", "Entro quando dobbiamo adeguarci?"],
    "r": [],
    "s": "info"
  },
  {
    "q": "Cosa succede se non rispetto la direttiva NIS2? Rischio multe?",
    "a": "Sì, le sanzioni per non conformità sono molto severe (Art. 34). Per i Soggetti Essenziali, le multe possono arrivare a 10 milioni di euro o al 2% del fatturato mondiale annuo (si applica l'importo maggiore). Per i Soggetti Importanti, il tetto è 7 milioni di euro o 1,4% del fatturato. Inoltre, i vertici aziendali possono essere ritenuti personalmente responsabili e, nei casi più gravi, sospesi temporaneamente.",
    "c": "sanctions",
    "d": "",
    "f": ["Chi controlla il rispetto della NIS2?", "Quali sono le responsabilità del CDA?", "Cos'è un soggetto essenziale?"],
    "r": [],
    "s": "warning"
  },
  {
    "q": "Aiuto, abbiamo subito un attacco ransomware! Cosa prevede la NIS2?",
    "a": "La NIS2 ha scadenze rigorosissime per gli incidenti (Art. 23). Se l'attacco è un 'incidente significativo', devi inviare un preallarme al tuo CSIRT nazionale (es. ACN in Italia) entro 24 ore da quando ne vieni a conoscenza. Entro 72 ore devi fornire una notifica completa con un'analisi iniziale, ed entro 1 mese devi inviare una relazione finale. Ti consiglio di attivare subito il tuo piano di Incident Response e isolare i sistemi infetti.",
    "c": "incident_response",
    "d": "24h",
    "f": ["Cos'è un incidente significativo?", "Come strutturo un piano di incident response?", "Dobbiamo avvisare anche i clienti?"],
    "r": ["ISO 27035"],
    "s": "danger"
  },
  {
    "q": "Il CDA è responsabile della cybersecurity? Rischiamo qualcosa?",
    "a": "Assolutamente sì. L'Articolo 20 trasforma la cybersecurity in un obbligo per i vertici. Il Consiglio di Amministrazione (o l'organo direttivo) deve approvare le misure di sicurezza, supervisionarne l'implementazione e seguire una formazione obbligatoria. In caso di inadempienza, le autorità possono ritenere i membri del CDA personalmente responsabili dei danni.",
    "c": "governance",
    "d": "",
    "f": ["Che formazione deve fare il CDA?", "Quali sono le sanzioni?", "Quali sono le misure di sicurezza richieste?"],
    "r": [],
    "s": "warning"
  },
  {
    "q": "Quanto costa adeguarsi alla NIS2?",
    "a": "Non esiste un costo fisso. ENISA stima che i costi di adeguamento si aggirino intorno al 10-22% in più del budget attuale per l'ICT, a seconda del livello di maturità di partenza. Se hai già la ISO 27001, il gap da colmare sarà ridotto. L'investimento andrà distribuito in consulenza legale, tecnologie di sicurezza (MFA, crittografia, monitoraggio) e, soprattutto, processi e formazione.",
    "c": "general",
    "d": "",
    "f": ["Da dove iniziare con NIS2?", "Quali tecnologie devo comprare?", "Cos'è la ISO 27001?"],
    "r": [],
    "s": "info"
  },
  {
    "q": "I nostri fornitori devono rispettare la NIS2?",
    "a": "Sì, la sicurezza della supply chain è un pilastro della NIS2 (Art. 21). Anche se il tuo fornitore è troppo piccolo per rientrare direttamente nella direttiva, sarai TU a dovergli richiedere adeguate garanzie di sicurezza tramite contratto. Dovrai valutare il rischio che ciascun fornitore rappresenta per la tua azienda (es. accesso ai dati, connessioni di rete).",
    "c": "supply_chain",
    "d": "",
    "f": ["Come valuto un fornitore?", "Se il fornitore viene hackerato di chi è la colpa?", "Quali clausole mettere nel contratto?"],
    "r": ["ISO 27001", "ISO 27036"],
    "s": "warning"
  },
  {
    "q": "Devo criptare tutti i miei dati?",
    "a": "La NIS2 (Art. 21) impone l'adozione di policy su crittografia e cifratura 'ove opportuno'. Non sei obbligato a criptare tutto indiscriminatamente, ma devi farlo in base all'analisi dei rischi. I dati in transito (comunicazioni) e i dati a riposo critici o sensibili devono essere protetti. Utilizzare standard forti come AES-256 e TLS 1.2+ è altamente raccomandato.",
    "c": "encryption",
    "d": "",
    "f": ["Quale cifratura usare?", "Come fare l'analisi dei rischi?", "Cosa dice la direttiva sulla MFA?"],
    "r": ["FIPS 140-3", "NIST SP 800-111"],
    "s": "info"
  }
];

// Nuovi record conversazionali in Inglese
const newENEntries = [
  {
    "q": "I run a company with 30 employees, do I have to care about NIS2?",
    "a": "If your company has 30 employees and less than 10 million EUR in turnover, it is generally classified as a small enterprise. Under Article 2, NIS2 size thresholds exempt most small and micro businesses unless you operate in a hyper-critical sub-sector (e.g. trust services, TLD registries) or act as a sole provider. However, keep an eye on your supply chain: larger clients who are subject to NIS2 will likely impose strict cybersecurity clauses on you.",
    "c": "applicability",
    "d": "",
    "f": ["What are the critical sectors?", "What is an essential entity?", "What does NIS2 say about supply chain?"],
    "r": [],
    "s": "info"
  },
  {
    "q": "We are a manufacturing plant. Are we in scope for NIS2?",
    "a": "Yes, manufacturing (including medical devices, computers, electronics, machinery, and motor vehicles) is listed in Annex II (Other Critical Sectors). If you exceed the threshold of 50 employees or 10 million EUR in turnover, you fall under NIS2 as an 'Important Entity'. You will need to implement Article 21 risk-management measures and adhere to the 24-hour incident reporting window under Article 23.",
    "c": "applicability",
    "d": "",
    "f": ["What are the mandatory security measures?", "What is the difference between essential and important?", "When is the deadline to comply?"],
    "r": [],
    "s": "info"
  },
  {
    "q": "What's the worst that can happen if we ignore NIS2?",
    "a": "The financial and legal consequences are severe (Art. 34). Essential Entities can be fined up to 10 million EUR or 2% of their total worldwide annual turnover, whichever is higher. For Important Entities, it's 7 million EUR or 1.4%. Beyond fines, national authorities can temporarily suspend your operations or hold the management body (C-level/Board) personally liable for negligence.",
    "c": "sanctions",
    "d": "",
    "f": ["Who enforces NIS2?", "What are the board's responsibilities?", "What is an essential entity?"],
    "r": [],
    "s": "warning"
  },
  {
    "q": "Help, we just discovered a data breach! What should we do under NIS2?",
    "a": "Time is running out. Article 23 enforces a strict multi-stage reporting process for 'significant incidents'. You must issue an Early Warning to your national CSIRT within 24 hours. A full Incident Notification detailing the impact is due within 72 hours. Finally, a comprehensive Final Report is required within 1 month. Immediately activate your Incident Response plan and contain the breach.",
    "c": "incident_response",
    "d": "24h",
    "f": ["What counts as a significant incident?", "How do I build an incident response plan?", "Do we need to tell our customers?"],
    "r": ["ISO 27035", "NIST SP 800-61"],
    "s": "danger"
  },
  {
    "q": "Can the CEO go to jail or be held liable?",
    "a": "While jail time depends on local member state transposition, personal liability is explicitly written into the directive. Article 20 mandates that management bodies must approve and oversee the cybersecurity risk measures. If an organization fails to comply, authorities can hold the management personally accountable and, in extreme cases, temporarily prohibit individuals from holding managerial positions.",
    "c": "governance",
    "d": "",
    "f": ["What training is required for management?", "What are the NIS2 sanctions?", "How do we start a risk assessment?"],
    "r": [],
    "s": "warning"
  }
];

// Add the new entries
const allNew = [...newITEntries, ...newENEntries];
allNew.forEach(entry => dataset.push(entry));

fs.writeFileSync(datasetPath, JSON.stringify(dataset, null, 2));
console.log(`Added ${allNew.length} new conversational entries to dataset.json. Total entries: ${dataset.length}`);
