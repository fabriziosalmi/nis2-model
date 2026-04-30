// NIS2 Glossary Expansion — batch 3 (100+ additional terms, longer definitions)

const G3 = {
// === NIS2 ARTICLES (expanded) ===
'art. 21(2)(a)':{it:'Politiche di analisi dei rischi e sicurezza dei sistemi informativi — obbligo fondamentale da cui derivano tutte le altre misure dell\'Art. 21.',en:'Risk analysis and information system security policies — foundational obligation from which all other Art. 21 measures derive.'},
'art. 21(2)(b)':{it:'Gestione degli incidenti — procedure di rilevamento, analisi, contenimento, eliminazione, ripristino e revisione post-incidente.',en:'Incident handling — detection, analysis, containment, eradication, recovery, and post-incident review procedures.'},
'art. 21(2)(c)':{it:'Continuità operativa — gestione backup, ripristino in caso di disastro, gestione delle crisi. I piani devono essere documentati e testati.',en:'Business continuity — backup management, disaster recovery, crisis management. Plans must be documented and tested.'},
'art. 21(2)(d)':{it:'Sicurezza della catena di approvvigionamento — clausole di sicurezza nei contratti, valutazione fornitori, monitoraggio rischi, coordinamento incidenti.',en:'Supply chain security — security clauses in contracts, vendor assessments, risk monitoring, incident coordination.'},
'art. 21(2)(e)':{it:'Sicurezza nell\'acquisizione, sviluppo e manutenzione dei sistemi — gestione vulnerabilità, SDLC sicuro, patch management.',en:'Security in system acquisition, development and maintenance — vulnerability management, secure SDLC, patch management.'},
'art. 21(2)(f)':{it:'Politiche per valutare l\'efficacia delle misure di sicurezza — audit interni, penetration test, revisioni periodiche, KPI di sicurezza.',en:'Policies to assess effectiveness of security measures — internal audits, penetration tests, periodic reviews, security KPIs.'},
'art. 21(2)(g)':{it:'Pratiche di igiene informatica di base e formazione — configurazione dispositivi, aggiornamenti, gestione password, formazione periodica dipendenti.',en:'Basic cyber hygiene practices and training — device configuration, updates, password management, periodic employee training.'},
'art. 21(2)(h)':{it:'Politiche di crittografia e cifratura — AES-256 per dati a riposo, TLS 1.2+ per dati in transito, gestione chiavi, PKI, firma digitale.',en:'Cryptography and encryption policies — AES-256 for data at rest, TLS 1.2+ for data in transit, key management, PKI, digital signatures.'},
'art. 21(2)(i)':{it:'Sicurezza delle risorse umane, controllo accessi e gestione degli asset — IAM, RBAC, PAM, inventario asset, classificazione dati.',en:'HR security, access control and asset management — IAM, RBAC, PAM, asset inventory, data classification.'},
'art. 21(2)(j)':{it:'Autenticazione multi-fattore o continua, comunicazioni protette — MFA, FIDO2, VPN, cifratura voce/video, comunicazioni di emergenza.',en:'Multi-factor or continuous authentication, secured communications — MFA, FIDO2, VPN, voice/video encryption, emergency communications.'},

// === INCIDENT TIMELINE ===
'preallarme':{it:'Prima notifica al CSIRT entro 24 ore dalla conoscenza di un incidente significativo (Art. 23(4)(a)). Deve indicare se malevolo e se transfrontaliero.',en:'First notification to CSIRT within 24 hours of becoming aware of a significant incident (Art. 23(4)(a)). Must indicate if malicious and cross-border.'},
'notifica completa':{it:'Seconda comunicazione al CSIRT entro 72 ore (Art. 23(4)(b)). Include valutazione severità, impatto, indicatori di compromissione.',en:'Second communication to CSIRT within 72 hours (Art. 23(4)(b)). Includes severity assessment, impact, indicators of compromise.'},
'relazione finale':{it:'Report conclusivo al CSIRT entro 30 giorni (Art. 23(4)(d)). Root cause analysis, misure adottate, impatto transfrontaliero, lezioni apprese.',en:'Final report to CSIRT within 30 days (Art. 23(4)(d)). Root cause analysis, measures taken, cross-border impact, lessons learned.'},
'incidente significativo':{it:'Incidente con impatto operativo rilevante sul servizio, perdita finanziaria sostanziale, o danno a persone fisiche (Art. 23(3)).',en:'Incident with significant operational impact on service, substantial financial loss, or harm to natural persons (Art. 23(3)).'},

// === ITALIAN REGULATORY ===
'acn registrazione':{it:'Le entità soggette a NIS2 devono registrarsi presso l\'ACN. La registrazione include informazioni su settore, servizi, contatti di sicurezza.',en:'Entities subject to NIS2 must register with ACN. Registration includes sector, services, and security contact information.'},
'csirt italia':{it:'Il CSIRT italiano presso l\'ACN riceve le notifiche di incidente. Fornisce supporto tecnico, condivide IoC, coordina la risposta a livello nazionale.',en:'The Italian CSIRT at ACN receives incident notifications. Provides technical support, shares IoCs, coordinates national-level response.'},
'perimetro di sicurezza nazionale cibernetica':{it:'Framework italiano pre-NIS2 per la protezione delle infrastrutture critiche. La NIS2 lo integra e amplia.',en:'Italian pre-NIS2 framework for critical infrastructure protection. NIS2 integrates and expands it.'},
'garante privacy':{it:'Autorità italiana per la protezione dei dati personali. Art. 35 NIS2 richiede cooperazione quando incidenti coinvolgono dati personali.',en:'Italian data protection authority. NIS2 Art. 35 requires cooperation when incidents involve personal data.'},

// === ADVANCED SECURITY CONCEPTS ===
'security by design':{it:'Principio di integrazione della sicurezza fin dalla fase di progettazione, non aggiunta a posteriori. Richiesto implicitamente da Art. 21(2)(e).',en:'Principle of integrating security from the design phase, not added afterwards. Implicitly required by Art. 21(2)(e).'},
'security orchestration':{it:'Automazione e coordinamento dei processi di sicurezza tramite piattaforme SOAR per ridurre tempi di risposta e errori umani.',en:'Automation and coordination of security processes via SOAR platforms to reduce response times and human errors.'},
'threat intelligence':{it:'Raccolta e analisi di informazioni su minacce informatiche per prevenzione proattiva. Feed IoC, rapporti settoriali, sharing ISAC.',en:'Collection and analysis of cyber threat information for proactive prevention. IoC feeds, sector reports, ISAC sharing.'},
'security posture':{it:'Stato complessivo della sicurezza di un\'organizzazione: controlli implementati, vulnerabilità note, capacità di rilevamento e risposta.',en:'Overall security state of an organization: implemented controls, known vulnerabilities, detection and response capabilities.'},
'cyber resilience':{it:'Capacità di un\'organizzazione di prepararsi, resistere, rispondere e riprendersi da incidenti informatici mantenendo le operazioni.',en:'Organization\'s ability to prepare for, withstand, respond to, and recover from cyber incidents while maintaining operations.'},
'kill chain':{it:'Modello a fasi di un attacco informatico: ricognizione, armamento, consegna, sfruttamento, installazione, comando e controllo, azioni.',en:'Phased model of a cyber attack: reconnaissance, weaponization, delivery, exploitation, installation, C2, actions on objectives.'},
'assume breach':{it:'Approccio alla sicurezza che presume la compromissione sia già avvenuta. Focalizza su rilevamento, contenimento e resilienza.',en:'Security approach assuming compromise has already occurred. Focuses on detection, containment, and resilience.'},

// === COMPLIANCE METHODOLOGIES ===
'iso 27001 annex a':{it:'Lista di 93 controlli di sicurezza organizzati in 4 temi: organizzativi, persone, fisici, tecnologici. Riferimento per mappatura NIS2.',en:'List of 93 security controls organized in 4 themes: organizational, people, physical, technological. Reference for NIS2 mapping.'},
'nist sp 800-30':{it:'Guida NIST per la conduzione di valutazioni del rischio — metodologia strutturata per analisi minacce, vulnerabilità, impatti.',en:'NIST guide for conducting risk assessments — structured methodology for threat, vulnerability, and impact analysis.'},
'iso 27002':{it:'Linee guida per l\'implementazione dei controlli ISO 27001. Fornisce dettagli pratici per ogni controllo dell\'Annex A.',en:'Guidelines for implementing ISO 27001 controls. Provides practical details for each Annex A control.'},
'cobit 2019':{it:'Framework ISACA per la governance IT aziendale. Complementare a NIS2 per il livello di governance e supervisione del CDA.',en:'ISACA framework for enterprise IT governance. Complementary to NIS2 for board-level governance and oversight.'},
'enisa guidelines':{it:'Linee guida tecniche dell\'ENISA per l\'implementazione delle misure NIS2. Coprono tutti gli ambiti dell\'Art. 21.',en:'ENISA technical guidelines for NIS2 measure implementation. Cover all areas of Art. 21.'},

// === IDENTITY & AUTH (expanded) ===
'conditional access':{it:'Politiche di accesso condizionale che valutano rischio del dispositivo, posizione, comportamento utente prima di concedere accesso.',en:'Conditional access policies evaluating device risk, location, user behavior before granting access.'},
'just-in-time access':{it:'Accesso privilegiato concesso solo quando necessario e per la durata minima richiesta. Riduce la superficie di attacco persistente.',en:'Privileged access granted only when needed and for minimum required duration. Reduces persistent attack surface.'},
'session management':{it:'Gestione delle sessioni utente: timeout, invalidazione, single-session enforcement, protezione da session hijacking.',en:'User session management: timeout, invalidation, single-session enforcement, session hijacking protection.'},
'certificate pinning':{it:'Tecnica che associa un certificato specifico a un servizio per prevenire attacchi man-in-the-middle con certificati fraudolenti.',en:'Technique binding a specific certificate to a service to prevent MITM attacks with fraudulent certificates.'},

// === NETWORK ADVANCED ===
'east-west traffic':{it:'Traffico laterale tra sistemi nella stessa rete. Spesso non monitorato, è il canale principale del movimento laterale degli attaccanti.',en:'Lateral traffic between systems in the same network. Often unmonitored, it\'s the main channel for attacker lateral movement.'},
'north-south traffic':{it:'Traffico tra la rete interna e internet. Tipicamente filtrato da firewall perimetrali e proxy.',en:'Traffic between internal network and internet. Typically filtered by perimeter firewalls and proxies.'},
'network taps':{it:'Dispositivi hardware per cattura passiva del traffico di rete senza impatto sulle performance. Essenziali per NDR e forensics.',en:'Hardware devices for passive network traffic capture without performance impact. Essential for NDR and forensics.'},
'sdn':{it:'Software-Defined Networking — separazione del piano di controllo dal piano dati per gestione rete programmabile e centralizzata.',en:'Software-Defined Networking — separation of control plane from data plane for programmable centralized network management.'},

// === BACKUP ADVANCED ===
'ransomware-proof backup':{it:'Strategia di backup resistente a ransomware: backup immutabili, air-gapped, con MFA per accesso, test di ripristino regolari.',en:'Ransomware-resistant backup strategy: immutable backups, air-gapped, MFA for access, regular restore testing.'},
'backup verification':{it:'Processo di verifica automatica dell\'integrità e recuperabilità dei backup tramite restore test e checksum.',en:'Automatic verification process of backup integrity and recoverability through restore tests and checksums.'},
'rppo':{it:'Recovery Point Performance Objective — velocità minima di ripristino dati richiesta per rispettare l\'RPO.',en:'Recovery Point Performance Objective — minimum data restore speed required to meet RPO.'},

// === CLOUD SECURITY (expanded) ===
'shared responsibility model':{it:'Modello di responsabilità condivisa nel cloud: il provider protegge l\'infrastruttura, il cliente protegge dati, configurazioni e accessi.',en:'Shared responsibility model in cloud: provider protects infrastructure, customer protects data, configurations, and access.'},
'cloud workload protection':{it:'Protezione dei carichi di lavoro cloud: monitoraggio container, serverless, VM. Rilevamento anomalie e vulnerabilità in runtime.',en:'Cloud workload protection: container, serverless, VM monitoring. Runtime anomaly and vulnerability detection.'},
'data residency':{it:'Localizzazione geografica dei dati. Rilevante per NIS2 quando i dati risiedono in paesi extra-UE — verificare adeguatezza e clausole contrattuali.',en:'Geographic location of data. Relevant for NIS2 when data resides in non-EU countries — verify adequacy and contractual clauses.'},

// === GOVERNANCE (expanded) ===
'three lines model':{it:'Modello delle tre linee di difesa: (1) management operativo, (2) risk/compliance, (3) audit interno. Framework per governance sicurezza.',en:'Three lines of defense model: (1) operational management, (2) risk/compliance, (3) internal audit. Framework for security governance.'},
'security steering committee':{it:'Comitato direttivo di sicurezza che riporta al CDA. Composizione: CISO, CTO, legale, compliance, HR. Decisioni strategiche di sicurezza.',en:'Security steering committee reporting to the board. Composition: CISO, CTO, legal, compliance, HR. Strategic security decisions.'},
'security charter':{it:'Documento che definisce mandato, autorità, risorse e responsabilità della funzione di sicurezza all\'interno dell\'organizzazione.',en:'Document defining the mandate, authority, resources, and responsibilities of the security function within the organization.'},

// === INCIDENT RESPONSE (expanded) ===
'chain of custody':{it:'Catena di custodia — documentazione continua della gestione delle prove digitali per garantirne l\'ammissibilità legale.',en:'Chain of custody — continuous documentation of digital evidence handling to ensure legal admissibility.'},
'containment strategy':{it:'Strategia di contenimento incidente: isolamento rete, blocco account compromessi, quarantena endpoint, preservazione prove.',en:'Incident containment strategy: network isolation, blocking compromised accounts, endpoint quarantine, evidence preservation.'},
'lessons learned':{it:'Revisione post-incidente per identificare miglioramenti: cosa ha funzionato, cosa no, gap nei processi, aggiornamenti necessari.',en:'Post-incident review to identify improvements: what worked, what didn\'t, process gaps, needed updates.'},
'war room':{it:'Sala operativa di crisi attivata durante incidenti maggiori per coordinamento centralizzato delle operazioni di risposta.',en:'Crisis operations room activated during major incidents for centralized coordination of response operations.'},

// === VULNERABILITY (expanded) ===
'cisa kev catalog':{it:'Catalogo CISA delle vulnerabilità attivamente sfruttate. Patching obbligatorio entro le scadenze indicate per sistemi governativi USA, best practice per tutti.',en:'CISA catalog of actively exploited vulnerabilities. Mandatory patching within deadlines for US government systems, best practice for all.'},
'responsible disclosure':{it:'Divulgazione responsabile: il ricercatore segnala la vulnerabilità al vendor e attende un periodo ragionevole prima della pubblicazione.',en:'Responsible disclosure: researcher reports vulnerability to vendor and waits a reasonable period before publication.'},
'virtual patching':{it:'Mitigazione temporanea di vulnerabilità tramite regole WAF/IPS quando la patch ufficiale non è ancora disponibile o applicabile.',en:'Temporary vulnerability mitigation via WAF/IPS rules when the official patch is not yet available or applicable.'},
'attack path analysis':{it:'Analisi dei percorsi di attacco: mappatura delle catene di vulnerabilità che un attaccante potrebbe sfruttare per raggiungere asset critici.',en:'Attack path analysis: mapping vulnerability chains an attacker could exploit to reach critical assets.'},

// === DATA GOVERNANCE ===
'data lifecycle':{it:'Ciclo di vita del dato: creazione, classificazione, archiviazione, utilizzo, condivisione, archiviazione, distruzione sicura.',en:'Data lifecycle: creation, classification, storage, use, sharing, archiving, secure destruction.'},
'data retention':{it:'Politiche di conservazione dati: definire per quanto tempo conservare ogni tipo di dato. Log di sicurezza: minimo 12 mesi online.',en:'Data retention policies: define how long to keep each data type. Security logs: minimum 12 months online.'},
'secure deletion':{it:'Cancellazione sicura: sovrascrittura multipla (NIST SP 800-88), distruzione fisica per supporti critici, crittografia con cancellazione chiave.',en:'Secure deletion: multiple overwriting (NIST SP 800-88), physical destruction for critical media, encryption with key destruction.'},

// === PHYSICAL (expanded) ===
'mantrap':{it:'Bussola di sicurezza — doppia porta interbloccata che impedisce il tailgating negli accessi controllati ad aree critiche.',en:'Security mantrap — double interlocked door preventing tailgating in controlled access to critical areas.'},
'environmental monitoring':{it:'Monitoraggio ambientale: temperatura, umidità, allagamento, fumo, alimentazione in data center e sale server.',en:'Environmental monitoring: temperature, humidity, flooding, smoke, power supply in data centers and server rooms.'},
'ups':{it:'Uninterruptible Power Supply — gruppo di continuità che protegge i sistemi da interruzioni di corrente e sbalzi di tensione.',en:'Uninterruptible Power Supply — protects systems from power outages and voltage fluctuations.'},

// === OPERATIONAL ===
'change advisory board':{it:'Comitato consultivo per i cambiamenti (CAB) — approva e pianifica le modifiche ai sistemi in produzione con review di sicurezza.',en:'Change Advisory Board (CAB) — approves and plans changes to production systems with security review.'},
'configuration management':{it:'Gestione delle configurazioni — mantenere baseline di sicurezza, rilevare drift, enforcement automatico via IaC.',en:'Configuration management — maintain security baselines, detect drift, automated enforcement via IaC.'},
'capacity planning':{it:'Pianificazione capacità — dimensionamento risorse per garantire disponibilità dei servizi anche sotto carico o attacco DDoS.',en:'Capacity planning — resource sizing to ensure service availability even under load or DDoS attack.'},

// === TESTING ===
'chaos engineering':{it:'Ingegneria del caos — introduzione controllata di guasti per testare la resilienza dei sistemi in produzione.',en:'Chaos engineering — controlled introduction of failures to test system resilience in production.'},
'fuzz testing':{it:'Test di fuzzing — invio di dati casuali/malformati a un sistema per scoprire vulnerabilità di parsing e gestione errori.',en:'Fuzz testing — sending random/malformed data to a system to discover parsing and error handling vulnerabilities.'},
'threat hunting':{it:'Caccia alle minacce — ricerca proattiva di indicatori di compromissione nella rete senza attendere alert automatici.',en:'Threat hunting — proactive search for indicators of compromise in the network without waiting for automated alerts.'},

// === PRIVACY (expanded) ===
'privacy by design':{it:'Protezione dati fin dalla progettazione — principio GDPR complementare al security by design di NIS2.',en:'Data protection by design — GDPR principle complementary to NIS2\'s security by design.'},
'data minimization':{it:'Principio di minimizzazione: raccogliere e conservare solo i dati strettamente necessari allo scopo dichiarato.',en:'Minimization principle: collect and retain only data strictly necessary for the stated purpose.'},
'right to erasure':{it:'Diritto alla cancellazione (GDPR Art. 17) — richiede capacità tecniche di cancellazione sicura e completa.',en:'Right to erasure (GDPR Art. 17) — requires technical capabilities for secure and complete deletion.'},

// === EU REGULATORY LANDSCAPE ===
'cyber resilience act':{it:'Regolamento UE sulla ciberresilienza per prodotti con elementi digitali. Complementare a NIS2, copre sicurezza dei prodotti IoT/software.',en:'EU regulation on cyber resilience for products with digital elements. Complementary to NIS2, covers IoT/software product security.'},
'eu cybersecurity act':{it:'Regolamento (UE) 2019/881 — istituisce ENISA e il framework di certificazione di cybersecurity dell\'UE.',en:'Regulation (EU) 2019/881 — establishes ENISA and the EU cybersecurity certification framework.'},
'cef':{it:'Connecting Europe Facility — programma di finanziamento UE per infrastrutture digitali e progetti di cybersecurity.',en:'Connecting Europe Facility — EU funding program for digital infrastructure and cybersecurity projects.'},
'eu-cyclone':{it:'European Cyber Crises Liaison Organisation Network — rete per la gestione coordinata delle crisi cyber a livello UE.',en:'European Cyber Crises Liaison Organisation Network — network for coordinated EU-level cyber crisis management.'},
'isac':{it:'Information Sharing and Analysis Center — centro settoriale per la condivisione di intelligence sulle minacce cyber.',en:'Information Sharing and Analysis Center — sector-specific center for sharing cyber threat intelligence.'},
}

export default G3
