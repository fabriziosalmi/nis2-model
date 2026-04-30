// NIS2 Glossary Expansion — batch 2 (150 additional terms)

const G2 = {
// === SECTORS (Annex I) ===
'energia':{it:'Settore Allegato I: elettricità, petrolio, gas, idrogeno, teleriscaldamento.',en:'Annex I sector: electricity, oil, gas, hydrogen, district heating.'},
'trasporti':{it:'Settore Allegato I: aereo, ferroviario, marittimo, stradale.',en:'Annex I sector: air, rail, maritime, road.'},
'settore bancario':{it:'Settore Allegato I: istituti di credito soggetti a NIS2 e DORA.',en:'Annex I sector: credit institutions subject to NIS2 and DORA.'},
'sanità':{it:'Settore Allegato I: ospedali, laboratori, produttori dispositivi medici.',en:'Annex I sector: hospitals, laboratories, medical device manufacturers.'},
'acqua potabile':{it:'Settore Allegato I: fornitura e distribuzione di acqua potabile.',en:'Annex I sector: drinking water supply and distribution.'},
'acque reflue':{it:'Settore Allegato I: raccolta e trattamento acque reflue.',en:'Annex I sector: wastewater collection and treatment.'},
'infrastruttura digitale':{it:'Settore Allegato I: DNS, TLD, cloud, data center, CDN, TSP.',en:'Annex I sector: DNS, TLD, cloud, data center, CDN, TSP.'},
'pubblica amministrazione':{it:'Settore Allegato I: entità essenziali indipendentemente dalla dimensione.',en:'Annex I sector: essential entities regardless of size.'},
'spazio':{it:'Settore Allegato I: operatori infrastrutture terrestri per servizi spaziali.',en:'Annex I sector: ground infrastructure operators for space services.'},

// === SECTORS (Annex II) ===
'servizi postali':{it:'Settore Allegato II: operatori postali e corrieri.',en:'Annex II sector: postal operators and couriers.'},
'gestione rifiuti':{it:'Settore Allegato II: raccolta, trattamento e smaltimento rifiuti.',en:'Annex II sector: waste collection, treatment, and disposal.'},
'chimica':{it:'Settore Allegato II: produzione e distribuzione sostanze chimiche.',en:'Annex II sector: production and distribution of chemicals.'},
'alimentare':{it:'Settore Allegato II: produzione, trasformazione e distribuzione alimenti.',en:'Annex II sector: food production, processing, and distribution.'},
'manifattura':{it:'Settore Allegato II: produzione dispositivi medici, elettronica, macchinari, veicoli.',en:'Annex II sector: medical devices, electronics, machinery, vehicles manufacturing.'},
'ricerca':{it:'Settore Allegato II: organizzazioni di ricerca.',en:'Annex II sector: research organizations.'},

// === SECURITY ARCHITECTURE ===
'defense in depth':{it:'Difesa in profondità — più livelli di sicurezza sovrapposti.',en:'Defense in depth — multiple overlapping security layers.'},
'microsegmentazione':{it:'Segmentazione granulare della rete fino al livello di singolo workload.',en:'Granular network segmentation down to individual workload level.'},
'bastion host':{it:'Server hardened usato come punto di accesso controllato alla rete interna.',en:'Hardened server used as controlled access point to internal network.'},
'jump server':{it:'Server intermedio per accesso amministrativo a sistemi in zone sicure.',en:'Intermediate server for administrative access to systems in secure zones.'},
'honeypot':{it:'Sistema esca progettato per attirare e studiare gli attaccanti.',en:'Decoy system designed to attract and study attackers.'},
'sandbox':{it:'Ambiente isolato per esecuzione sicura di codice o file sospetti.',en:'Isolated environment for safe execution of suspicious code or files.'},
'container':{it:'Contenitore — unità di virtualizzazione leggera per applicazioni (Docker, K8s).',en:'Container — lightweight virtualization unit for applications (Docker, K8s).'},
'kubernetes':{it:'Piattaforma di orchestrazione container per deploy e scaling automatizzato.',en:'Container orchestration platform for automated deployment and scaling.'},
'hardening':{it:'Rafforzamento della sicurezza — riduzione della superficie di attacco di un sistema.',en:'Security hardening — reducing the attack surface of a system.'},
'attack surface':{it:'Superficie di attacco — insieme di tutti i punti di ingresso potenziali.',en:'Attack surface — set of all potential entry points.'},

// === AUTHENTICATION ===
'biometria':{it:'Autenticazione basata su caratteristiche fisiche (impronta, volto, iride).',en:'Authentication based on physical characteristics (fingerprint, face, iris).'},
'passwordless':{it:'Autenticazione senza password — basata su FIDO2, biometria o certificati.',en:'Passwordless authentication — based on FIDO2, biometrics, or certificates.'},
'passkey':{it:'Credenziale FIDO2 sincronizzabile per autenticazione passwordless.',en:'Syncable FIDO2 credential for passwordless authentication.'},
'certificate-based':{it:'Autenticazione basata su certificati digitali X.509.',en:'Authentication based on X.509 digital certificates.'},

// === BACKUP & STORAGE ===
'backup incrementale':{it:'Backup che salva solo i dati modificati dall\'ultimo backup.',en:'Backup that saves only data changed since the last backup.'},
'backup differenziale':{it:'Backup che salva i dati modificati dall\'ultimo backup completo.',en:'Backup that saves data changed since the last full backup.'},
'snapshot':{it:'Istantanea dello stato di un sistema in un punto nel tempo.',en:'Point-in-time capture of a system state.'},
'immutable backup':{it:'Backup immutabile — non modificabile né cancellabile per il periodo di ritenzione.',en:'Immutable backup — cannot be modified or deleted during retention period.'},
'regola 3-2-1':{it:'3 copie dei dati, su 2 supporti diversi, 1 offsite. Best practice per backup.',en:'3 copies of data, on 2 different media, 1 offsite. Backup best practice.'},
'cold storage':{it:'Archiviazione a freddo — storage offline a basso costo per dati ad accesso raro.',en:'Cold storage — offline low-cost storage for rarely accessed data.'},
'tape backup':{it:'Backup su nastro — archiviazione offline per retention a lungo termine e air gap.',en:'Tape backup — offline storage for long-term retention and air gap.'},

// === COMPLIANCE PROCESSES ===
'audit trail':{it:'Traccia di audit — registrazione cronologica di tutte le azioni su un sistema.',en:'Audit trail — chronological record of all actions on a system.'},
'evidence collection':{it:'Raccolta prove — documentazione delle misure di sicurezza per audit e ispezioni.',en:'Evidence collection — documentation of security measures for audits and inspections.'},
'compliance mapping':{it:'Mappatura conformità — correlazione tra requisiti NIS2 e controlli implementati.',en:'Compliance mapping — correlation between NIS2 requirements and implemented controls.'},
'remediation plan':{it:'Piano di rimedio — azioni correttive per sanare le non conformità identificate.',en:'Remediation plan — corrective actions to address identified non-compliance.'},
'internal audit':{it:'Audit interno — verifica interna dell\'efficacia delle misure (Art. 21(2)(f)).',en:'Internal audit — internal verification of measure effectiveness (Art. 21(2)(f)).'},
'penetration testing':{it:'Test di penetrazione — simulazione di attacco per verificare la sicurezza.',en:'Penetration testing — attack simulation to verify security.'},
'red team':{it:'Red team — team che simula attacchi reali per testare difese e processi.',en:'Red team — team simulating real attacks to test defenses and processes.'},
'blue team':{it:'Blue team — team difensivo che protegge e risponde agli attacchi.',en:'Blue team — defensive team that protects and responds to attacks.'},
'purple team':{it:'Purple team — collaborazione tra red e blue team per miglioramento continuo.',en:'Purple team — collaboration between red and blue teams for continuous improvement.'},

// === THREATS (expanded) ===
'man in the middle':{it:'Attacco in cui l\'attaccante intercetta comunicazioni tra due parti.',en:'Attack where the attacker intercepts communications between two parties.'},
'dns spoofing':{it:'Manipolazione delle risposte DNS per reindirizzare il traffico.',en:'Manipulation of DNS responses to redirect traffic.'},
'sql injection':{it:'Iniezione SQL — attacco che inserisce codice SQL malevolo nelle query.',en:'SQL injection — attack inserting malicious SQL code into queries.'},
'xss':{it:'Cross-Site Scripting — iniezione di script malevoli in pagine web.',en:'Cross-Site Scripting — injection of malicious scripts into web pages.'},
'brute force':{it:'Attacco a forza bruta — tentativi sistematici di tutte le combinazioni di password.',en:'Brute force attack — systematic attempts of all password combinations.'},
'insider threat':{it:'Minaccia interna — rischio da dipendenti o collaboratori con accesso privilegiato.',en:'Insider threat — risk from employees or collaborators with privileged access.'},
'watering hole':{it:'Attacco che compromette siti web frequentati dal target per infettarlo.',en:'Attack compromising websites frequented by the target to infect them.'},
'zero day':{it:'Vulnerabilità sconosciuta al vendor, senza patch disponibile.',en:'Vulnerability unknown to the vendor, with no available patch.'},
'botnet':{it:'Rete di dispositivi compromessi controllati da un attaccante per attacchi coordinati.',en:'Network of compromised devices controlled by an attacker for coordinated attacks.'},
'malware':{it:'Software malevolo progettato per danneggiare o compromettere sistemi.',en:'Malicious software designed to damage or compromise systems.'},
'trojan':{it:'Cavallo di Troia — malware mascherato da software legittimo.',en:'Trojan horse — malware disguised as legitimate software.'},
'rootkit':{it:'Kit di strumenti che si nascondono nel sistema operativo per mantenere accesso persistente.',en:'Toolkit hiding in the OS to maintain persistent access.'},
'keylogger':{it:'Software che registra le digitazioni per catturare password e dati sensibili.',en:'Software recording keystrokes to capture passwords and sensitive data.'},
'wiper':{it:'Malware progettato per distruggere dati in modo irreversibile.',en:'Malware designed to irreversibly destroy data.'},
'cryptojacking':{it:'Uso non autorizzato di risorse computazionali per mining di criptovalute.',en:'Unauthorized use of computing resources for cryptocurrency mining.'},

// === ITALIAN LEGAL ===
'codice ateco':{it:'Classificazione ISTAT delle attività economiche — usato per determinare il settore NIS2.',en:'ISTAT classification of economic activities — used to determine NIS2 sector.'},
'organi direttivi':{it:'CDA e dirigenti responsabili per Art. 20 NIS2 — approvazione e supervisione.',en:'Board and executives responsible under NIS2 Art. 20 — approval and oversight.'},
'soggetto obbligato':{it:'Entità che rientra nell\'ambito di applicazione della NIS2.',en:'Entity falling within the scope of NIS2.'},
'registro operatori':{it:'Registro presso ACN delle entità soggette a NIS2 in Italia.',en:'Registry at ACN of entities subject to NIS2 in Italy.'},
'sanzione amministrativa':{it:'Multa pecuniaria applicata dall\'autorità competente per violazioni NIS2.',en:'Administrative fine applied by the competent authority for NIS2 violations.'},

// === SUPPLY CHAIN ===
'vendor lock-in':{it:'Dipendenza da un singolo fornitore che rende difficile il cambio.',en:'Dependency on a single vendor making it difficult to switch.'},
'due diligence':{it:'Dovere di diligenza — valutazione approfondita di un fornitore prima della contrattazione.',en:'Due diligence — thorough assessment of a vendor before contracting.'},
'risk scoring':{it:'Punteggio di rischio — valutazione numerica del livello di rischio di un fornitore.',en:'Risk scoring — numerical assessment of a vendor\'s risk level.'},
'sbom':{it:'Software Bill of Materials — elenco completo delle componenti software e dipendenze.',en:'Software Bill of Materials — complete list of software components and dependencies.'},
'third party risk':{it:'Rischio di terze parti — rischio derivante da fornitori e partner esterni.',en:'Third-party risk — risk from external vendors and partners.'},

// === PRIVACY & LEGAL ===
'data controller':{it:'Titolare del trattamento — determina finalità e mezzi del trattamento dati.',en:'Data controller — determines purposes and means of data processing.'},
'data processor':{it:'Responsabile del trattamento — tratta dati per conto del titolare.',en:'Data processor — processes data on behalf of the controller.'},
'dpia':{it:'Data Protection Impact Assessment — valutazione d\'impatto sulla protezione dei dati.',en:'Data Protection Impact Assessment — assessment of impact on data protection.'},
'breach notification':{it:'Notifica di violazione — obbligo di comunicazione incidente a autorità e interessati.',en:'Breach notification — obligation to communicate incident to authorities and affected parties.'},

// === TRAINING ===
'security awareness':{it:'Consapevolezza sulla sicurezza — programma di formazione per tutti i dipendenti (Art. 21(2)(g)).',en:'Security awareness — training program for all employees (Art. 21(2)(g)).'},
'cyber hygiene':{it:'Igiene informatica — pratiche base di sicurezza: password, aggiornamenti, configurazioni.',en:'Cyber hygiene — basic security practices: passwords, updates, configurations.'},
'phishing simulation':{it:'Simulazione di phishing — test di consapevolezza con email finte controllate.',en:'Phishing simulation — awareness test with controlled fake emails.'},
'incident drill':{it:'Esercitazione di risposta incidenti — simulazione pratica di uno scenario di attacco.',en:'Incident drill — practical simulation of an attack scenario.'},

// === METRICS ===
'mttd':{it:'Mean Time to Detect — tempo medio di rilevamento di un incidente.',en:'Mean Time to Detect — average time to detect an incident.'},
'mttr':{it:'Mean Time to Respond — tempo medio di risposta a un incidente.',en:'Mean Time to Respond — average time to respond to an incident.'},
'mtta':{it:'Mean Time to Acknowledge — tempo medio di presa in carico di un alert.',en:'Mean Time to Acknowledge — average time to acknowledge an alert.'},
'dwell time':{it:'Tempo di permanenza — durata della presenza dell\'attaccante prima del rilevamento.',en:'Dwell time — duration of attacker presence before detection.'},
'false positive':{it:'Falso positivo — allarme generato per un evento non malevolo.',en:'False positive — alert generated for a non-malicious event.'},
'mean time to patch':{it:'Tempo medio di applicazione patch — KPI per gestione vulnerabilità.',en:'Mean time to patch — KPI for vulnerability management.'},

// === ADDITIONAL STANDARDS ===
'iso 22301':{it:'Standard per sistemi di gestione della continuità operativa.',en:'Standard for business continuity management systems.'},
'iso 27035':{it:'Standard per la gestione degli incidenti di sicurezza delle informazioni.',en:'Standard for information security incident management.'},
'iso 31000':{it:'Standard per la gestione del rischio — principi e linee guida.',en:'Standard for risk management — principles and guidelines.'},
'nist sp 800-53':{it:'Catalogo NIST di controlli di sicurezza per sistemi informativi federali.',en:'NIST catalog of security controls for federal information systems.'},
'cobit':{it:'Control Objectives for Information Technologies — framework governance IT.',en:'Control Objectives for Information Technologies — IT governance framework.'},
'itil':{it:'Information Technology Infrastructure Library — framework per gestione servizi IT.',en:'Information Technology Infrastructure Library — IT service management framework.'},

// === INFRASTRUCTURE ===
'load balancer':{it:'Bilanciatore di carico — distribuisce traffico tra più server.',en:'Load balancer — distributes traffic across multiple servers.'},
'cdn':{it:'Content Delivery Network — rete di distribuzione contenuti per performance e resilienza.',en:'Content Delivery Network — content distribution network for performance and resilience.'},
'dns':{it:'Domain Name System — sistema di risoluzione nomi di dominio in indirizzi IP.',en:'Domain Name System — system for resolving domain names to IP addresses.'},
'tld':{it:'Top-Level Domain — dominio di primo livello (.it, .com, .eu).',en:'Top-Level Domain — first-level domain (.it, .com, .eu).'},
'bgp':{it:'Border Gateway Protocol — protocollo di routing inter-dominio per Internet.',en:'Border Gateway Protocol — inter-domain routing protocol for the Internet.'},
'proxy':{it:'Proxy — intermediario che filtra e instrada il traffico di rete.',en:'Proxy — intermediary that filters and routes network traffic.'},
'reverse proxy':{it:'Proxy inverso — intermediario lato server per sicurezza e bilanciamento.',en:'Reverse proxy — server-side intermediary for security and load balancing.'},
'api gateway':{it:'Gateway API — punto di ingresso centralizzato per le API con sicurezza e rate limiting.',en:'API gateway — centralized entry point for APIs with security and rate limiting.'},

// === CONFIGURATION ===
'cis benchmark':{it:'CIS Benchmark — configurazione di sicurezza raccomandata per sistemi operativi e servizi.',en:'CIS Benchmark — recommended security configuration for operating systems and services.'},
'baseline':{it:'Baseline di sicurezza — configurazione minima di sicurezza standard per tutti i sistemi.',en:'Security baseline — standard minimum security configuration for all systems.'},
'golden image':{it:'Immagine dorata — template di sistema pre-configurato e hardened per deploy rapido.',en:'Golden image — pre-configured and hardened system template for rapid deployment.'},
'iac':{it:'Infrastructure as Code — gestione infrastruttura tramite codice versionato.',en:'Infrastructure as Code — infrastructure management through versioned code.'},
'configuration drift':{it:'Deriva di configurazione — differenza tra configurazione attesa e reale.',en:'Configuration drift — difference between expected and actual configuration.'},
}

export default G2
