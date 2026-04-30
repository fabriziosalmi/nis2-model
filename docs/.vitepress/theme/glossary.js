// NIS2 Glossary — bilingual term definitions for inline augmentation.
// Format: key (lowercase match) → { it: definition, en: definition }
// Split into sections for maintainability.

const G = {
// === NIS2 DIRECTIVE & ARTICLES ===
'nis2':{it:'Direttiva (UE) 2022/2555 sulla sicurezza delle reti e dei sistemi informativi.',en:'Directive (EU) 2022/2555 on network and information system security.'},
'art. 20':{it:'Governance: gli organi direttivi approvano e sovraintendono le misure di cybersecurity.',en:'Governance: management bodies approve and oversee cybersecurity measures.'},
'art. 21':{it:'Misure di gestione del rischio di cybersecurity che le entità devono adottare.',en:'Cybersecurity risk-management measures entities must adopt.'},
'art. 23':{it:'Obblighi di segnalazione incidenti: preallarme 24h, notifica 72h, relazione 30gg.',en:'Incident reporting: early warning 24h, notification 72h, final report 30d.'},
'art. 24':{it:'Database europeo delle vulnerabilità gestito da ENISA.',en:'European vulnerability database managed by ENISA.'},
'art. 29':{it:'Accordi di condivisione delle informazioni sulla cybersecurity.',en:'Cybersecurity information-sharing arrangements.'},
'art. 32':{it:'Misure di vigilanza per entità essenziali (audit proattivi).',en:'Supervisory measures for essential entities (proactive audits).'},
'art. 33':{it:'Misure di vigilanza per entità importanti (audit ex-post).',en:'Supervisory measures for important entities (ex-post audits).'},
'art. 34':{it:'Sanzioni: fino a 10M EUR o 2% fatturato per entità essenziali.',en:'Penalties: up to 10M EUR or 2% turnover for essential entities.'},
'allegato i':{it:'Settori ad alta criticità: energia, trasporti, banche, sanità, acqua, infrastruttura digitale, PA.',en:'High-criticality sectors: energy, transport, banking, health, water, digital infrastructure, public admin.'},
'allegato ii':{it:'Altri settori critici: posta, rifiuti, chimica, alimentare, manifattura, ricerca, digitale.',en:'Other critical sectors: postal, waste, chemicals, food, manufacturing, research, digital.'},
'annex i':{it:'Settori ad alta criticità della NIS2.',en:'NIS2 high-criticality sectors.'},
'annex ii':{it:'Altri settori critici della NIS2.',en:'NIS2 other critical sectors.'},

// === ENTITIES & ROLES ===
'entità essenziale':{it:'Organizzazione in settore Allegato I sopra soglia dimensionale, soggetta a vigilanza proattiva.',en:'Organization in Annex I sector above size threshold, subject to proactive supervision.'},
'entità importante':{it:'Organizzazione in settore Allegato II sopra soglia, soggetta a vigilanza ex-post.',en:'Organization in Annex II sector above threshold, subject to ex-post supervision.'},
'essential entity':{it:'Entità essenziale: settore Allegato I, vigilanza proattiva Art. 32.',en:'Essential entity: Annex I sector, proactive supervision Art. 32.'},
'important entity':{it:'Entità importante: settore Allegato II, vigilanza ex-post Art. 33.',en:'Important entity: Annex II sector, ex-post supervision Art. 33.'},
'ciso':{it:'Chief Information Security Officer — responsabile sicurezza informatica.',en:'Chief Information Security Officer — head of information security.'},
'dpo':{it:'Data Protection Officer — responsabile protezione dati personali (GDPR).',en:'Data Protection Officer — responsible for personal data protection (GDPR).'},
'cda':{it:'Consiglio di Amministrazione — organo direttivo responsabile per Art. 20.',en:'Board of Directors — management body responsible under Art. 20.'},

// === AUTHORITIES & ORGANIZATIONS ===
'csirt':{it:'Computer Security Incident Response Team — team di risposta incidenti designato per Art. 10.',en:'Computer Security Incident Response Team — designated incident response per Art. 10.'},
'enisa':{it:'Agenzia dell\'UE per la cybersicurezza — coordina standard e vulnerabilità.',en:'EU Agency for Cybersecurity — coordinates standards and vulnerabilities.'},
'acn':{it:'Agenzia per la Cybersicurezza Nazionale — autorità competente NIS2 in Italia.',en:'National Cybersecurity Agency — NIS2 competent authority in Italy.'},
'bsi':{it:'Bundesamt für Sicherheit — autorità competente NIS2 in Germania.',en:'Federal Office for Information Security — NIS2 authority in Germany.'},
'anssi':{it:'Agence nationale de la sécurité — autorità competente NIS2 in Francia.',en:'National Agency for Information Systems Security — NIS2 authority in France.'},
'cert':{it:'Computer Emergency Response Team — equivalente di CSIRT.',en:'Computer Emergency Response Team — equivalent to CSIRT.'},

// === COMPLIANCE FRAMEWORKS ===
'iso 27001':{it:'Standard internazionale per sistemi di gestione della sicurezza delle informazioni (ISMS).',en:'International standard for information security management systems (ISMS).'},
'iso 27005':{it:'Standard per la gestione del rischio di sicurezza delle informazioni.',en:'Standard for information security risk management.'},
'nist csf':{it:'NIST Cybersecurity Framework — framework USA per gestione rischio cyber.',en:'NIST Cybersecurity Framework — US framework for cyber risk management.'},
'cis controls':{it:'Center for Internet Security Controls — 18 controlli prioritizzati di sicurezza.',en:'Center for Internet Security Controls — 18 prioritized security controls.'},
'gdpr':{it:'Regolamento Generale sulla Protezione dei Dati — protezione dati personali UE.',en:'General Data Protection Regulation — EU personal data protection.'},
'dora':{it:'Digital Operational Resilience Act — resilienza operativa per il settore finanziario.',en:'Digital Operational Resilience Act — operational resilience for financial sector.'},
'eidas':{it:'Regolamento UE su identificazione elettronica e servizi fiduciari.',en:'EU regulation on electronic identification and trust services.'},
'soc 2':{it:'Service Organization Control 2 — standard di audit per provider di servizi.',en:'Service Organization Control 2 — audit standard for service providers.'},

// === RISK MANAGEMENT ===
'bia':{it:'Business Impact Analysis — analisi dell\'impatto operativo sulle attività.',en:'Business Impact Analysis — assessment of operational impact on activities.'},
'rto':{it:'Recovery Time Objective — tempo massimo di inattività tollerabile.',en:'Recovery Time Objective — maximum acceptable downtime.'},
'rpo':{it:'Recovery Point Objective — perdita dati massima accettabile.',en:'Recovery Point Objective — maximum acceptable data loss.'},
'risk appetite':{it:'Propensione al rischio — livello di rischio accettabile per l\'organizzazione.',en:'Risk appetite — level of risk acceptable to the organization.'},
'rischio residuo':{it:'Rischio che rimane dopo l\'applicazione delle misure di mitigazione.',en:'Risk remaining after mitigation measures are applied.'},
'threat modeling':{it:'Modellazione delle minacce — identificazione sistematica delle minacce in fase di design.',en:'Threat modeling — systematic identification of threats during design phase.'},
'cvss':{it:'Common Vulnerability Scoring System — sistema di punteggio severità vulnerabilità (0-10).',en:'Common Vulnerability Scoring System — vulnerability severity scoring (0-10).'},
'cve':{it:'Common Vulnerabilities and Exposures — identificatore univoco per vulnerabilità note.',en:'Common Vulnerabilities and Exposures — unique identifier for known vulnerabilities.'},

// === SECURITY OPERATIONS ===
'siem':{it:'Security Information and Event Management — correlazione log e rilevamento minacce.',en:'Security Information and Event Management — log correlation and threat detection.'},
'soc':{it:'Security Operations Center — centro operativo per monitoraggio sicurezza 24/7.',en:'Security Operations Center — operational center for 24/7 security monitoring.'},
'soar':{it:'Security Orchestration, Automation and Response — automazione risposta incidenti.',en:'Security Orchestration, Automation and Response — automated incident response.'},
'edr':{it:'Endpoint Detection and Response — rilevamento e risposta sugli endpoint.',en:'Endpoint Detection and Response — detection and response on endpoints.'},
'xdr':{it:'Extended Detection and Response — rilevamento cross-platform (endpoint+rete+cloud).',en:'Extended Detection and Response — cross-platform detection (endpoint+network+cloud).'},
'mdr':{it:'Managed Detection and Response — servizio gestito di rilevamento e risposta.',en:'Managed Detection and Response — managed detection and response service.'},
'ids':{it:'Intrusion Detection System — sistema di rilevamento intrusioni.',en:'Intrusion Detection System — system for detecting intrusions.'},
'ips':{it:'Intrusion Prevention System — sistema di prevenzione intrusioni attiva.',en:'Intrusion Prevention System — active intrusion prevention system.'},
'waf':{it:'Web Application Firewall — firewall specializzato per applicazioni web.',en:'Web Application Firewall — specialized firewall for web applications.'},
'dlp':{it:'Data Loss Prevention — prevenzione perdita/esfiltrazione dati.',en:'Data Loss Prevention — preventing data loss/exfiltration.'},
'ndr':{it:'Network Detection and Response — rilevamento e risposta a livello rete.',en:'Network Detection and Response — network-level detection and response.'},

// === ACCESS CONTROL ===
'mfa':{it:'Multi-Factor Authentication — autenticazione a più fattori (Art. 21(2)(j)).',en:'Multi-Factor Authentication — multi-factor authentication (Art. 21(2)(j)).'},
'pam':{it:'Privileged Access Management — gestione accessi privilegiati.',en:'Privileged Access Management — managing privileged access.'},
'iam':{it:'Identity and Access Management — gestione identità e accessi.',en:'Identity and Access Management — managing identities and access.'},
'rbac':{it:'Role-Based Access Control — controllo accessi basato su ruoli.',en:'Role-Based Access Control — access control based on roles.'},
'sso':{it:'Single Sign-On — accesso unico a più sistemi con una sola autenticazione.',en:'Single Sign-On — single authentication for multiple systems.'},
'fido2':{it:'Standard di autenticazione passwordless basato su chiavi crittografiche.',en:'Passwordless authentication standard based on cryptographic keys.'},
'totp':{it:'Time-based One-Time Password — password monouso temporizzata (es. Google Authenticator).',en:'Time-based One-Time Password — time-limited one-time password.'},
'ldap':{it:'Lightweight Directory Access Protocol — protocollo per directory utenti.',en:'Lightweight Directory Access Protocol — protocol for user directories.'},
'saml':{it:'Security Assertion Markup Language — standard per SSO federato.',en:'Security Assertion Markup Language — standard for federated SSO.'},
'oauth':{it:'Open Authorization — protocollo di autorizzazione delegata.',en:'Open Authorization — delegated authorization protocol.'},
'zero trust':{it:'Modello di sicurezza che non assume fiducia implicita — verifica sempre.',en:'Security model that assumes no implicit trust — always verify.'},
'least privilege':{it:'Principio del minimo privilegio — solo accessi strettamente necessari.',en:'Principle of least privilege — only strictly necessary access.'},

// === ENCRYPTION ===
'aes':{it:'Advanced Encryption Standard — algoritmo di cifratura simmetrica (AES-256 raccomandato).',en:'Advanced Encryption Standard — symmetric encryption algorithm (AES-256 recommended).'},
'aes-256':{it:'Cifratura simmetrica a 256 bit — standard raccomandato da ENISA per dati a riposo.',en:'256-bit symmetric encryption — ENISA-recommended standard for data at rest.'},
'tls':{it:'Transport Layer Security — protocollo di cifratura per dati in transito (minimo TLS 1.2).',en:'Transport Layer Security — encryption protocol for data in transit (minimum TLS 1.2).'},
'mtls':{it:'Mutual TLS — autenticazione bidirezionale tra client e server.',en:'Mutual TLS — bidirectional authentication between client and server.'},
'pki':{it:'Public Key Infrastructure — infrastruttura a chiave pubblica per certificati digitali.',en:'Public Key Infrastructure — public key infrastructure for digital certificates.'},
'hsm':{it:'Hardware Security Module — modulo hardware per protezione chiavi crittografiche.',en:'Hardware Security Module — hardware module for cryptographic key protection.'},
'kms':{it:'Key Management System — sistema di gestione delle chiavi crittografiche.',en:'Key Management System — system for managing cryptographic keys.'},
'hsts':{it:'HTTP Strict Transport Security — header che forza connessioni HTTPS.',en:'HTTP Strict Transport Security — header enforcing HTTPS connections.'},

// === NETWORK ===
'dmz':{it:'Demilitarized Zone — segmento di rete tra internet e rete interna.',en:'Demilitarized Zone — network segment between internet and internal network.'},
'vlan':{it:'Virtual LAN — segmentazione logica della rete.',en:'Virtual LAN — logical network segmentation.'},
'vpn':{it:'Virtual Private Network — tunnel cifrato per accesso remoto sicuro.',en:'Virtual Private Network — encrypted tunnel for secure remote access.'},
'ztna':{it:'Zero Trust Network Access — accesso di rete a fiducia zero, alternativa moderna alla VPN.',en:'Zero Trust Network Access — zero trust network access, modern VPN alternative.'},
'dnssec':{it:'DNS Security Extensions — estensioni di sicurezza per integrità DNS.',en:'DNS Security Extensions — security extensions for DNS integrity.'},
'nat':{it:'Network Address Translation — traduzione indirizzi di rete.',en:'Network Address Translation — network address translation.'},
'sdwan':{it:'Software-Defined WAN — WAN definita via software per reti distribuite.',en:'Software-Defined WAN — software-defined WAN for distributed networks.'},

// === INCIDENT RESPONSE ===
'ioc':{it:'Indicator of Compromise — indicatore di compromissione (IP, hash, dominio malevolo).',en:'Indicator of Compromise — compromise indicator (IP, hash, malicious domain).'},
'ttp':{it:'Tactics, Techniques and Procedures — comportamenti dell\'attaccante (framework MITRE ATT&CK).',en:'Tactics, Techniques and Procedures — attacker behaviors (MITRE ATT&CK framework).'},
'mitre att&ck':{it:'Framework di classificazione delle tattiche e tecniche di attacco.',en:'Framework for classifying attack tactics and techniques.'},
'rca':{it:'Root Cause Analysis — analisi della causa radice di un incidente.',en:'Root Cause Analysis — analysis of the root cause of an incident.'},
'forensics':{it:'Analisi forense digitale — raccolta e analisi prove digitali post-incidente.',en:'Digital forensics — collection and analysis of digital evidence post-incident.'},
'tabletop':{it:'Esercitazione tabletop — simulazione scenari di incidente a tavolino.',en:'Tabletop exercise — simulated incident scenario discussion.'},
'playbook':{it:'Playbook — procedura operativa predefinita per risposta a specifici tipi di incidente.',en:'Playbook — predefined operational procedure for responding to specific incident types.'},

// === THREATS ===
'ransomware':{it:'Malware che cifra i dati e richiede riscatto per il ripristino.',en:'Malware that encrypts data and demands ransom for recovery.'},
'phishing':{it:'Attacco di ingegneria sociale via email per rubare credenziali o installare malware.',en:'Social engineering attack via email to steal credentials or install malware.'},
'spear phishing':{it:'Phishing mirato a individui specifici con contenuto personalizzato.',en:'Targeted phishing aimed at specific individuals with personalized content.'},
'apt':{it:'Advanced Persistent Threat — minaccia avanzata persistente (attaccanti statali/organizzati).',en:'Advanced Persistent Threat — advanced persistent threat (state/organized attackers).'},
'ddos':{it:'Distributed Denial of Service — attacco distribuito per rendere un servizio indisponibile.',en:'Distributed Denial of Service — distributed attack to make a service unavailable.'},
'supply chain attack':{it:'Attacco alla catena di fornitura — compromissione tramite fornitori/software terzi.',en:'Supply chain attack — compromise through third-party vendors/software.'},
'credential stuffing':{it:'Attacco che usa credenziali rubate da altri breach per accedere a sistemi.',en:'Attack using credentials stolen from other breaches to access systems.'},
'lateral movement':{it:'Movimento laterale — tecnica per spostarsi tra sistemi dopo la compromissione iniziale.',en:'Lateral movement — technique to move between systems after initial compromise.'},
'data exfiltration':{it:'Esfiltrazione dati — trasferimento non autorizzato di dati fuori dalla rete.',en:'Data exfiltration — unauthorized transfer of data outside the network.'},
'social engineering':{it:'Ingegneria sociale — manipolazione psicologica per ottenere accesso o informazioni.',en:'Social engineering — psychological manipulation to gain access or information.'},

// === DEVELOPMENT ===
'sdlc':{it:'Software Development Life Cycle — ciclo di vita dello sviluppo software sicuro.',en:'Software Development Life Cycle — secure software development lifecycle.'},
'sast':{it:'Static Application Security Testing — analisi statica del codice sorgente.',en:'Static Application Security Testing — static source code analysis.'},
'dast':{it:'Dynamic Application Security Testing — test di sicurezza su applicazione in esecuzione.',en:'Dynamic Application Security Testing — security testing on running application.'},
'sca':{it:'Software Composition Analysis — analisi delle dipendenze open source per vulnerabilità.',en:'Software Composition Analysis — open source dependency analysis for vulnerabilities.'},
'owasp':{it:'Open Web Application Security Project — standard e guide per sicurezza applicativa.',en:'Open Web Application Security Project — standards and guides for application security.'},
'ci/cd':{it:'Continuous Integration/Continuous Delivery — pipeline automatizzata di build e deploy.',en:'Continuous Integration/Continuous Delivery — automated build and deploy pipeline.'},
'devsecops':{it:'Integrazione della sicurezza nel ciclo DevOps — sicurezza come codice.',en:'Integration of security into DevOps lifecycle — security as code.'},

// === EMAIL SECURITY ===
'spf':{it:'Sender Policy Framework — record DNS che autorizza IP a inviare email per il dominio.',en:'Sender Policy Framework — DNS record authorizing IPs to send email for the domain.'},
'dkim':{it:'DomainKeys Identified Mail — firma crittografica sulle email per autenticità.',en:'DomainKeys Identified Mail — cryptographic signature on emails for authenticity.'},
'dmarc':{it:'Domain-based Message Authentication — policy di enforcement per SPF+DKIM.',en:'Domain-based Message Authentication — enforcement policy for SPF+DKIM.'},
'mta-sts':{it:'SMTP MTA Strict Transport Security — forza TLS tra server email.',en:'SMTP MTA Strict Transport Security — enforces TLS between mail servers.'},

// === BUSINESS CONTINUITY ===
'bcp':{it:'Business Continuity Plan — piano di continuità operativa (Art. 21(2)(c)).',en:'Business Continuity Plan — business continuity plan (Art. 21(2)(c)).'},
'drp':{it:'Disaster Recovery Plan — piano di ripristino in caso di disastro.',en:'Disaster Recovery Plan — disaster recovery plan.'},
'failover':{it:'Failover — passaggio automatico a sistema di backup in caso di guasto.',en:'Failover — automatic switch to backup system in case of failure.'},
'air gap':{it:'Air gap — isolamento fisico di un sistema dalla rete per protezione massima.',en:'Air gap — physical isolation of a system from the network for maximum protection.'},
'sla':{it:'Service Level Agreement — accordo sui livelli di servizio con fornitori.',en:'Service Level Agreement — service level agreement with providers.'},

// === COMPLIANCE & AUDIT ===
'gap analysis':{it:'Analisi dei gap — valutazione delle differenze tra stato attuale e requisiti NIS2.',en:'Gap analysis — assessment of differences between current state and NIS2 requirements.'},
'maturity model':{it:'Modello di maturità — framework per valutare il livello di maturità della sicurezza.',en:'Maturity model — framework for assessing security maturity level.'},
'kpi':{it:'Key Performance Indicator — indicatore chiave di prestazione per misurare l\'efficacia.',en:'Key Performance Indicator — key metric for measuring effectiveness.'},
'kri':{it:'Key Risk Indicator — indicatore chiave di rischio per monitoraggio proattivo.',en:'Key Risk Indicator — key risk metric for proactive monitoring.'},

// === CLOUD ===
'iaas':{it:'Infrastructure as a Service — infrastruttura cloud (server, storage, rete).',en:'Infrastructure as a Service — cloud infrastructure (servers, storage, network).'},
'paas':{it:'Platform as a Service — piattaforma cloud per sviluppo applicazioni.',en:'Platform as a Service — cloud platform for application development.'},
'saas':{it:'Software as a Service — software accessibile via cloud (es. Office 365).',en:'Software as a Service — cloud-accessible software (e.g. Office 365).'},
'cspm':{it:'Cloud Security Posture Management — gestione della postura di sicurezza cloud.',en:'Cloud Security Posture Management — cloud security posture management.'},
'casb':{it:'Cloud Access Security Broker — intermediario di sicurezza per servizi cloud.',en:'Cloud Access Security Broker — security intermediary for cloud services.'},

// === DATA PROTECTION ===
'data at rest':{it:'Dati a riposo — dati memorizzati su disco, database o backup.',en:'Data at rest — data stored on disk, database, or backup.'},
'data in transit':{it:'Dati in transito — dati trasmessi attraverso la rete.',en:'Data in transit — data transmitted across the network.'},
'data classification':{it:'Classificazione dei dati — categorizzazione per livello di sensibilità/criticità.',en:'Data classification — categorization by sensitivity/criticality level.'},
'pseudonymization':{it:'Pseudonimizzazione — tecnica per ridurre l\'identificabilità dei dati personali.',en:'Pseudonymization — technique to reduce identifiability of personal data.'},
'data breach':{it:'Violazione dei dati — accesso non autorizzato o perdita di dati.',en:'Data breach — unauthorized access or loss of data.'},

// === GOVERNANCE ===
'isms':{it:'Information Security Management System — sistema di gestione della sicurezza (ISO 27001).',en:'Information Security Management System — security management system (ISO 27001).'},
'risk register':{it:'Registro dei rischi — documento che cataloga rischi, impatti e misure di mitigazione.',en:'Risk register — document cataloging risks, impacts, and mitigation measures.'},
'security policy':{it:'Policy di sicurezza — documento fondamentale che definisce le regole di sicurezza.',en:'Security policy — foundational document defining security rules.'},
'acceptable use':{it:'Policy di uso accettabile — regole per l\'uso corretto delle risorse IT aziendali.',en:'Acceptable use policy — rules for proper use of corporate IT resources.'},

// === ASSET MANAGEMENT ===
'cmdb':{it:'Configuration Management Database — database di gestione delle configurazioni IT.',en:'Configuration Management Database — IT configuration management database.'},
'asset inventory':{it:'Inventario degli asset — registro completo di hardware, software e dati.',en:'Asset inventory — complete register of hardware, software, and data.'},
'shadow it':{it:'Shadow IT — sistemi e servizi IT usati senza approvazione del dipartimento IT.',en:'Shadow IT — IT systems and services used without IT department approval.'},
'eol':{it:'End of Life — fine del ciclo di vita di un prodotto, senza più aggiornamenti di sicurezza.',en:'End of Life — end of product lifecycle, no more security updates.'},

// === VULNERABILITY MANAGEMENT ===
'patch management':{it:'Gestione delle patch — processo di applicazione aggiornamenti di sicurezza.',en:'Patch management — process of applying security updates.'},
'vulnerability disclosure':{it:'Divulgazione coordinata delle vulnerabilità (Art. 21(2)(e)).',en:'Coordinated vulnerability disclosure (Art. 21(2)(e)).'},
'pentest':{it:'Penetration test — test di penetrazione per verificare la sicurezza dei sistemi.',en:'Penetration test — security testing to verify system security.'},
'bug bounty':{it:'Programma di ricompensa per segnalazione vulnerabilità da parte di ricercatori esterni.',en:'Reward program for vulnerability reports from external researchers.'},
'kev':{it:'Known Exploited Vulnerabilities — catalogo CISA delle vulnerabilità attivamente sfruttate.',en:'Known Exploited Vulnerabilities — CISA catalog of actively exploited vulnerabilities.'},

// === MONITORING ===
'netflow':{it:'Protocollo per raccolta metadati del traffico di rete per analisi.',en:'Protocol for collecting network traffic metadata for analysis.'},
'syslog':{it:'Standard per l\'invio di messaggi di log attraverso la rete.',en:'Standard for sending log messages across the network.'},
'anomaly detection':{it:'Rilevamento anomalie — identificazione di comportamenti devianti dalla baseline.',en:'Anomaly detection — identification of behaviors deviating from baseline.'},

// === PROTOCOLS ===
'ssh':{it:'Secure Shell — protocollo per accesso remoto sicuro a sistemi.',en:'Secure Shell — protocol for secure remote access to systems.'},
'https':{it:'HTTP Secure — protocollo web cifrato con TLS.',en:'HTTP Secure — encrypted web protocol using TLS.'},
'ipsec':{it:'Internet Protocol Security — suite di protocolli per comunicazioni IP sicure.',en:'Internet Protocol Security — protocol suite for secure IP communications.'},
'radius':{it:'Remote Authentication Dial-In User Service — protocollo di autenticazione di rete.',en:'Remote Authentication Dial-In User Service — network authentication protocol.'},
'802.1x':{it:'Standard IEEE per controllo accessi alla rete basato su porte (NAC).',en:'IEEE standard for port-based network access control (NAC).'},
'snmp':{it:'Simple Network Management Protocol — protocollo per gestione dispositivi di rete.',en:'Simple Network Management Protocol — protocol for network device management.'},

// === MOBILE / REMOTE ===
'mdm':{it:'Mobile Device Management — gestione centralizzata dei dispositivi mobili.',en:'Mobile Device Management — centralized management of mobile devices.'},
'byod':{it:'Bring Your Own Device — politica di utilizzo dispositivi personali in azienda.',en:'Bring Your Own Device — policy for using personal devices in the workplace.'},
'nac':{it:'Network Access Control — controllo dell\'accesso alla rete basato su compliance del dispositivo.',en:'Network Access Control — network access control based on device compliance.'},

// === LEGAL ITALIAN ===
'd.lgs. 138/2024':{it:'Decreto legislativo di recepimento della NIS2 in Italia, in vigore dal 16 ottobre 2024.',en:'Italian legislative decree transposing NIS2, effective from 16 October 2024.'},
'autorita competente':{it:'Autorità designata dallo stato membro per NIS2 (ACN in Italia, BSI in Germania).',en:'Authority designated by the member state for NIS2 (ACN in Italy, BSI in Germany).'},
'recepimento':{it:'Trasposizione di una direttiva UE in legislazione nazionale (scadenza: 17 ottobre 2024).',en:'Transposition of an EU directive into national law (deadline: 17 October 2024).'},
'soglia dimensionale':{it:'50+ dipendenti O 10M+ EUR fatturato — criterio per rientrare in NIS2.',en:'50+ employees OR 10M+ EUR revenue — criterion for falling under NIS2.'},
}

import G2 from './glossary2.js'
import G3 from './glossary3.js'

// Merge all glossaries
const ALL = { ...G, ...G2, ...G3 }

// Build lookup index (lowercase keys for matching, longest first to avoid partial matches)
const KEYS = Object.keys(ALL).sort((a, b) => b.length - a.length)

/**
 * Find glossary terms present in a text string.
 * Returns array of { term, it, en } objects, deduplicated, max 6.
 */
export function findTerms(text) {
  const low = text.toLowerCase()
  const found = []
  const seen = new Set()
  for (const k of KEYS) {
    if (found.length >= 6) break
    if (low.includes(k) && !seen.has(k)) {
      seen.add(k)
      found.push({ term: k, ...ALL[k] })
    }
  }
  return found
}

/**
 * Get definition for a specific term.
 */
export function getDefinition(term, lang) {
  const entry = ALL[term.toLowerCase()]
  return entry ? entry[lang] || entry.en : null
}

/**
 * Total number of glossary terms.
 */
export const GLOSSARY_SIZE = Object.keys(ALL).length
