// Complete i18n module for the NIS2 chatbot.
// All user-facing strings in one place.

export const translations = {
  it: {
    title: 'NIS2 Compliance Engine',
    sub: 'Deterministico · Zero LLM',
    placeholder: 'Chiedi qualcosa sulla conformità NIS2...',
    loading: 'Caricamento knowledge base...',
    welcome: 'Chiedi qualsiasi cosa su NIS2',
    welcomeSub: 'risposte pre-calcolate · IT/EN · Art. 20, 21, 23, 34',
    miss: 'Domanda non trovata. Prova a riformulare o usa uno dei suggerimenti.',
    coverageLabel: 'Copertura',
    session: 'Sessione',
    queries: 'Domande',
    hits: 'Risposte',
    areas: 'Aree esplorate',
    recent: 'Recenti',
    linkProject: 'Progetto NIS2',
    linkEngine: 'Motore',
    categories: {
      applicability: 'Applicabilità',
      governance: 'Governance',
      access_control: 'Controllo accessi',
      encryption: 'Crittografia',
      incident_response: 'Risposta incidenti',
      business_continuity: 'Continuità operativa',
      supply_chain: 'Supply chain',
      vulnerability_mgmt: 'Gestione vulnerabilità',
      risk_assessment: 'Analisi rischi',
      network_security: 'Sicurezza di rete',
      detection: 'Rilevamento',
      email_security: 'Sicurezza email',
      documentation: 'Documentazione',
      remote_work: 'Lavoro remoto',
      physical: 'Sicurezza fisica',
      legal: 'Legale',
      sanctions: 'Sanzioni',
      asset_management: 'Gestione asset',
      development: 'Sviluppo sicuro',
    },
    suggestions: [
      'La mia azienda rientra nella NIS2?',
      'Da dove iniziare con NIS2?',
      'Ci hanno hackerato, cosa facciamo?',
      'Quanto costa adeguarsi a NIS2?',
      'Serve la crittografia?',
      'Quali sono le sanzioni NIS2?',
    ],
    missSuggestions: [
      'La mia azienda rientra nella NIS2?',
      'Da dove iniziare con NIS2?',
      'Quali sono gli obblighi NIS2?',
      'Quanto costa adeguarsi?',
    ],
  },
  en: {
    title: 'NIS2 Compliance Engine',
    sub: 'Deterministic · Zero LLM',
    placeholder: 'Ask anything about NIS2 compliance...',
    loading: 'Loading knowledge base...',
    welcome: 'Ask anything about NIS2',
    welcomeSub: 'pre-computed answers · IT/EN · Art. 20, 21, 23, 34',
    miss: 'Question not found. Try rephrasing or use a suggestion below.',
    coverageLabel: 'Coverage',
    session: 'Session',
    queries: 'Queries',
    hits: 'Hits',
    areas: 'Areas explored',
    recent: 'Recent',
    linkProject: 'NIS2 Project',
    linkEngine: 'Engine',
    categories: {
      applicability: 'Applicability',
      governance: 'Governance',
      access_control: 'Access Control',
      encryption: 'Encryption',
      incident_response: 'Incident Response',
      business_continuity: 'Business Continuity',
      supply_chain: 'Supply Chain',
      vulnerability_mgmt: 'Vulnerability Mgmt',
      risk_assessment: 'Risk Assessment',
      network_security: 'Network Security',
      detection: 'Detection',
      email_security: 'Email Security',
      documentation: 'Documentation',
      remote_work: 'Remote Work',
      physical: 'Physical Security',
      legal: 'Legal',
      sanctions: 'Sanctions',
      asset_management: 'Asset Management',
      development: 'Secure Development',
    },
    suggestions: [
      'Does NIS2 apply to my company?',
      'Where to start with NIS2?',
      'We got hacked, what do we do?',
      'What are the NIS2 obligations?',
      'Do we need encryption?',
      'What are the NIS2 sanctions?',
    ],
    missSuggestions: [
      'Does NIS2 apply to my company?',
      'Where to start with NIS2?',
      'What are the NIS2 obligations?',
      'What are the sanctions?',
    ],
  },
}

export function getStrings(lang) {
  return translations[lang] || translations.en
}

export function getCategoryName(catId, lang) {
  const t = getStrings(lang)
  const base = catId.replace(/_impl$/, '')
  return t.categories[base] || base.replace(/_/g, ' ')
}
