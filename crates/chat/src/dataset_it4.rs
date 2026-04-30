pub fn italian_practical() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        // BACKUP & DISASTER RECOVERY
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
        // INCIDENT RESPONSE
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
        // ASSET MANAGEMENT
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
        // MULTI-FACTOR AUTHENTICATION
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
        // VULNERABILITY MANAGEMENT
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
        // SUPPLY CHAIN
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
    ]
}
