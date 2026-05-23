<script setup>
import { ref, onMounted, nextTick, computed } from 'vue'
import ChatMessages from './ChatMessages.vue'
import ChatInput from './ChatInput.vue'
import { bm25Search, isItalian, findFollowUps } from './search.js'
import { formatAnswer, extractArticles, getCategoryLink } from './formatter.js'
import { findTerms, GLOSSARY_SIZE } from './glossary.js'
import { getStrings, getCategoryName } from './i18n.js'

const dataset = ref([])
const messages = ref([])
const input = ref('')
const isLoading = ref(false)
const chatEl = ref(null)
const stats = ref({ entries: 0, queries: 0, hits: 0, avgMs: 0 })
const askedQuestions = ref(new Set())
const totalMs = ref(0)
const ready = ref(false)
const loadProgress = ref(0)
const hasAccepted = ref(false)
const showLeft = ref(true)
const showRight = ref(true)
const focusMode = ref(false)
const sideTab = ref('norm') // 'norm' or 'docs'

// WASM Integration & Wizard State
const wizardState = ref(null)
const wizardProfile = ref({ sector: '', employees: 0, revenue: 0 })
let wasmEngine = null

const assessmentResult = ref(null)
const checklistState = ref({})

const checklistKey = computed(() => {
  if (!wizardProfile.value.sector || !wizardProfile.value.employees) return null
  return `nis2_checklist_${wizardProfile.value.sector}_${wizardProfile.value.employees}_${wizardProfile.value.revenue}`
})

// Watch checklistKey to load from localStorage
import { watch } from 'vue'
watch(checklistKey, (newKey) => {
  if (newKey && typeof window !== 'undefined') {
    const saved = localStorage.getItem(newKey)
    if (saved) {
      try {
        checklistState.value = JSON.parse(saved)
      } catch (e) {
        checklistState.value = {}
      }
    } else {
      checklistState.value = {}
    }
  } else {
    checklistState.value = {}
  }
}, { immediate: true })

// Save checklist state to localStorage
function saveChecklist() {
  const key = checklistKey.value
  if (key && typeof window !== 'undefined') {
    localStorage.setItem(key, JSON.stringify(checklistState.value))
  }
}

function updateObligationStatus(obId, status) {
  checklistState.value[obId] = status
  saveChecklist()
}

const checklistProgress = computed(() => {
  if (!assessmentResult.value || !assessmentResult.value.obligations) return { completed: 0, total: 0, pct: 0 }
  const total = assessmentResult.value.obligations.length
  if (total === 0) return { completed: 0, total: 0, pct: 0 }
  const completed = assessmentResult.value.obligations.filter(ob => checklistState.value[ob.id] === 'Implemented').length
  const pct = Math.round((completed / total) * 100)
  return { completed, total, pct }
})

const lang = ref('en')
onMounted(() => {
  const nav = navigator.language || navigator.userLanguage || 'en'
  if (nav.startsWith('it')) lang.value = 'it'
  if (typeof window !== 'undefined' && window.innerWidth < 960) {
    showLeft.value = false
    showRight.value = false
  }
})

const t = computed(() => getStrings(lang.value))

const visitedCategories = ref(new Set())
const AREA_IDS = [
  'applicability','governance','access_control','encryption','incident_response',
  'business_continuity','supply_chain','vulnerability_mgmt','risk_assessment',
  'network_security','detection','email_security','documentation',
  'remote_work','physical','legal','sanctions','asset_management','development',
]

const coverage = computed(() => {
  const explored = AREA_IDS.filter(a => visitedCategories.value.has(a)).length
  return { explored, total: AREA_IDS.length, pct: Math.round(explored / AREA_IDS.length * 100) }
})

const areaStatus = computed(() =>
  AREA_IDS.map(id => ({
    id, name: getCategoryName(id, lang.value),
    visited: visitedCategories.value.has(id),
  }))
)

const ringR = 28
const ringC = 2 * Math.PI * ringR
const ringOffset = computed(() => ringC - (coverage.value.pct / 100) * ringC)

function filterAsked(items) {
  return items.filter(q => !askedQuestions.value.has(q.toLowerCase()))
}

function dedup(arr) {
  const seen = new Set()
  return arr.filter(q => {
    const k = q.toLowerCase()
    if (seen.has(k)) return false
    seen.add(k)
    return true
  })
}

function exploreSuggestions(uiLang, limit = 4) {
  const explored = visitedCategories.value
  const unexplored = AREA_IDS.filter(a => !explored.has(a))
  if (!unexplored.length) return []
  const candidates = []
  for (const cat of unexplored) {
    const entries = dataset.value.filter(d => d.c.replace(/_impl$/, '') === cat)
    if (entries.length) {
      const entry = entries[0]
      const qText = uiLang === 'it' ? (entry.it?.q || entry.en?.q || '') : (entry.en?.q || entry.it?.q || '')
      candidates.push(qText)
    }
    if (candidates.length >= limit) break
  }
  return filterAsked(candidates)
}

function search(query) {
  const t0 = performance.now()
  
  // Auto-detect query language and sync UI
  if (isItalian(query) && lang.value !== 'it') lang.value = 'it';
  else if (!isItalian(query) && /[a-z]/i.test(query) && lang.value === 'it') lang.value = 'en';
  
  const uiLang = lang.value

  // Map bilingual dataset to active language
  const activeDataset = dataset.value.map(d => ({
    q: d[uiLang]?.q || d.en?.q || '',
    a: d[uiLang]?.a || d.en?.a || '',
    c: d.c,
    s: d.s,
    d: d.d,
    r: d.r,
    id: d.id
  }))

  let results = bm25Search(activeDataset, query, 3)

  const elapsed = performance.now() - t0

  if (results.length > 0 && results[0].score > 0.5) {
    const entry = activeDataset[results[0].idx]
    const cat = entry.c.replace(/_impl$/, '')
    visitedCategories.value = new Set([...visitedCategories.value, cat])
    stats.value.hits++
    const rawFollowUps = findFollowUps(activeDataset, entry.c, uiLang)
    let followUps = filterAsked(rawFollowUps)
    if (followUps.length < 2) {
      followUps = [...followUps, ...exploreSuggestions(uiLang, 4 - followUps.length)]
    }
    return {
      hit: true, answer: entry.a, html: formatAnswer(entry.a),
      category: getCategoryName(entry.c, uiLang), followUps: dedup(followUps), elapsed,
      refs: extractArticles(entry.a),
      catLink: getCategoryLink(entry.c, uiLang),
      severity: entry.s || 'info',
      deadline: entry.d || '',
      glossary: findTerms(entry.a).map(t => ({ term: t.term.toUpperCase(), def: uiLang === 'it' ? t.it : t.en })),
      standards: entry.r || [],
      confidence: Math.min(99, Math.round(results[0].score * 100)),
      source: entry.c.replace(/_impl$/, ''),
    }
  }
  const missStrings = getStrings(uiLang)
  let followUps = filterAsked(missStrings.missSuggestions)
  if (followUps.length < 2) followUps = [...followUps, ...exploreSuggestions(uiLang, 4 - followUps.length)]
  return { hit: false, elapsed, followUps: dedup(followUps) }
}

async function sendMessage(text) {
  const query = (text || input.value).trim()
  if (!query || isLoading.value) return
  input.value = ''
  
  // Handle Wizard State Machine
  if (wizardState.value) {
    messages.value.push({ role: 'user', text: query })
    isLoading.value = true
    await nextTick(); scrollBottom()
    await new Promise(r => setTimeout(r, 400))
    
    if (wizardState.value === 'sector') {
      let s = query.toLowerCase()
      let sectorId = 'other'
      if (s.includes('energ')) sectorId = 'energy'
      else if (s.includes('trasport') || s.includes('transport')) sectorId = 'transport'
      else if (s.includes('banc') || s.includes('bank') || s.includes('finanz') || s.includes('financ')) sectorId = 'banking'
      else if (s.includes('mercato') || s.includes('market') || s.includes('borsa') || s.includes('clearing') || s.includes('trading')) sectorId = 'financial_market_infrastructure'
      else if (s.includes('salut') || s.includes('health') || s.includes('ospedal') || s.includes('sanit') || s.includes('farmac')) sectorId = 'health'
      else if (s.includes('reflu') || s.includes('waste water') || s.includes('fogn') || s.includes('depura')) sectorId = 'waste_water'
      else if (s.includes('potabil') || s.includes('drinking') || s.includes('acqued') || s.includes('acqua') || s.includes('water')) sectorId = 'drinking_water'
      else if (s.includes('datacent') || s.includes('hosting') || s.includes('dns') || s.includes('telecom') || s.includes('connettiv')) sectorId = 'digital_infrastructure'
      else if (s.includes('digital') || s.includes('cloud') || s.includes('ict') || s.includes('it ')) sectorId = 'digital_infrastructure'
      else if (s.includes('msp') || s.includes('mssp') || s.includes('b2b') || s.includes('consulenza it') || s.includes('it consulting') || s.includes('gestiti') || s.includes('managed')) sectorId = 'ict_service_management_b2b'
      else if (s.includes('pubblic') || s.includes('public') || s.includes('pa') || s.includes('comun') || s.includes('provinc') || s.includes('region') || s.includes('minister')) sectorId = 'public_administration'
      else if (s.includes('spazio') || s.includes('space') || s.includes('aerospaz') || s.includes('aerospace') || s.includes('satellit')) sectorId = 'space'
      else if (s.includes('posta') || s.includes('postal') || s.includes('corrier') || s.includes('courier') || s.includes('spedizion') || s.includes('shipping')) sectorId = 'postal_courier'
      else if (s.includes('rifiut') || s.includes('waste') || s.includes('immondiz') || s.includes('spazzatur')) sectorId = 'waste_management'
      else if (s.includes('chimic') || s.includes('chemical')) sectorId = 'chemicals'
      else if (s.includes('aliment') || s.includes('food')) sectorId = 'food'
      else if (s.includes('ricerca') || s.includes('research') || s.includes('universit') || s.includes('laborator')) sectorId = 'research'
      else if (s.includes('motore di ricerca') || s.includes('search engine') || s.includes('marketplace') || s.includes('e-commerce') || s.includes('social') || s.includes('piattaforma digitale') || s.includes('online provider')) sectorId = 'digital_providers'
      else if (s.includes('manifattur') || s.includes('manufactur') || s.includes('produzion') || s.includes('fabbric')) sectorId = 'manufacturing'
      
      wizardProfile.value.sector = sectorId
      wizardState.value = 'employees'
      const txt = lang.value==='it' ? "Ottimo. Quanti **dipendenti** ha la tua azienda?" : "Great. How many **employees** do you have?"
      messages.value.push({ role:'assistant', text: txt, html: formatAnswer(txt), typing: false, displayHtml: formatAnswer(txt) })
    }
    else if (wizardState.value === 'employees') {
      const match = query.match(/\d+/)
      wizardProfile.value.employees = match ? parseInt(match[0], 10) : 0
      wizardState.value = 'revenue'
      const txt = lang.value==='it' ? "E qual è il **fatturato** annuo (in milioni di euro)?" : "And what is the annual **revenue** (in millions of euros)?"
      messages.value.push({ role:'assistant', text: txt, html: formatAnswer(txt), typing: false, displayHtml: formatAnswer(txt) })
    }
    else if (wizardState.value === 'revenue') {
      const normalizedQuery = query.replace(',', '.')
      const match = normalizedQuery.match(/\d+(?:\.\d+)?/)
      wizardProfile.value.revenue = match ? parseFloat(match[0]) : 0.0
      wizardState.value = null // reset wizard
      
      // Run WASM engine
      if (wasmEngine) {
        const payload = JSON.stringify({
          name: "Company",
          sector: wizardProfile.value.sector,
          employees: wizardProfile.value.employees,
          annual_revenue_eur_m: wizardProfile.value.revenue,
          balance_sheet_eur_m: wizardProfile.value.revenue,
          services: [],
          member_states: ["IT"]
        })
        
        try {
          const resStr = wasmEngine.evaluate_profile_json(payload)
          const res = JSON.parse(resStr)
          assessmentResult.value = res
          if (typeof window !== 'undefined') {
            localStorage.setItem('nis2_wizard_profile', JSON.stringify(wizardProfile.value))
            localStorage.setItem('nis2_assessment_result', JSON.stringify(res))
          }
          let ans = ""
          let severity = "info"
          
          if (res.applicable) {
             ans = lang.value === 'it' 
               ? `**Risultato Assessment (Motore WASM):**\nLa tua azienda ricade nella Direttiva NIS2 ed è classificata come **Soggetto ${res.category === 'Essential' ? 'Essenziale' : 'Importante'}**.\n\n` +
                 `- **Sanzione max potenziale:** €${(res.max_sanction_eur / 1000000).toFixed(1)} Milioni\n` +
                 `- **Obblighi assegnati:** ${res.obligations.length} requisiti di sicurezza\n` +
                 `- **Notifica incidenti:** Early warning entro ${res.incident_reporting.early_warning_hours}h.\n\n` +
                 `*Nota: Calcolo effettuato localmente nel browser a latenza zero tramite Rust WebAssembly.*`
               : `**Assessment Result (WASM Engine):**\nYour company falls under the NIS2 Directive and is classified as an **${res.category} Entity**.\n\n` +
                 `- **Max potential fine:** €${(res.max_sanction_eur / 1000000).toFixed(1)} Million\n` +
                 `- **Assigned obligations:** ${res.obligations.length} security requirements\n` +
                 `- **Incident reporting:** Early warning within ${res.incident_reporting.early_warning_hours}h.\n\n` +
                 `*Note: Computed locally in browser at zero latency via Rust WebAssembly.*`
             severity = res.category === 'Essential' ? 'danger' : 'warning'
          } else {
             ans = lang.value === 'it'
               ? `**Risultato Assessment (Motore WASM):**\nSulla base dei dati forniti, la tua azienda **NON** sembra rientrare direttamente negli obblighi NIS2 (Categoria: ${res.category}).\n\nTuttavia, fai attenzione ai requisiti della supply chain se fornisci servizi IT a soggetti essenziali o importanti.\n\n*Nota: Calcolo effettuato localmente nel browser tramite Rust WebAssembly.*`
               : `**Assessment Result (WASM Engine):**\nBased on the data, your company **does NOT** appear to fall directly under NIS2 obligations (Category: ${res.category}).\n\nHowever, be mindful of supply chain requirements if you provide IT services to essential or important entities.\n\n*Note: Computed locally in browser via Rust WebAssembly.*`
             severity = 'info'
          }
          
          messages.value.push({ role:'assistant', text: ans, html: formatAnswer(ans), category: 'Applicability', followUps: [], severity, typing: false, displayHtml: formatAnswer(ans) })
        } catch (err) {
          console.error("WASM Evaluation error:", err)
          const errTxt = lang.value === 'it' ? "Errore durante l'elaborazione WASM." : "Error during WASM evaluation."
          messages.value.push({ role:'assistant', text: errTxt, html: formatAnswer(errTxt), category: 'Error', followUps: [], typing: false, displayHtml: formatAnswer(errTxt) })
        }
      } else {
         const errTxt = lang.value === 'it' ? "Motore WASM non caricato. Impossibile eseguire l'assessment." : "WASM engine not loaded. Cannot perform assessment."
         messages.value.push({ role:'assistant', text: errTxt, html: formatAnswer(errTxt), category: 'Error', followUps: [], typing: false, displayHtml: formatAnswer(errTxt) })
      }
    }
    
    isLoading.value = false
    await nextTick(); scrollBottom()
    return
  }

  // Intercept Intent to Start Wizard
  if (query.match(/(valuta|soggetto|assess|am i subject|rientro)/i) && query.match(/(azienda|nis2|company|compliance)/i)) {
    messages.value.push({ role: 'user', text: query })
    isLoading.value = true
    await nextTick(); scrollBottom()
    await new Promise(r => setTimeout(r, 400))
    
    wizardState.value = 'sector'
    const txt = lang.value === 'it' 
      ? "Certo! Attiviamo l'**Assessment Guidato**. Per iniziare, in quale **settore** opera la tua azienda?" 
      : "Sure! Let's start the **Guided Assessment**. To begin, what **sector** does your company operate in?"
    const options = lang.value === 'it'
      ? ['Energia', 'Trasporti', 'Banche', 'Sanità', 'Servizi Digitali', 'Pubblica Amministrazione', 'Alimentare', 'Manifattura', 'Chimica', 'Spazio', 'Servizi Postali', 'Gestione Rifiuti', 'Ricerca']
      : ['Energy', 'Transport', 'Banking', 'Health', 'Digital Services', 'Public Administration', 'Food', 'Manufacturing', 'Chemicals', 'Space', 'Postal Services', 'Waste Management', 'Research']
    messages.value.push({ role:'assistant', text: txt, html: formatAnswer(txt), typing: false, displayHtml: formatAnswer(txt), options })
    
    isLoading.value = false
    await nextTick(); scrollBottom()
    return
  }

  askedQuestions.value = new Set([...askedQuestions.value, query.toLowerCase()])
  messages.value.push({ role: 'user', text: query })
  isLoading.value = true
  stats.value.queries++
  await nextTick(); scrollBottom()
  await new Promise(r => setTimeout(r, 50))

  const result = search(query)
  totalMs.value += result.elapsed
  stats.value.avgMs = totalMs.value / stats.value.queries
  const missText = t.value.miss

  if (result.hit) {
    const msg = { role:'assistant', text:result.answer, html:result.html, category:result.category, followUps:result.followUps, refs:result.refs, catLink:result.catLink, severity:result.severity, deadline:result.deadline, standards:result.standards, glossary:result.glossary, confidence:result.confidence, source:result.source, typing:true, displayHtml:'' }
    messages.value.push(msg)
    const words = result.answer.split(' ')
    for (let i = 0; i < words.length; i++) {
      msg.displayHtml = formatAnswer(words.slice(0, i + 1).join(' '))
      if (i % 3 === 0) { await new Promise(r => setTimeout(r, 8)); scrollBottom() }
    }
    msg.typing = false; msg.displayHtml = result.html
  } else {
    messages.value.push({ role:'assistant', text: missText, html: missText, category:'miss', followUps:result.followUps, typing:false, displayHtml:'' })
  }
  isLoading.value = false
  await nextTick(); scrollBottom()
}

function scrollBottom() {
  const el = chatEl.value?.$el || chatEl.value
  if (el) {
    el.scrollTop = el.scrollHeight
    setTimeout(() => {
      el.scrollTop = el.scrollHeight
    }, 50)
  }
}

// Keyboard shortcuts
onMounted(() => {
  document.addEventListener('keydown', (e) => {
    const active = document.activeElement?.tagName
    if (active === 'INPUT' || active === 'TEXTAREA') {
      if (e.key === 'Escape') { input.value = ''; document.activeElement.blur() }
      return
    }
    if (e.key === '/' || e.key === 'k' && e.metaKey) {
      e.preventDefault()
      document.querySelector('.in-wrap input')?.focus()
    }
    if (e.key === 'l' || e.key === 'L') {
      lang.value = lang.value === 'it' ? 'en' : 'it'
    }
    if (e.key === 'f' || e.key === 'F') {
      focusMode.value = !focusMode.value
    }
  })
})

onMounted(async () => {
  try {
    loadProgress.value = 20
    const base = import.meta.env.BASE_URL || '/'
    
    // Load WASM dynamically
    try {
      const mod = await import(/* @vite-ignore */ `${base}wasm/nis2_rules.js`)
      await mod.default(`${base}wasm/nis2_rules_bg.wasm`)
      wasmEngine = mod
      console.log('Rust WASM engine loaded successfully')
    } catch (we) {
      console.error('WASM load failed:', we)
    }
    
    const res = await fetch(`${base}dataset.json`)
    loadProgress.value = 60
    dataset.value = await res.json()
    loadProgress.value = 90
    stats.value.entries = dataset.value.length
    await new Promise(r => setTimeout(r, 400))
    loadProgress.value = 100
    await new Promise(r => setTimeout(r, 300))
    
    // Restore profile and assessment result from localStorage
    if (typeof window !== 'undefined') {
      const savedProfile = localStorage.getItem('nis2_wizard_profile')
      const savedResult = localStorage.getItem('nis2_assessment_result')
      if (savedProfile) {
        try {
          wizardProfile.value = JSON.parse(savedProfile)
        } catch (e) {}
      }
      if (savedResult) {
        try {
          assessmentResult.value = JSON.parse(savedResult)
        } catch (e) {}
      }
    }
    ready.value = true
  } catch (e) { console.error('Dataset load failed:', e); ready.value = true }
})

// Live Compliance Document — built from session answers
const complianceDoc = computed(() => {
  const sections = []
  const seen = new Set()
  for (const m of messages.value) {
    if (m.role !== 'assistant' || m.category === 'miss' || !m.text) continue
    const cat = m.source || m.category || 'general'
    if (seen.has(cat)) continue
    seen.add(cat)
    sections.push({
      title: m.category,
      summary: m.text.length > 200 ? m.text.substring(0, 200) + '...' : m.text,
      severity: m.severity || 'info',
      confidence: m.confidence || 0,
      refs: (m.refs || []).map(r => 'Art. ' + r.num),
    })
  }
  return sections
})

function exportReport() {
  const isIt = lang.value === 'it'
  const title = isIt ? 'Report Conformità NIS2' : 'NIS2 Compliance Report'
  const date = new Date().toLocaleDateString(lang.value === 'it' ? 'it-IT' : 'en-GB', { year:'numeric', month:'long', day:'numeric' })
  let md = `# ${title}\n> ${date}\n\n`
  md += `---\n\n`
  md += isIt ? `## Riepilogo Sessione\n` : `## Session Summary\n`
  md += isIt
    ? `- **Domande analizzate:** ${stats.value.queries}\n- **Risposte trovate:** ${stats.value.hits}\n- **Copertura aree:** ${coverage.value.explored}/${coverage.value.total} (${coverage.value.pct}%)\n\n`
    : `- **Queries analyzed:** ${stats.value.queries}\n- **Hits:** ${stats.value.hits}\n- **Area coverage:** ${coverage.value.explored}/${coverage.value.total} (${coverage.value.pct}%)\n\n`
  md += `---\n\n`
  for (const s of complianceDoc.value) {
    md += `### ${s.title}\n`
    md += `> ${(isIt ? 'Confidenza' : 'Confidence')}: **${s.confidence}%** | ${s.refs.join(', ')}\n\n`
    md += `${s.summary}\n\n`
  }
  md += `---\n\n`
  md += isIt
    ? `**⚠️ AVVISO LEGALE:** Questo report è generato automaticamente da un motore di analisi software. Non costituisce consulenza legale. Per una valutazione vincolante, consultare un avvocato qualificato. Rif: D.Lgs. 138/2024 / Direttiva (UE) 2022/2555.`
    : `**⚠️ LEGAL NOTICE:** This report is automatically generated by a software analysis engine. It does not constitute legal advice. For a binding assessment, consult a qualified attorney. Ref: Directive (EU) 2022/2555.`
  const blob = new Blob([md], { type: 'text/markdown' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url; a.download = `nis2-report-${Date.now()}.md`
  a.click(); URL.revokeObjectURL(url)
}

// Cited articles — extracted from all session answers
const citedArticles = computed(() => {
  const seen = new Map()
  for (const m of messages.value) {
    if (m.role !== 'assistant' || !m.refs) continue
    for (const r of m.refs) {
      if (!seen.has(r.num)) {
        seen.set(r.num, { num: r.num, url: r.url, label: `Art. ${r.num} — Direttiva 2022/2555` })
      }
    }
  }
  return [...seen.values()].sort((a, b) => a.num - b.num)
})
</script>

<template>
<!-- Loader -->
<div v-if="!ready" class="loader">
  <div class="loader-inner">
    <svg class="loader-shield" width="36" height="36" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
    <div class="loader-text">{{ t.loading }}</div>
    <div class="loader-bar"><div class="loader-fill" :style="{width: loadProgress+'%'}"></div></div>
  </div>
</div>

<!-- Consent interstitial -->
<div v-else-if="!hasAccepted" class="consent">
  <div class="consent-card">
    <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
    <h2 class="consent-title">{{ lang === 'it' ? 'Avviso Legale' : 'Legal Notice' }}</h2>
    <p class="consent-text">
      {{ lang === 'it'
        ? 'Questo strumento fornisce classificazioni automatizzate basate su un sottoinsieme della Direttiva NIS2 (UE 2022/2555). Non costituisce consulenza legale. Consultare un avvocato qualificato per determinare gli obblighi applicabili.'
        : 'This tool provides automated classifications based on a subset of the NIS2 Directive (EU 2022/2555). It does not constitute legal advice. Consult a qualified attorney to determine applicable obligations.' }}
    </p>
    <div class="consent-links">
      <a href="/nis2-model/legal/terms" target="_blank">{{ lang === 'it' ? 'Termini di utilizzo' : 'Terms of Use' }}</a>
      <span>·</span>
      <a href="/nis2-model/legal/privacy" target="_blank">{{ lang === 'it' ? 'Privacy Policy' : 'Privacy Policy' }}</a>
    </div>
    <button class="consent-btn" @click="hasAccepted = true">
      {{ lang === 'it' ? 'Ho compreso — Procedi' : 'I Understand — Proceed' }}
    </button>
  </div>
</div>

<!-- App -->
<div v-else :class="['app', { focus: focusMode }]">
  <header class="hd">
    <div class="hd-left">
      <button v-if="!focusMode" class="hd-toggle" @click="showLeft = !showLeft" :title="t.coverageLabel">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="2"/><line x1="9" y1="3" x2="9" y2="21"/></svg>
      </button>
      <div class="hd-cov">{{ coverage.explored }}/{{ coverage.total }}</div>
    </div>
    <div class="hd-center">
      <svg class="hd-icon" width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
      <div class="hd-title">{{ t.title }}</div>
    </div>
    <div class="hd-right">
      <button v-if="complianceDoc.length" class="export-btn" @click="exportReport" :title="t.tooltips?.exportReport">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
        <span>Report</span>
      </button>
      <button class="focus-btn" @click="focusMode = !focusMode" :class="{ active: focusMode }" :title="t.tooltips?.focusMode">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"/></svg>
      </button>
      <button class="lang-btn" @click="lang = lang === 'it' ? 'en' : 'it'" :title="t.tooltips?.langToggle">{{ lang.toUpperCase() }}</button>
      <button v-if="!focusMode" class="hd-toggle" @click="showRight = !showRight" :title="t.tooltips?.rightPanel">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="2"/><line x1="15" y1="3" x2="15" y2="21"/></svg>
      </button>
    </div>
  </header>

  <div class="body">
    <!-- Left sidebar -->
    <aside v-if="showLeft" class="side side-l">
      <div class="ring-row" :title="t.tooltips?.coverage">
        <svg class="ring" viewBox="0 0 64 64" width="56" height="56">
          <circle cx="32" cy="32" :r="ringR" fill="none" stroke="var(--vp-c-divider)" stroke-width="3"/>
          <circle cx="32" cy="32" :r="ringR" fill="none" stroke="var(--vp-c-brand-1)" stroke-width="3"
            stroke-linecap="round" :stroke-dasharray="ringC" :stroke-dashoffset="ringOffset"
            transform="rotate(-90 32 32)" class="ring-arc"/>
        </svg>
        <div class="ring-info">
          <div class="ring-pct">{{ coverage.pct }}%</div>
          <div class="ring-cap">{{ coverage.explored }}/{{ coverage.total }} {{ t.coverageLabel.toLowerCase() }}</div>
        </div>
      </div>

      <div class="areas">
        <div v-for="a in areaStatus" :key="a.id" :class="['area', { on: a.visited }]" :title="a.visited ? t.tooltips?.areaVisited : t.tooltips?.areaNotVisited">
          <span class="area-dot"></span>
          <span>{{ a.name }}</span>
        </div>
      </div>

      <div class="side-footer">
        <a href="https://github.com/fabriziosalmi/nis2-public" target="_blank" class="side-link" :title="t.tooltips?.linkProject">
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z"/><path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z"/></svg>
          {{ t.linkProject }}
        </a>
        <a href="https://github.com/fabriziosalmi/nis2-model" target="_blank" class="side-link" :title="t.tooltips?.linkEngine">
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"/></svg>
          {{ t.linkEngine }}
        </a>
      </div>
    </aside>

    <!-- Center — shared axis container -->
    <div class="center">
      <ChatMessages ref="chatEl" :messages="messages" :isLoading="isLoading" :lang="lang" @followUp="sendMessage">
        <template #welcome>
          <div v-if="messages.length === 0" class="welcome">
            <svg class="w-icon" width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
            <h2 class="w-title">{{ t.welcome }}</h2>
            <div class="w-badge">{{ t.welcomeBadge }} · {{ stats.entries }} entries</div>
            <p class="w-sub">{{ stats.entries }} {{ t.welcomeSub }}</p>
            <div class="w-grid">
              <button v-for="s in t.suggestions" :key="s" @click="sendMessage(s)" class="w-card">
                <span>{{ s }}</span>
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="9 18 15 12 9 6"/></svg>
              </button>
            </div>
          </div>
        </template>
      </ChatMessages>
      <ChatInput v-model:input="input" :disabled="isLoading || !stats.entries" :placeholder="t.placeholder" :disclaimer="t.disclaimer" :sendTooltip="t.tooltips?.sendBtn" @send="sendMessage()" />
    </div>

    <!-- Right sidebar: Command Dashboard -->
    <aside v-if="showRight && !focusMode" class="side side-r">
      <!-- GRC Compliance Checklist -->
      <div v-if="assessmentResult && assessmentResult.applicable" class="checklist-section">
        <div class="side-label">{{ lang === 'it' ? 'Checklist Conformità GRC' : 'GRC Compliance Checklist' }}</div>
        
        <!-- Live progress indicator -->
        <div class="checklist-progress">
          <div class="progress-bar-container">
            <div class="progress-bar-fill" :style="{ width: checklistProgress.pct + '%' }"></div>
          </div>
          <div class="progress-text">
            {{ checklistProgress.completed }}/{{ checklistProgress.total }} {{ lang === 'it' ? 'completati' : 'completed' }} ({{ checklistProgress.pct }}%)
          </div>
        </div>

        <!-- Obligations list -->
        <div class="checklist-items">
          <div v-for="ob in assessmentResult.obligations" :key="ob.id" class="checklist-item">
            <div class="item-header">
              <span class="item-ref">{{ ob.article_ref }}</span>
              <select :value="checklistState[ob.id] || 'Pending'" @change="updateObligationStatus(ob.id, $event.target.value)" class="item-status-select">
                <option value="Pending">{{ lang === 'it' ? 'Pendente' : 'Pending' }}</option>
                <option value="In Progress">{{ lang === 'it' ? 'In Corso' : 'In Progress' }}</option>
                <option value="Implemented">{{ lang === 'it' ? 'Implementato' : 'Implemented' }}</option>
              </select>
            </div>
            <div class="item-desc" :title="ob.description">{{ ob.description }}</div>
          </div>
        </div>
      </div>

      <!-- Normativa identificata -->
      <div class="side-label">{{ lang === 'it' ? 'Norme Identificate' : 'Identified Legislation' }}</div>
      <div class="side-panel">
        <template v-if="citedArticles.length">
          <a v-for="art in citedArticles" :key="art.num" :href="art.url" target="_blank" class="norm-card" :title="t.tooltips?.normCard + ' — Art. ' + art.num">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z"/><path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z"/></svg>
            <div class="norm-info">
              <span class="norm-title">Art. {{ art.num }}</span>
              <span class="norm-sub">Direttiva (UE) 2022/2555</span>
            </div>
            <svg class="norm-ext" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/><polyline points="15 3 21 3 21 9"/><line x1="10" y1="14" x2="21" y2="3"/></svg>
          </a>
        </template>
        <div v-else class="empty-session">
          {{ lang === 'it' ? 'Le norme appariranno qui durante la conversazione.' : 'Legislation will appear here as you converse.' }}
        </div>
      </div>

      <!-- Artefatti -->
      <div v-if="complianceDoc.length" class="side-label" style="margin-top:16px">{{ lang === 'it' ? 'Azioni' : 'Actions' }}</div>
      <button v-if="complianceDoc.length" class="artifact-card" @click="exportReport" :title="t.tooltips?.artifactCard">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
        <div class="norm-info">
          <span class="norm-title">{{ lang === 'it' ? 'Scarica Report' : 'Download Report' }}</span>
          <span class="norm-sub">{{ complianceDoc.length }} {{ lang === 'it' ? 'sezioni' : 'sections' }} · Markdown</span>
        </div>
      </button>
    </aside>
  </div>
</div>
</template>

<style scoped>
*{box-sizing:border-box}
.app,.loader{
  font-family:-apple-system,BlinkMacSystemFont,'SF Pro Text','SF Pro Display','Inter',system-ui,sans-serif;
  -webkit-font-smoothing:antialiased;
  -moz-osx-font-smoothing:grayscale;
  color:var(--vp-c-text-1);
}

/* Loader */
.loader{position:fixed;inset:0;display:flex;align-items:center;justify-content:center;background:var(--vp-c-bg);z-index:999}
.loader-inner{text-align:center;animation:fadeUp .5s ease}
.loader-shield{color:var(--vp-c-text-3);margin-bottom:14px;animation:pulse 2s infinite}
.loader-text{font-size:13px;color:var(--vp-c-text-3);margin-bottom:14px;letter-spacing:.01em}
.loader-bar{width:160px;height:2px;background:var(--vp-c-divider);border-radius:1px;overflow:hidden;margin:0 auto}
.loader-fill{height:100%;background:var(--vp-c-brand-1);border-radius:1px;transition:width .3s}

/* Consent interstitial */
.consent{position:fixed;inset:0;display:flex;align-items:center;justify-content:center;background:var(--vp-c-bg);z-index:999}
.consent-card{max-width:480px;padding:40px 32px;text-align:center;animation:fadeUp .5s ease}
.consent-card svg{color:var(--vp-c-brand-1);opacity:.4;margin-bottom:16px}
.consent-title{font-size:20px;font-weight:700;margin:0 0 12px;letter-spacing:-.02em}
.consent-text{font-size:14px;color:var(--vp-c-text-2);line-height:1.65;margin:0 0 20px}
.consent-links{display:flex;align-items:center;justify-content:center;gap:8px;margin-bottom:24px;font-size:13px}
.consent-links a{color:var(--vp-c-brand-1);text-decoration:none}
.consent-links a:hover{text-decoration:underline}
.consent-links span{color:var(--vp-c-text-3)}
.consent-btn{
  padding:10px 28px;border:none;border-radius:8px;
  background:#2563eb;color:#fff;font-size:14px;font-weight:600;
  cursor:pointer;transition:all .15s;
}
.consent-btn:hover{background:#1d4ed8;transform:scale(1.02)}
.consent-btn:active{transform:scale(.98)}

/* Header — 3 column centered */
.hd{
  display:grid;grid-template-columns:1fr auto 1fr;align-items:center;
  padding:12px 16px;border-bottom:1px solid var(--vp-c-divider);flex-shrink:0;
  background:var(--vp-c-bg);
  box-shadow:0 1px 2px rgba(0,0,0,.05);
  position:relative;z-index:10;
}
.hd-left{display:flex;align-items:center;gap:8px}
.hd-center{display:flex;align-items:center;justify-content:center;gap:7px}
.hd-right{display:flex;align-items:center;gap:8px;justify-content:flex-end}
.hd-icon{color:var(--vp-c-brand-1);flex-shrink:0}
.hd-title{font-size:15px;font-weight:600;letter-spacing:-.02em}
.hd-cov{font-size:12.5px;color:var(--vp-c-text-3);font-variant-numeric:tabular-nums;font-weight:500}
.hd-toggle{
  padding:5px;border:none;background:none;color:var(--vp-c-text-3);
  cursor:pointer;border-radius:6px;display:flex;align-items:center;transition:all .15s;
}
.hd-toggle:hover{color:var(--vp-c-text-1);background:var(--vp-c-bg-soft)}
.lang-btn{
  font-size:11.5px;font-weight:700;padding:3px 10px;
  border:1px solid var(--vp-c-divider);border-radius:5px;
  background:none;color:var(--vp-c-text-2);cursor:pointer;
  letter-spacing:.04em;transition:all .15s;
}
.lang-btn:hover{border-color:var(--vp-c-brand-1);color:var(--vp-c-brand-1)}

/* Layout */
.app{
  display:flex;flex-direction:column;
  height:100vh;
  height:100dvh;
  max-height:100vh;
  max-height:100dvh;
  overflow:hidden;
  background:var(--vp-c-bg);
}
.body{display:flex;flex:1;overflow:hidden}
.center{flex:1;display:flex;flex-direction:column;overflow:hidden;min-width:0}

/* Sidebars */
.side{
  width:220px;flex-shrink:0;padding:16px 12px;overflow-y:auto;overflow-x:hidden;
  display:flex;flex-direction:column;background:var(--vp-c-bg-soft);
}
.side::-webkit-scrollbar{width:6px}
.side::-webkit-scrollbar-track{background:transparent}
.side::-webkit-scrollbar-thumb{background:var(--vp-c-divider);border-radius:8px}
.side::-webkit-scrollbar-thumb:hover{background:var(--vp-c-text-3)}
.side-l{border-right:1px solid var(--vp-c-divider);box-shadow:2px 0 4px rgba(0,0,0,.03)}
.side-r{border-left:1px solid var(--vp-c-divider);box-shadow:-2px 0 4px rgba(0,0,0,.03)}
.side-label{
  font-size:10px;font-weight:700;text-transform:uppercase;
  letter-spacing:.12em;color:var(--vp-c-text-2);margin-bottom:8px;opacity:.7;
}

/* Coverage ring */
.ring-row{display:flex;align-items:center;gap:10px;margin-bottom:12px}
.ring{filter:drop-shadow(0 0 3px rgba(59,130,246,.15))}
.ring-arc{transition:stroke-dashoffset .6s ease}
.ring-info{display:flex;flex-direction:column}
.ring-pct{font-size:23px;font-weight:700;font-variant-numeric:tabular-nums;letter-spacing:-.03em}
.ring-cap{font-size:11.5px;color:var(--vp-c-text-3);letter-spacing:.01em}

/* Areas */
.areas{display:flex;flex-direction:column;gap:0;flex:1;overflow-y:auto;min-height:0}
.area{
  display:flex;align-items:center;gap:8px;padding:6px 8px;
  font-size:13px;color:var(--vp-c-text-3);transition:all .15s ease-out;
  border-radius:6px;cursor:default;
}
.area:hover{background:var(--vp-c-bg-alt);color:var(--vp-c-text-2)}
.area.on{color:var(--vp-c-text-1);font-weight:500}
.area-dot{width:6px;height:6px;border-radius:50%;background:var(--vp-c-divider);flex-shrink:0;transition:all .3s}
.area.on .area-dot{background:var(--vp-c-brand-1);box-shadow:0 0 6px rgba(59,130,246,.4)}

/* Footer links */
.side-footer{padding-top:10px;border-top:1px solid var(--vp-c-divider);display:flex;flex-direction:column;gap:5px;margin-top:6px}
.side-link{font-size:12.5px;color:var(--vp-c-text-3);text-decoration:none;display:flex;align-items:center;gap:6px;transition:color .12s;padding:2px 0}
.side-link:hover{color:var(--vp-c-brand-1)}

/* Focus mode button */
.focus-btn{
  padding:5px;border:none;background:none;color:var(--vp-c-text-3);
  cursor:pointer;border-radius:6px;display:flex;align-items:center;transition:all .15s;
}
.focus-btn:hover{color:var(--vp-c-text-1);background:var(--vp-c-bg-soft)}
.focus-btn.active{color:var(--vp-c-brand-1);background:rgba(59,130,246,.08)}

/* Export button */
.export-btn{
  display:flex;align-items:center;gap:5px;padding:4px 10px;
  border:1px solid var(--vp-c-divider);border-radius:6px;
  background:none;color:var(--vp-c-text-2);cursor:pointer;
  font-size:12.5px;font-weight:600;letter-spacing:.01em;transition:all .15s;
}
.export-btn:hover{border-color:var(--vp-c-brand-1);color:var(--vp-c-brand-1);background:rgba(59,130,246,.04)}
.export-btn svg{flex-shrink:0}

/* ── Status Badge ── */
.status-badge{
  display:flex;align-items:center;gap:8px;
  padding:8px 12px;border-radius:8px;
  font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.08em;
  margin-bottom:12px;
}
.status-dot{width:8px;height:8px;border-radius:50%;flex-shrink:0}
.status-none{background:rgba(100,116,139,.1);color:var(--vp-c-text-3)}
.status-none .status-dot{background:var(--vp-c-text-3)}
.status-eval{background:rgba(234,179,8,.08);color:#ca8a04}
.status-eval .status-dot{background:#eab308;box-shadow:0 0 6px rgba(234,179,8,.4)}
.status-ok{background:rgba(16,185,129,.08);color:#10b981}
.status-ok .status-dot{background:#10b981;box-shadow:0 0 6px rgba(16,185,129,.4)}

/* ── Side Panel (content area) ── */
.side-panel{display:flex;flex-direction:column;gap:4px;flex:1;overflow-y:auto;min-height:0}

/* ── Normativa Cards ── */
.norm-card{
  display:flex;align-items:center;gap:8px;
  padding:8px 10px;border-radius:8px;
  text-decoration:none;color:var(--vp-c-text-2);
  transition:all .15s ease-out;cursor:pointer;
}
.norm-card:hover{background:var(--vp-c-bg-alt);color:var(--vp-c-text-1)}
.norm-card svg{flex-shrink:0;color:var(--vp-c-brand-1);opacity:.6}
.norm-card:hover svg{opacity:.9}
.norm-info{display:flex;flex-direction:column;flex:1;min-width:0}
.norm-title{font-size:12px;font-weight:600;color:var(--vp-c-text-1);line-height:1.3}
.norm-sub{font-size:10px;color:var(--vp-c-text-3);line-height:1.3;margin-top:1px}
.norm-ext{opacity:0;transition:opacity .15s ease-out}
.norm-card:hover .norm-ext{opacity:.4}

/* ── Artifact Card ── */
.artifact-card{
  display:flex;align-items:center;gap:8px;
  padding:10px 10px;border-radius:8px;border:none;
  background:rgba(16,185,129,.06);color:var(--vp-c-text-2);
  cursor:pointer;transition:all .15s ease-out;text-align:left;
  margin-bottom:8px;
}
.artifact-card:hover{background:rgba(16,185,129,.1);color:var(--vp-c-text-1)}
.artifact-card svg{flex-shrink:0;color:#10b981;opacity:.7}
.artifact-card:hover svg{opacity:1}
.artifact-card .norm-ext{opacity:0}
.artifact-card:hover .norm-ext{opacity:.4}

/* ── Doc Section (compact) ── */
.doc-section{
  padding:8px 0;border-bottom:1px solid var(--vp-c-divider);
  animation:fadeUp .3s ease;
}
.doc-section:last-child{border-bottom:none}
.doc-head{display:flex;align-items:center;gap:6px;margin-bottom:2px}
.doc-title{font-size:11px;font-weight:600;color:var(--vp-c-text-1);flex:1}
.doc-conf{
  font-size:9px;font-weight:700;padding:2px 6px;border-radius:9999px;
  background:rgba(16,185,129,.1);color:#10b981;
  font-variant-numeric:tabular-nums;
}
.doc-refs{display:flex;flex-wrap:wrap;gap:3px}
.doc-ref{font-size:9px;padding:2px 6px;border-radius:9999px;background:var(--vp-c-bg-alt);color:var(--vp-c-text-3)}
.sev-dot-sm{width:5px;height:5px;border-radius:50%;flex-shrink:0}
.sev-dot-sm.critical{background:#ef4444}
.sev-dot-sm.high{background:#f59e0b}
.sev-dot-sm.medium{background:#8b5cf6}
.sev-dot-sm.info{background:var(--vp-c-brand-1)}

.empty-session{
  font-size:12px;color:var(--vp-c-text-3);opacity:.6;
  padding:16px 4px;line-height:1.5;font-style:italic;text-align:center;
}

/* Focus mode */
.app.focus .side{display:none!important}
.app.focus .center{max-width:100%}

/* Welcome */
.welcome{display:flex;flex-direction:column;align-items:center;justify-content:center;flex:1;text-align:center;padding:40px 24px;animation:fadeUp .5s ease}
.w-icon{color:var(--vp-c-brand-1);opacity:.15;margin-bottom:10px}
.w-title{font-size:21px;font-weight:600;margin:0;letter-spacing:-.03em;color:var(--vp-c-text-1)}
.w-badge{
  display:inline-flex;align-items:center;gap:4px;
  margin:8px auto 0;padding:3px 11px;border-radius:12px;
  font-size:11.5px;font-weight:600;letter-spacing:.02em;
  background:rgba(59,130,246,.08);color:var(--vp-c-brand-1);
}
.w-sub{color:var(--vp-c-text-3);font-size:14px;margin:6px 0 0;opacity:.6}
.w-grid{display:grid;grid-template-columns:1fr 1fr;gap:6px;max-width:520px;margin-top:24px;width:100%}
.w-card{
  display:flex;align-items:center;justify-content:space-between;
  text-align:left;padding:11px 14px;gap:8px;
  border:1px solid var(--vp-c-divider);border-radius:10px;
  background:var(--vp-c-bg-soft);color:var(--vp-c-text-2);
  font-size:14px;cursor:pointer;transition:all .15s;line-height:1.4;
}
.w-card:hover{border-color:var(--vp-c-brand-1);color:var(--vp-c-text-1);background:rgba(59,130,246,.03)}
.w-card svg{flex-shrink:0;opacity:.2;transition:opacity .15s}
.w-card:hover svg{opacity:.5}

@keyframes fadeUp{from{opacity:0;transform:translateY(4px)}to{opacity:1;transform:translateY(0)}}
@keyframes fadeIn{from{opacity:0}to{opacity:1}}
@keyframes pulse{0%,100%{opacity:.4}50%{opacity:.7}}

@media(max-width:960px){
  .body {
    position: relative;
  }
  .side {
    position: absolute;
    top: 0;
    bottom: 0;
    z-index: 100;
    width: 240px;
    box-shadow: 0 4px 20px rgba(0,0,0,0.15);
    background: var(--vp-c-bg);
  }
  .side-l {
    left: 0;
    border-right: 1px solid var(--vp-c-divider);
  }
  .side-r {
    right: 0;
    border-left: 1px solid var(--vp-c-divider);
  }
  .focus-btn {
    display: none;
  }
}
@media(max-width:640px){.w-grid{grid-template-columns:1fr}.hd-sub{display:none}}

.checklist-section {
  margin-bottom: 20px;
  background: var(--vp-c-bg-alt);
  padding: 12px;
  border-radius: 8px;
  border: 1px solid var(--vp-c-divider);
}
.checklist-progress {
  margin-bottom: 12px;
}
.progress-bar-container {
  width: 100%;
  height: 6px;
  background: var(--vp-c-divider);
  border-radius: 3px;
  overflow: hidden;
  margin-bottom: 4px;
}
.progress-bar-fill {
  height: 100%;
  background: var(--vp-c-brand-1);
  border-radius: 3px;
  transition: width 0.3s ease;
}
.progress-text {
  font-size: 11px;
  color: var(--vp-c-text-3);
  text-align: right;
  font-weight: 500;
}
.checklist-items {
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-height: 250px;
  overflow-y: auto;
  padding-right: 4px;
}
.checklist-items::-webkit-scrollbar {
  width: 4px;
}
.checklist-items::-webkit-scrollbar-thumb {
  background: var(--vp-c-divider);
  border-radius: 4px;
}
.checklist-item {
  background: var(--vp-c-bg);
  border: 1px solid var(--vp-c-divider);
  border-radius: 6px;
  padding: 8px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.item-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.item-ref {
  font-size: 11px;
  font-weight: 700;
  color: var(--vp-c-brand-1);
}
.item-status-select {
  font-size: 10px;
  padding: 2px 4px;
  border-radius: 4px;
  border: 1px solid var(--vp-c-divider);
  background: var(--vp-c-bg-alt);
  color: var(--vp-c-text-1);
  cursor: pointer;
  outline: none;
}
.item-status-select:focus {
  border-color: var(--vp-c-brand-1);
}
.item-desc {
  font-size: 10.5px;
  color: var(--vp-c-text-2);
  line-height: 1.3;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* No separate light-mode overrides needed — all colors use
   VitePress CSS custom properties that auto-adapt to theme. */
</style>
