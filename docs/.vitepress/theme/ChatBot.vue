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

const lang = ref('en')
onMounted(() => {
  const nav = navigator.language || navigator.userLanguage || 'en'
  if (nav.startsWith('it')) lang.value = 'it'
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
    const entries = dataset.value.filter(d => {
      const dc = d.c.replace(/_impl$/, '')
      return dc === cat && (uiLang === 'it' ? isItalian(d.q) : !isItalian(d.q))
    })
    if (entries.length) candidates.push(entries[0].q)
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

  // --- REAL-TIME TOOLING: Dynamic Applicability Assessment ---
  const employeesMatch = query.match(/(\d+)\s*(dipendent[ei]|employees?|persone|lavoratori)/i);
  const revenueMatch = query.match(/(\d+)\s*(milion[ei]|million|mln)/i);
  
  if (employeesMatch || revenueMatch) {
    const emp = employeesMatch ? parseInt(employeesMatch[1], 10) : 0;
    const rev = revenueMatch ? parseInt(revenueMatch[1], 10) : 0;
    
    let answer = '';
    let category = 'applicability';
    let severity = 'info';
    
    const sectorNote = uiLang === 'it'
      ? '\n\n⚠️ **Nota:** questa classificazione preliminare dipende dal settore di appartenenza (Allegato I/II), non indicato nella domanda. Verificare con un consulente qualificato.'
      : '\n\n⚠️ **Note:** this preliminary classification depends on your sector (Annex I/II), not specified in your question. Verify with a qualified advisor.';
    if (emp >= 250 || rev >= 50) {
      answer = uiLang === 'it' 
        ? `Classificazione automatizzata: con **${emp || '>250'} dipendenti** e/o **${rev || '>50'} milioni di fatturato**, il motore classifica l'azienda come grande impresa, potenzialmente rientrante nella **NIS2 come Soggetto Essenziale** (se il settore rientra nell'Allegato I). Art. 21 (Gestione Rischi) e Art. 23 (Notifiche Incidenti in 24h). Sanzioni massime: 10M EUR o 2% del fatturato mondiale.${sectorNote}`
        : `Automated classification: with **${emp || '>250'} employees** and/or **${rev || '>50'} million revenue**, the engine classifies your company as a large enterprise, potentially falling under **NIS2 as an Essential Entity** (if sector is in Annex I). Art. 21 (Risk Management) and Art. 23 (24h Incident Reporting). Max fines: 10M EUR or 2% of global turnover.${sectorNote}`;
      severity = 'danger';
    } else if (emp >= 50 || rev >= 10) {
      answer = uiLang === 'it'
        ? `Classificazione automatizzata: con almeno **50 dipendenti** o **10 milioni di fatturato** (indicato: ${emp ? emp+' dipendenti' : ''}${emp&&rev?' e ':''}${rev ? rev+' milioni' : ''}), il motore classifica il soggetto come potenzialmente rientrante nella **NIS2 come Soggetto Importante** (se il settore rientra nell'Allegato II). Sanzioni: fino a 7M EUR o 1.4% del fatturato.${sectorNote}`
        : `Automated classification: by meeting the threshold of **50 employees** or **10 million revenue** (indicated: ${emp ? emp+' employees' : ''}${emp&&rev?' and ':''}${rev ? rev+' million' : ''}), the engine classifies the entity as potentially falling under **NIS2 as an Important Entity** (if sector is in Annex II). Fines: up to 7M EUR or 1.4% turnover.${sectorNote}`;
      severity = 'warning';
    } else {
      answer = uiLang === 'it'
        ? `Classificazione automatizzata: con **${emp || '<50'} dipendenti** e/o **${rev || '<10'} milioni di fatturato**, il motore classifica il soggetto come micro o piccola impresa. In generale (Art. 2), il soggetto non rientra negli obblighi diretti della NIS2, salvo eccezioni per servizi critici (es. TLD, trust services) o se unico fornitore di un servizio chiave (Art. 2(2)). Attenzione alla supply chain.${sectorNote}`
        : `Automated classification: with **${emp || '<50'} employees** and/or **${rev || '<10'} million revenue**, the engine classifies the entity as a micro or small enterprise. Generally (Art. 2), it does not fall under direct NIS2 obligations, unless managing critical services (e.g. TLD, trust services) or acting as sole provider (Art. 2(2)). Watch out for supply chain requirements.${sectorNote}`;
      severity = 'info';
    }
    
    visitedCategories.value = new Set([...visitedCategories.value, category])
    stats.value.hits++
    
    const fUps = filterAsked([
      uiLang === 'it' ? "Cos'è un soggetto essenziale nella NIS2?" : "What is an essential entity under NIS2?",
      uiLang === 'it' ? "Cosa dice la direttiva sulla supply chain?" : "What does the directive say about supply chain?",
      uiLang === 'it' ? "Quali sono le sanzioni NIS2?" : "What are the NIS2 sanctions?"
    ]);
    
    return {
      hit: true, answer, html: formatAnswer(answer),
      category: getCategoryName(category, uiLang), 
      followUps: dedup(fUps), 
      elapsed: performance.now() - t0,
      refs: extractArticles(answer),
      catLink: getCategoryLink(category, uiLang),
      severity, deadline: '', standards: [],
      glossary: findTerms(answer).map(t => ({ term: t.term.toUpperCase(), def: uiLang === 'it' ? t.it : t.en })),
      confidence: 100, source: category,
    }
  }
  // --- END REAL-TIME TOOLING ---

  // Filter dataset by UI language first, fallback to full dataset
  const langDataset = dataset.value.filter(d =>
    uiLang === 'it' ? isItalian(d.q) : !isItalian(d.q)
  )
  let results = bm25Search(langDataset, query, 3)
  let searchPool = langDataset

  // Fallback: if no good results in lang-filtered set, try full dataset
  if (!results.length || results[0].score <= 0.5) {
    results = bm25Search(dataset.value, query, 3)
    searchPool = dataset.value
  }

  const elapsed = performance.now() - t0

  if (results.length > 0 && results[0].score > 0.5) {
    const entry = searchPool[results[0].idx]
    const cat = entry.c.replace(/_impl$/, '')
    visitedCategories.value = new Set([...visitedCategories.value, cat])
    stats.value.hits++
    const rawFollowUps = findFollowUps(dataset.value, entry.c, uiLang)
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
  if (el) el.scrollTop = el.scrollHeight
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
    const res = await fetch(`${base}dataset.json`)
    loadProgress.value = 60
    dataset.value = await res.json()
    loadProgress.value = 90
    stats.value.entries = dataset.value.length
    await new Promise(r => setTimeout(r, 400))
    loadProgress.value = 100
    await new Promise(r => setTimeout(r, 300))
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
      <button v-if="complianceDoc.length" class="export-btn" @click="exportReport" title="Export Report">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
        <span>Report</span>
      </button>
      <button class="focus-btn" @click="focusMode = !focusMode" :class="{ active: focusMode }" title="Focus mode [F]">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"/></svg>
      </button>
      <button class="lang-btn" @click="lang = lang === 'it' ? 'en' : 'it'">{{ lang.toUpperCase() }}</button>
      <button v-if="!focusMode" class="hd-toggle" @click="showRight = !showRight" :title="t.session">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="2"/><line x1="15" y1="3" x2="15" y2="21"/></svg>
      </button>
    </div>
  </header>

  <div class="body">
    <!-- Left sidebar -->
    <aside v-if="showLeft" class="side side-l">
      <div class="ring-row">
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
        <div v-for="a in areaStatus" :key="a.id" :class="['area', { on: a.visited }]">
          <span class="area-dot"></span>
          <span>{{ a.name }}</span>
        </div>
      </div>

      <div class="side-footer">
        <a href="https://github.com/fabriziosalmi/nis2-public" target="_blank" class="side-link">
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z"/><path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z"/></svg>
          {{ t.linkProject }}
        </a>
        <a href="https://github.com/fabriziosalmi/nis2-model" target="_blank" class="side-link">
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
      <ChatInput v-model:input="input" :disabled="isLoading || !stats.entries" :placeholder="t.placeholder" :disclaimer="t.disclaimer" @send="sendMessage()" />
    </div>

    <!-- Right sidebar: Command Dashboard -->
    <aside v-if="showRight && !focusMode" class="side side-r">
      <!-- Status badge -->
      <div class="side-label">{{ lang === 'it' ? 'Stato Conformità' : 'Compliance Status' }}</div>
      <div :class="['status-badge', coverage.pct >= 70 ? 'status-ok' : coverage.pct > 0 ? 'status-eval' : 'status-none']">
        <span class="status-dot"></span>
        {{ coverage.pct >= 70 ? (lang === 'it' ? 'IN REGOLA' : 'COMPLIANT') : coverage.pct > 0 ? (lang === 'it' ? 'IN VALUTAZIONE' : 'EVALUATING') : (lang === 'it' ? 'NON VALUTATO' : 'NOT ASSESSED') }}
      </div>

      <!-- Normativa identificata -->
      <div class="side-label" style="margin-top:16px">{{ lang === 'it' ? 'Norme Identificate' : 'Identified Legislation' }}</div>
      <div class="side-panel">
        <template v-if="citedArticles.length">
          <a v-for="art in citedArticles" :key="art.num" :href="art.url" target="_blank" class="norm-card">
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
      <button v-if="complianceDoc.length" class="artifact-card" @click="exportReport">
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
  padding:12px 16px;border-bottom:1px solid rgba(255,255,255,.06);flex-shrink:0;
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
  display:flex;flex-direction:column;height:100vh;max-height:100vh;overflow:hidden;
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
.side::-webkit-scrollbar-thumb{background:rgba(255,255,255,.08);border-radius:8px}
.side::-webkit-scrollbar-thumb:hover{background:rgba(255,255,255,.15)}
.side-l{border-right:1px solid rgba(255,255,255,.06);box-shadow:2px 0 4px rgba(0,0,0,.03)}
.side-r{border-left:1px solid rgba(255,255,255,.06);box-shadow:-2px 0 4px rgba(0,0,0,.03)}
.side-label{
  font-size:10px;font-weight:700;text-transform:uppercase;
  letter-spacing:.12em;color:var(--vp-c-text-3);margin-bottom:8px;opacity:.55;
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
  font-size:13px;color:#64748b;transition:all .15s ease-out;
  border-radius:6px;cursor:default;
}
.area:hover{background:rgba(255,255,255,.04);color:#94a3b8}
.area.on{color:#ffffff;font-weight:500}
.area-dot{width:6px;height:6px;border-radius:50%;background:rgba(255,255,255,.1);flex-shrink:0;transition:all .3s}
.area.on .area-dot{background:#60a5fa;box-shadow:0 0 6px rgba(96,165,250,.4)}

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
.status-none{background:rgba(100,116,139,.1);color:#64748b}
.status-none .status-dot{background:#64748b}
.status-eval{background:rgba(251,191,36,.08);color:#fbbf24}
.status-eval .status-dot{background:#fbbf24;box-shadow:0 0 6px rgba(251,191,36,.4)}
.status-ok{background:rgba(16,185,129,.08);color:#34d399}
.status-ok .status-dot{background:#34d399;box-shadow:0 0 6px rgba(16,185,129,.4)}

/* ── Side Panel (content area) ── */
.side-panel{display:flex;flex-direction:column;gap:4px;flex:1;overflow-y:auto;min-height:0}

/* ── Normativa Cards ── */
.norm-card{
  display:flex;align-items:center;gap:8px;
  padding:8px 10px;border-radius:8px;
  text-decoration:none;color:#94a3b8;
  transition:all .15s ease-out;cursor:pointer;
}
.norm-card:hover{background:rgba(255,255,255,.05);color:#e2e8f0}
.norm-card svg{flex-shrink:0;color:#60a5fa;opacity:.6}
.norm-card:hover svg{opacity:.9}
.norm-info{display:flex;flex-direction:column;flex:1;min-width:0}
.norm-title{font-size:12px;font-weight:600;color:#e2e8f0;line-height:1.3}
.norm-sub{font-size:10px;color:#64748b;line-height:1.3;margin-top:1px}
.norm-ext{opacity:0;transition:opacity .15s ease-out}
.norm-card:hover .norm-ext{opacity:.4}

/* ── Artifact Card ── */
.artifact-card{
  display:flex;align-items:center;gap:8px;
  padding:10px 10px;border-radius:8px;border:none;
  background:rgba(16,185,129,.06);color:#94a3b8;
  cursor:pointer;transition:all .15s ease-out;text-align:left;
  margin-bottom:8px;
}
.artifact-card:hover{background:rgba(16,185,129,.1);color:#e2e8f0}
.artifact-card svg{flex-shrink:0;color:#34d399;opacity:.7}
.artifact-card:hover svg{opacity:1}
.artifact-card .norm-ext{opacity:0}
.artifact-card:hover .norm-ext{opacity:.4}

/* ── Doc Section (compact) ── */
.doc-section{
  padding:8px 0;border-bottom:1px solid rgba(255,255,255,.04);
  animation:fadeUp .3s ease;
}
.doc-section:last-child{border-bottom:none}
.doc-head{display:flex;align-items:center;gap:6px;margin-bottom:2px}
.doc-title{font-size:11px;font-weight:600;color:#cbd5e1;flex:1}
.doc-conf{
  font-size:9px;font-weight:700;padding:2px 6px;border-radius:9999px;
  background:rgba(16,185,129,.1);color:#34d399;
  font-variant-numeric:tabular-nums;
}
.doc-refs{display:flex;flex-wrap:wrap;gap:3px}
.doc-ref{font-size:9px;padding:2px 6px;border-radius:9999px;background:rgba(255,255,255,.05);color:#64748b}
.sev-dot-sm{width:5px;height:5px;border-radius:50%;flex-shrink:0}
.sev-dot-sm.critical{background:#f87171}
.sev-dot-sm.high{background:#fbbf24}
.sev-dot-sm.medium{background:#a78bfa}
.sev-dot-sm.info{background:#60a5fa}

.empty-session{
  font-size:12px;color:#64748b;opacity:.6;
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

@media(max-width:960px){.side{display:none}.hd-toggle{display:none}.focus-btn{display:none}}
@media(max-width:640px){.w-grid{grid-template-columns:1fr}.hd-sub{display:none}}

/* ══════════════════════════════════════════════
   LIGHT MODE OVERRIDES
   ══════════════════════════════════════════════ */

/* Sidebar areas */
:root:not(.dark) .area{color:#475569}
:root:not(.dark) .area:hover{background:rgba(0,0,0,.04);color:#1e293b}
:root:not(.dark) .area.on{color:#1e293b;font-weight:600}
:root:not(.dark) .area-dot{background:rgba(0,0,0,.1)}
:root:not(.dark) .area.on .area-dot{background:#2563eb;box-shadow:0 0 6px rgba(37,99,235,.35)}

/* Sidebar scrollbar */
:root:not(.dark) .side::-webkit-scrollbar-thumb{background:rgba(0,0,0,.1)}
:root:not(.dark) .side::-webkit-scrollbar-thumb:hover{background:rgba(0,0,0,.18)}

/* Right panel — norm cards */
:root:not(.dark) .norm-card{color:#475569}
:root:not(.dark) .norm-card:hover{background:rgba(0,0,0,.04);color:#1e293b}
:root:not(.dark) .norm-card svg{color:#2563eb}
:root:not(.dark) .norm-title{color:#1e293b}
:root:not(.dark) .norm-sub{color:#64748b}

/* Right panel — artifact cards */
:root:not(.dark) .artifact-card{background:rgba(16,185,129,.06);color:#475569}
:root:not(.dark) .artifact-card:hover{background:rgba(16,185,129,.1);color:#1e293b}
:root:not(.dark) .artifact-card svg{color:#059669}

/* Status badges */
:root:not(.dark) .status-none{background:rgba(100,116,139,.08);color:#475569}
:root:not(.dark) .status-none .status-dot{background:#64748b}
:root:not(.dark) .status-eval{background:rgba(217,119,6,.08);color:#b45309}
:root:not(.dark) .status-eval .status-dot{background:#d97706;box-shadow:0 0 6px rgba(217,119,6,.35)}
:root:not(.dark) .status-ok{background:rgba(5,150,105,.08);color:#059669}
:root:not(.dark) .status-ok .status-dot{background:#059669;box-shadow:0 0 6px rgba(5,150,105,.35)}

/* Doc section */
:root:not(.dark) .doc-section{border-bottom-color:rgba(0,0,0,.06)}
:root:not(.dark) .doc-title{color:#1e293b}
:root:not(.dark) .doc-ref{background:rgba(0,0,0,.05);color:#475569}
:root:not(.dark) .doc-conf{background:rgba(5,150,105,.08);color:#059669}

/* Empty session */
:root:not(.dark) .empty-session{color:#64748b}
</style>
