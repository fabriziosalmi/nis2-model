// Advanced answer formatter — structured HTML + contextual reference links.

const EURLEX_BASE = 'https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32022L2555'

// Map article numbers to EUR-Lex anchors and short descriptions
const ART_META = {
  '20': { anchor: '#art_20', en: 'Governance & board liability', it: 'Governance e responsabilità del CDA' },
  '21': { anchor: '#art_21', en: 'Cybersecurity risk-management measures', it: 'Misure di gestione del rischio' },
  '23': { anchor: '#art_23', en: 'Incident reporting obligations', it: 'Obblighi di notifica incidenti' },
  '24': { anchor: '#art_24', en: 'European vulnerability database', it: 'Database europeo vulnerabilità' },
  '29': { anchor: '#art_29', en: 'Information sharing arrangements', it: 'Accordi di condivisione informazioni' },
  '32': { anchor: '#art_32', en: 'Supervisory measures (essential)', it: 'Misure di supervisione (essenziali)' },
  '33': { anchor: '#art_33', en: 'Supervisory measures (important)', it: 'Misure di supervisione (importanti)' },
  '34': { anchor: '#art_34', en: 'Penalties & enforcement', it: 'Sanzioni e applicazione' },
}

// Category → relevant nis2-public resources
const CAT_LINKS = {
  it: {
    applicability: { label: 'Verifica applicabilità NIS2', url: 'https://github.com/fabriziosalmi/nis2-public' },
    governance: { label: 'Framework governance NIS2', url: 'https://github.com/fabriziosalmi/nis2-public' },
    incident_response: { label: 'Procedure notifica incidenti', url: 'https://github.com/fabriziosalmi/nis2-public' },
    sanctions: { label: 'Regime sanzionatorio NIS2', url: 'https://github.com/fabriziosalmi/nis2-public' },
    supply_chain: { label: 'Sicurezza supply chain', url: 'https://github.com/fabriziosalmi/nis2-public' },
  },
  en: {
    applicability: { label: 'NIS2 applicability checker', url: 'https://github.com/fabriziosalmi/nis2-public' },
    governance: { label: 'NIS2 governance framework', url: 'https://github.com/fabriziosalmi/nis2-public' },
    incident_response: { label: 'Incident notification procedures', url: 'https://github.com/fabriziosalmi/nis2-public' },
    sanctions: { label: 'NIS2 penalties regime', url: 'https://github.com/fabriziosalmi/nis2-public' },
    supply_chain: { label: 'Supply chain security', url: 'https://github.com/fabriziosalmi/nis2-public' },
  },
}

// Escape HTML entities to prevent XSS via v-html injection.
function escapeHtml(text) {
  return text
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#039;')
}

// Convert **bold** markdown to <strong> tags (applied after escaping).
function convertBold(text) {
  return text.replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>')
}

export function formatAnswer(text) {
  // SECURITY: escape HTML entities first to prevent XSS
  let safe = escapeHtml(text)
  // Convert markdown bold after escaping
  safe = convertBold(safe)

  let header = ''
  let body = safe
  const artHeaderMatch = body.match(/^(Art\.\s*\d+(?:\([^)]+\))*(?:\s*\+\s*Art\.\s*\d+(?:\([^)]+\))*)*)\s*:\s*/)
  if (artHeaderMatch) {
    header = artHeaderMatch[1]
    body = body.slice(artHeaderMatch[0].length)
  }

  const stepRegex = /\((\d+)\)\s/g
  const hasSteps = stepRegex.test(body)
  let html = ''

  if (header) {
    html += `<div class="ans-header"><span class="ans-art">${header}</span></div>`
  }

  if (hasSteps) {
    const firstStep = body.indexOf('(1)')
    let intro = ''
    let stepsText = body
    if (firstStep > 0) {
      intro = body.slice(0, firstStep).trim()
      stepsText = body.slice(firstStep)
    }
    if (intro) {
      html += `<div class="ans-intro">${highlightArticles(intro)}</div>`
    }
    const steps = []
    const parts = stepsText.split(/\((\d+)\)\s/)
    for (let i = 1; i < parts.length; i += 2) {
      const num = parts[i]
      const content = (parts[i + 1] || '').trim().replace(/\.\s*$/, '')
      if (content) steps.push({ num, content: highlightArticles(content) })
    }
    if (steps.length) {
      html += '<div class="ans-steps">'
      for (const s of steps) {
        html += `<div class="ans-step"><span class="step-num">${s.num}</span><span class="step-text">${s.content}</span></div>`
      }
      html += '</div>'
    }
  } else {
    html += `<div class="ans-body">${highlightArticles(body)}</div>`
  }

  return html
}

// Extract article numbers referenced in text
export function extractArticles(text) {
  const matches = [...text.matchAll(/Art\.\s*(\d+)/g)]
  const unique = [...new Set(matches.map(m => m[1]))]
  return unique.filter(n => ART_META[n]).map(n => ({
    num: n,
    url: EURLEX_BASE + (ART_META[n]?.anchor || ''),
    ...ART_META[n],
  }))
}

// Get category-specific resource link
export function getCategoryLink(category, lang) {
  const cat = category.replace(/_impl$/, '')
  const links = CAT_LINKS[lang] || CAT_LINKS.en
  return links[cat] || null
}

function highlightArticles(text) {
  return text.replace(/Art\.\s*(\d+)(\([^)]+\))*/g,
    '<span class="art-ref">Art. $1$2</span>')
}
