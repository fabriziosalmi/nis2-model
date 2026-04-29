<script setup>
import { ref, onMounted, nextTick, computed } from 'vue'
import ChatMessages from './ChatMessages.vue'
import ChatInput from './ChatInput.vue'
import { bm25Search, isItalian, findFollowUps } from './search.js'
import { formatAnswer } from './formatter.js'
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

// SVG ring calculations
const ringR = 32
const ringC = 2 * Math.PI * ringR
const ringOffset = computed(() => ringC - (coverage.value.pct / 100) * ringC)

// NIS2 public resources — contextual based on explored areas
const NIS2_RESOURCES = {
  it: [
    { label: 'Strumento di autovalutazione', url: 'https://github.com/fabriziosalmi/nis2-public', desc: 'Verifica se la tua organizzazione rientra in NIS2' },
    { label: 'Documentazione NIS2', url: 'https://github.com/fabriziosalmi/nis2-public#readme', desc: 'Guida completa al framework NIS2' },
  ],
  en: [
    { label: 'Self-assessment tool', url: 'https://github.com/fabriziosalmi/nis2-public', desc: 'Check if your organization falls under NIS2' },
    { label: 'NIS2 Documentation', url: 'https://github.com/fabriziosalmi/nis2-public#readme', desc: 'Complete guide to the NIS2 framework' },
  ],
}
const resources = computed(() => NIS2_RESOURCES[lang.value] || NIS2_RESOURCES.en)

function filterAsked(items) {
  return items.filter(q => !askedQuestions.value.has(q.toLowerCase()))
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
  const results = bm25Search(dataset.value, query, 3)
  const elapsed = performance.now() - t0
  const uiLang = lang.value

  if (results.length > 0 && results[0].score > 0.5) {
    const entry = dataset.value[results[0].idx]
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
      category: getCategoryName(entry.c, uiLang), followUps, elapsed,
    }
  }
  const missStrings = getStrings(uiLang)
  let followUps = filterAsked(missStrings.missSuggestions)
  if (followUps.length < 2) followUps = [...followUps, ...exploreSuggestions(uiLang, 4 - followUps.length)]
  return { hit: false, elapsed, followUps }
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
    const msg = { role:'assistant', text:result.answer, html:result.html, category:result.category, followUps:result.followUps, typing:true, displayHtml:'' }
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
</script>

<template>
<!-- Loader -->
<div v-if="!ready" class="loader">
  <div class="loader-inner">
    <svg class="loader-shield" width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
    <div class="loader-text">{{ t.loading }}</div>
    <div class="loader-bar"><div class="loader-fill" :style="{width: loadProgress+'%'}"></div></div>
  </div>
</div>

<!-- App -->
<div v-else class="app">
  <!-- Header -->
  <header class="hd">
    <div class="hd-left">
      <svg class="hd-icon" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
      <div>
        <div class="hd-title">{{ t.title }}</div>
        <div class="hd-sub">{{ stats.entries }} entries · {{ t.sub }}</div>
      </div>
    </div>
    <div class="hd-right">
      <button class="lang-btn" @click="lang = lang === 'it' ? 'en' : 'it'">
        {{ lang === 'it' ? 'IT' : 'EN' }}
      </button>
    </div>
  </header>

  <div class="body">
    <!-- Left: Coverage -->
    <aside class="side side-l">
      <!-- Ring -->
      <div class="ring-wrap">
        <svg class="ring" viewBox="0 0 80 80">
          <circle cx="40" cy="40" :r="ringR" fill="none" stroke="var(--vp-c-divider)" stroke-width="3"/>
          <circle cx="40" cy="40" :r="ringR" fill="none" stroke="var(--vp-c-brand-1)" stroke-width="3"
            stroke-linecap="round" :stroke-dasharray="ringC" :stroke-dashoffset="ringOffset"
            transform="rotate(-90 40 40)" class="ring-progress"/>
        </svg>
        <div class="ring-label">
          <span class="ring-pct">{{ coverage.pct }}</span>
          <span class="ring-unit">%</span>
        </div>
      </div>
      <div class="ring-caption">{{ coverage.explored }}/{{ coverage.total }} {{ t.coverageLabel.toLowerCase() }}</div>

      <!-- Area list -->
      <div class="areas">
        <div v-for="a in areaStatus" :key="a.id" :class="['area', { on: a.visited }]">
          <span class="area-dot"></span>
          <span>{{ a.name }}</span>
        </div>
      </div>

      <!-- Resources -->
      <div class="res-section">
        <div class="side-label">{{ t.linkProject }}</div>
        <a v-for="r in resources" :key="r.url" :href="r.url" target="_blank" class="res-card">
          <div class="res-name">{{ r.label }}</div>
          <div class="res-desc">{{ r.desc }}</div>
        </a>
      </div>
    </aside>

    <!-- Center: Chat -->
    <div class="center">
      <ChatMessages ref="chatEl" :messages="messages" :isLoading="isLoading" @followUp="sendMessage">
        <template #welcome>
          <div v-if="messages.length === 0" class="welcome">
            <svg class="w-icon" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
            <h2 class="w-title">{{ t.welcome }}</h2>
            <p class="w-sub">{{ stats.entries }} {{ t.welcomeSub }}</p>
            <div class="w-grid">
              <button v-for="s in t.suggestions" :key="s" @click="sendMessage(s)" class="w-card">
                <span class="w-card-text">{{ s }}</span>
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="9 18 15 12 9 6"/></svg>
              </button>
            </div>
          </div>
        </template>
      </ChatMessages>
      <ChatInput v-model:input="input" :disabled="isLoading || !stats.entries" :placeholder="t.placeholder" @send="sendMessage()" />
    </div>

    <!-- Right: Session -->
    <aside class="side side-r">
      <div class="side-label">{{ t.session }}</div>
      <div class="metrics">
        <div class="metric">
          <div class="metric-val">{{ stats.queries }}</div>
          <div class="metric-label">{{ t.queries }}</div>
        </div>
        <div class="metric">
          <div class="metric-val">{{ stats.hits }}</div>
          <div class="metric-label">{{ t.hits }}</div>
        </div>
        <div class="metric">
          <div class="metric-val">{{ stats.queries ? stats.avgMs.toFixed(0) : '–' }}</div>
          <div class="metric-label">ms avg</div>
        </div>
      </div>

      <template v-if="messages.filter(m=>m.role==='user').length">
        <div class="side-label" style="margin-top:20px">{{ t.recent }}</div>
        <div class="trail">
          <button v-for="(m, i) in messages.filter(m => m.role === 'user').slice(-8)" :key="i" class="trail-btn" @click="sendMessage(m.text)">
            {{ m.text }}
          </button>
        </div>
      </template>

      <!-- nis2-public CTA -->
      <div class="cta">
        <a href="https://github.com/fabriziosalmi/nis2-public" target="_blank" class="cta-link">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"/></svg>
          fabriziosalmi/nis2-public
        </a>
        <a href="https://github.com/fabriziosalmi/nis2-model" target="_blank" class="cta-link">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"/></svg>
          fabriziosalmi/nis2-model
        </a>
      </div>
    </aside>
  </div>
</div>
</template>

<style scoped>
*{box-sizing:border-box}
.app,.loader{
  font-family:-apple-system,BlinkMacSystemFont,'SF Pro Text','SF Pro Display','Helvetica Neue',sans-serif;
  -webkit-font-smoothing:antialiased;-moz-osx-font-smoothing:grayscale;
  color:var(--vp-c-text-1);
}

/* === LOADER === */
.loader{position:fixed;inset:0;display:flex;align-items:center;justify-content:center;background:var(--vp-c-bg);z-index:999}
.loader-inner{text-align:center;animation:fadeUp .5s ease}
.loader-shield{color:var(--vp-c-text-3);margin-bottom:16px;animation:pulse 2s infinite}
.loader-text{font-size:13px;color:var(--vp-c-text-3);margin-bottom:16px;letter-spacing:-.01em}
.loader-bar{width:160px;height:2px;background:var(--vp-c-divider);border-radius:1px;overflow:hidden;margin:0 auto}
.loader-fill{height:100%;background:var(--vp-c-brand-1);border-radius:1px;transition:width .3s}

/* === HEADER === */
.hd{display:flex;align-items:center;justify-content:space-between;padding:8px 16px;border-bottom:1px solid var(--vp-c-divider);flex-shrink:0}
.hd-left{display:flex;align-items:center;gap:10px}
.hd-icon{color:var(--vp-c-brand-1)}
.hd-title{font-size:13px;font-weight:600;letter-spacing:-.02em}
.hd-sub{font-size:10.5px;color:var(--vp-c-text-3);letter-spacing:-.01em}
.hd-right{display:flex;align-items:center;gap:8px}
.lang-btn{
  font-size:11px;font-weight:600;padding:3px 10px;
  border:1px solid var(--vp-c-divider);border-radius:6px;
  background:var(--vp-c-bg-soft);color:var(--vp-c-text-2);
  cursor:pointer;transition:all .15s;letter-spacing:.02em;
}
.lang-btn:hover{border-color:var(--vp-c-brand-1);color:var(--vp-c-brand-1)}

/* === LAYOUT === */
.app{display:flex;flex-direction:column;height:100vh;max-height:100vh;overflow:hidden;background:var(--vp-c-bg)}
.body{display:flex;flex:1;overflow:hidden}
.center{flex:1;display:flex;flex-direction:column;overflow:hidden;min-width:0}

/* === SIDEBARS === */
.side{width:200px;flex-shrink:0;padding:16px;overflow-y:auto;display:flex;flex-direction:column;background:var(--vp-c-bg-soft)}
.side-l{border-right:1px solid var(--vp-c-divider)}
.side-r{border-left:1px solid var(--vp-c-divider)}
.side-label{font-size:10px;font-weight:600;text-transform:uppercase;letter-spacing:.08em;color:var(--vp-c-text-3);margin-bottom:10px}

/* Coverage ring */
.ring-wrap{position:relative;width:80px;height:80px;margin:0 auto 4px}
.ring{width:100%;height:100%}
.ring-progress{transition:stroke-dashoffset .6s ease}
.ring-label{position:absolute;inset:0;display:flex;align-items:center;justify-content:center;gap:1px}
.ring-pct{font-size:20px;font-weight:700;font-variant-numeric:tabular-nums;letter-spacing:-.03em;color:var(--vp-c-text-1)}
.ring-unit{font-size:11px;font-weight:500;color:var(--vp-c-text-3);margin-top:4px}
.ring-caption{text-align:center;font-size:10.5px;color:var(--vp-c-text-3);margin-bottom:14px;letter-spacing:-.01em}

/* Area list */
.areas{display:flex;flex-direction:column;gap:1px;margin-bottom:16px}
.area{display:flex;align-items:center;gap:7px;padding:3px 4px;font-size:11.5px;color:var(--vp-c-text-3);transition:color .2s;letter-spacing:-.01em}
.area.on{color:var(--vp-c-text-1);font-weight:500}
.area-dot{width:5px;height:5px;border-radius:50%;background:var(--vp-c-divider);flex-shrink:0;transition:all .3s}
.area.on .area-dot{background:var(--vp-c-brand-1);box-shadow:0 0 4px rgba(59,130,246,.3)}

/* Resources */
.res-section{margin-top:auto;padding-top:12px;border-top:1px solid var(--vp-c-divider)}
.res-card{
  display:block;text-decoration:none;padding:8px;border-radius:6px;
  margin-top:6px;transition:background .15s;
}
.res-card:hover{background:var(--vp-c-bg)}
.res-name{font-size:11.5px;font-weight:600;color:var(--vp-c-text-1);letter-spacing:-.01em}
.res-desc{font-size:10.5px;color:var(--vp-c-text-3);margin-top:2px;line-height:1.4}

/* Session metrics */
.metrics{display:flex;gap:4px}
.metric{
  flex:1;text-align:center;padding:8px 4px;
  border-radius:6px;background:var(--vp-c-bg);
}
.metric-val{font-size:18px;font-weight:700;font-variant-numeric:tabular-nums;letter-spacing:-.03em;color:var(--vp-c-text-1)}
.metric-label{font-size:9.5px;color:var(--vp-c-text-3);text-transform:uppercase;letter-spacing:.04em;margin-top:2px}

/* Trail */
.trail{display:flex;flex-direction:column;gap:1px}
.trail-btn{
  text-align:left;font-size:11px;color:var(--vp-c-text-3);
  padding:4px 6px;border:none;background:none;border-radius:4px;
  cursor:pointer;transition:all .12s;letter-spacing:-.01em;
  display:-webkit-box;-webkit-line-clamp:2;-webkit-box-orient:vertical;overflow:hidden;
  line-height:1.4;
}
.trail-btn:hover{background:var(--vp-c-bg);color:var(--vp-c-text-1)}

/* CTA */
.cta{margin-top:auto;padding-top:12px;border-top:1px solid var(--vp-c-divider);display:flex;flex-direction:column;gap:6px}
.cta-link{
  font-size:11px;color:var(--vp-c-text-3);text-decoration:none;
  display:flex;align-items:center;gap:6px;transition:color .12s;
}
.cta-link:hover{color:var(--vp-c-brand-1)}

/* === WELCOME === */
.welcome{display:flex;flex-direction:column;align-items:center;justify-content:center;flex:1;text-align:center;padding:40px 24px;animation:fadeUp .5s ease}
.w-icon{color:var(--vp-c-text-3);opacity:.15;margin-bottom:8px}
.w-title{font-size:18px;font-weight:600;margin:0;letter-spacing:-.03em;color:var(--vp-c-text-1)}
.w-sub{color:var(--vp-c-text-3);font-size:12.5px;margin:4px 0 0;letter-spacing:-.01em}
.w-grid{display:grid;grid-template-columns:1fr 1fr;gap:6px;max-width:480px;margin-top:20px;width:100%}
.w-card{
  display:flex;align-items:center;justify-content:space-between;
  text-align:left;padding:10px 12px;
  border:1px solid var(--vp-c-divider);border-radius:8px;
  background:var(--vp-c-bg);color:var(--vp-c-text-2);
  font-size:12.5px;cursor:pointer;transition:all .15s;
  letter-spacing:-.01em;line-height:1.4;gap:8px;
}
.w-card:hover{border-color:var(--vp-c-brand-1);color:var(--vp-c-text-1)}
.w-card svg{flex-shrink:0;opacity:.2;transition:opacity .15s}
.w-card:hover svg{opacity:.5}
.w-card-text{flex:1}

@keyframes fadeUp{from{opacity:0;transform:translateY(6px)}to{opacity:1;transform:translateY(0)}}
@keyframes pulse{0%,100%{opacity:.4}50%{opacity:.8}}

@media(max-width:960px){.side{display:none}}
@media(max-width:640px){
  .w-grid{grid-template-columns:1fr}
  .hd-sub{display:none}
}
</style>
