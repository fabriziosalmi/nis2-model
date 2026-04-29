<script setup>
import { ref, onMounted, nextTick, computed, watch } from 'vue'
import ChatMessages from './ChatMessages.vue'
import ChatInput from './ChatInput.vue'
import { bm25Search, isItalian, findFollowUps } from './search.js'
import { formatAnswer } from './formatter.js'

const dataset = ref([])
const messages = ref([])
const input = ref('')
const isLoading = ref(false)
const chatEl = ref(null)
const stats = ref({ entries: 0, queries: 0, hits: 0, avgMs: 0 })
const totalMs = ref(0)
const ready = ref(false)
const loadProgress = ref(0)

// Auto-detect browser language
const lang = ref('en')
onMounted(() => {
  const nav = navigator.language || navigator.userLanguage || 'en'
  if (nav.startsWith('it')) lang.value = 'it'
})

const t = computed(() => lang.value === 'it' ? {
  title: 'NIS2 Compliance Engine',
  sub: 'Motore deterministico · Zero LLM · Open Source',
  placeholder: 'Chiedi qualcosa sulla conformità NIS2...',
  loading: 'Caricamento knowledge base...',
  welcome: 'Chiedi qualsiasi cosa su NIS2',
  welcomeSub: 'risposte pre-calcolate · Italiano/Inglese · Art. 20, 21, 23, 34',
  miss: 'Domanda non trovata nella knowledge base. Prova a riformulare.',
  coverage: 'Copertura',
  session: 'Sessione',
  queries: 'Domande',
  hits: 'Risposte',
  areas: 'Aree',
  recent: 'Recenti',
  linkProject: 'Progetto NIS2 principale',
  linkEngine: 'Motore compliance',
} : {
  title: 'NIS2 Compliance Engine',
  sub: 'Deterministic engine · Zero LLM · Open Source',
  placeholder: 'Ask anything about NIS2 compliance...',
  loading: 'Loading knowledge base...',
  welcome: 'Ask anything about NIS2',
  welcomeSub: 'pre-computed answers · Italian/English · Art. 20, 21, 23, 34',
  miss: 'Question not found in the knowledge base. Try rephrasing.',
  coverage: 'Coverage',
  session: 'Session',
  queries: 'Queries',
  hits: 'Hits',
  areas: 'Areas',
  recent: 'Recent',
  linkProject: 'Main NIS2 project',
  linkEngine: 'Compliance engine',
})

const visitedCategories = ref(new Set())
const AREAS = [
  { id:'governance', icon:'shield-check' },
  { id:'access_control', icon:'key-round' },
  { id:'encryption', icon:'lock' },
  { id:'incident_response', icon:'siren' },
  { id:'business_continuity', icon:'refresh-cw' },
  { id:'supply_chain', icon:'link' },
  { id:'vulnerability_mgmt', icon:'bug' },
  { id:'risk_assessment', icon:'activity' },
  { id:'network_security', icon:'wifi' },
  { id:'detection', icon:'eye' },
  { id:'email_security', icon:'mail' },
  { id:'documentation', icon:'file-text' },
  { id:'remote_work', icon:'laptop' },
  { id:'physical', icon:'building' },
  { id:'legal', icon:'scale' },
  { id:'sanctions', icon:'alert-triangle' },
]
const coverage = computed(() => {
  const explored = AREAS.filter(a => visitedCategories.value.has(a.id)).length
  return { explored, total: AREAS.length }
})
const areaStatus = computed(() =>
  AREAS.map(a => ({ ...a, name: a.id.replace(/_/g, ' '), visited: visitedCategories.value.has(a.id) }))
)

const suggestions = computed(() => lang.value === 'it' ? [
  { text: 'La mia azienda rientra nella NIS2?', icon: 'building' },
  { text: 'Da dove iniziare con NIS2?', icon: 'compass' },
  { text: 'Ci hanno hackerato, cosa facciamo?', icon: 'siren' },
  { text: 'Quanto costa adeguarsi a NIS2?', icon: 'coins' },
  { text: 'Serve la crittografia?', icon: 'lock' },
  { text: 'Quali sono le sanzioni NIS2?', icon: 'alert-triangle' },
] : [
  { text: 'Does NIS2 apply to my company?', icon: 'building' },
  { text: 'Where to start with NIS2?', icon: 'compass' },
  { text: 'We got hacked, what do we do?', icon: 'siren' },
  { text: 'What are the NIS2 obligations?', icon: 'clipboard-list' },
  { text: 'Do we need encryption?', icon: 'lock' },
  { text: 'What are the NIS2 sanctions?', icon: 'alert-triangle' },
])

function search(query) {
  const t0 = performance.now()
  const results = bm25Search(dataset.value, query, 3)
  const elapsed = performance.now() - t0
  const qLang = isItalian(query) ? 'it' : lang.value

  if (results.length > 0 && results[0].score > 0.5) {
    const entry = dataset.value[results[0].idx]
    const cat = entry.c.replace(/_impl$/, '')
    visitedCategories.value = new Set([...visitedCategories.value, cat])
    stats.value.hits++
    return {
      hit: true, answer: entry.a, html: formatAnswer(entry.a),
      category: entry.c, score: results[0].score,
      followUps: findFollowUps(dataset.value, entry.c, qLang), elapsed,
    }
  }
  const miss = qLang === 'it'
    ? ['La mia azienda rientra nella NIS2?','Da dove iniziare con NIS2?','Quali sono gli obblighi NIS2?','Quanto costa adeguarsi?']
    : ['Does NIS2 apply to my company?','Where to start with NIS2?','What are the NIS2 obligations?','What are the sanctions?']
  return { hit: false, elapsed, followUps: miss }
}

async function sendMessage(text) {
  const query = (text || input.value).trim()
  if (!query || isLoading.value) return
  input.value = ''
  messages.value.push({ role: 'user', text: query })
  isLoading.value = true
  stats.value.queries++
  await nextTick(); scrollBottom()
  await new Promise(r => setTimeout(r, 50))

  const result = search(query)
  totalMs.value += result.elapsed
  stats.value.avgMs = totalMs.value / stats.value.queries

  if (result.hit) {
    const msg = { role:'assistant', text:result.answer, html:result.html, category:result.category, score:result.score, elapsed:result.elapsed, followUps:result.followUps, typing:true, displayHtml:'' }
    messages.value.push(msg)
    const words = result.answer.split(' ')
    for (let i = 0; i < words.length; i++) {
      msg.displayHtml = formatAnswer(words.slice(0, i + 1).join(' '))
      if (i % 3 === 0) { await new Promise(r => setTimeout(r, 8)); scrollBottom() }
    }
    msg.typing = false; msg.displayHtml = result.html
  } else {
    messages.value.push({ role:'assistant', text: t.value.miss, html: t.value.miss, category:'miss', elapsed:result.elapsed, followUps:result.followUps, typing:false, displayHtml:'' })
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
    <div class="loader-icon">
      <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
    </div>
    <h2>NIS2 Compliance Engine</h2>
    <p>{{ t.loading }}</p>
    <div class="loader-bar"><div class="loader-fill" :style="{width: loadProgress+'%'}"></div></div>
    <span class="loader-pct">{{ loadProgress }}%</span>
  </div>
</div>

<!-- Main -->
<div v-else class="cb">
  <header class="cb-hd">
    <div class="cb-hd-l">
      <div class="cb-logo">
        <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
      </div>
      <div>
        <h1 class="cb-title">{{ t.title }}</h1>
        <p class="cb-sub">{{ stats.entries }} entries · {{ t.sub }}</p>
      </div>
    </div>
    <div class="cb-pills">
      <button class="pill pill-lang" @click="lang = lang === 'it' ? 'en' : 'it'">
        {{ lang === 'it' ? '🇮🇹 IT' : '🇬🇧 EN' }}
      </button>
      <div class="pill">
        <span class="pill-l">{{ t.coverage }}</span>
        <span class="pill-v">{{ coverage.explored }}/{{ coverage.total }}</span>
        <div class="cbar"><div class="cfill" :style="{width:(coverage.explored/coverage.total*100)+'%'}"></div></div>
      </div>
      <div class="pill" v-if="stats.queries > 0">
        <span class="pill-v">{{ stats.avgMs.toFixed(1) }}ms</span>
      </div>
    </div>
  </header>

  <div class="cb-body">
    <aside class="cb-side cb-side-l">
      <div class="side-hd">Art. 21 {{ t.coverage }}</div>
      <div class="area-list">
        <div v-for="a in areaStatus" :key="a.id" :class="['area-item', { visited: a.visited }]">
          <span :class="['area-check', { on: a.visited }]">
            <svg v-if="a.visited" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>
            <svg v-else width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/></svg>
          </span>
          <span class="area-name">{{ a.name }}</span>
        </div>
      </div>
      <div class="side-links">
        <a href="https://github.com/fabriziosalmi/nis2-public" target="_blank" class="side-link">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z"/><path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z"/></svg>
          {{ t.linkProject }}
        </a>
        <a href="https://github.com/fabriziosalmi/nis2-model" target="_blank" class="side-link">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>
          {{ t.linkEngine }}
        </a>
      </div>
    </aside>

    <div class="cb-center">
      <ChatMessages ref="chatEl" :messages="messages" :isLoading="isLoading" @followUp="sendMessage">
        <template #welcome>
          <div v-if="messages.length === 0" class="welcome">
            <div class="w-shield">
              <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
            </div>
            <h2>{{ t.welcome }}</h2>
            <p>{{ stats.entries }} {{ t.welcomeSub }}</p>
            <div class="suggestions">
              <button v-for="s in suggestions" :key="s.text" @click="sendMessage(s.text)" class="sug-btn">
                {{ s.text }}
              </button>
            </div>
          </div>
        </template>
      </ChatMessages>
      <ChatInput v-model:input="input" :disabled="isLoading || !stats.entries" :placeholder="t.placeholder" @send="sendMessage()" />
    </div>

    <aside class="cb-side cb-side-r">
      <div class="side-hd">{{ t.session }}</div>
      <div class="ss">
        <div class="ss-row"><span>{{ t.queries }}</span><span class="ss-val">{{ stats.queries }}</span></div>
        <div class="ss-row"><span>{{ t.hits }}</span><span class="ss-val">{{ stats.hits }}</span></div>
        <div class="ss-row"><span>{{ t.areas }}</span><span class="ss-val">{{ coverage.explored }}/{{ coverage.total }}</span></div>
      </div>
      <div v-if="messages.filter(m=>m.role==='user').length" class="side-hd" style="margin-top:20px">{{ t.recent }}</div>
      <div class="trail">
        <div v-for="(m, i) in messages.filter(m => m.role === 'user').slice(-10)" :key="i" class="trail-item" @click="sendMessage(m.text)">
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="9 18 15 12 9 6"/></svg>
          {{ m.text.length > 36 ? m.text.slice(0,36) + '…' : m.text }}
        </div>
      </div>
    </aside>
  </div>
</div>
</template>

<style scoped>
*{box-sizing:border-box}

/* Apple system font stack */
.cb,.loader{font-family:-apple-system,BlinkMacSystemFont,'SF Pro Text','SF Pro Display','Helvetica Neue',Helvetica,Arial,sans-serif;-webkit-font-smoothing:antialiased;-moz-osx-font-smoothing:grayscale}

/* === LOADER === */
.loader{position:fixed;inset:0;display:flex;align-items:center;justify-content:center;background:var(--vp-c-bg);z-index:999}
.loader-inner{text-align:center;animation:loaderIn .6s ease}
.loader-icon{margin:0 auto 20px;color:var(--vp-c-brand-1);animation:pulse 2s infinite}
.loader-icon svg{width:48px;height:48px}
.loader h2{font-size:22px;font-weight:600;margin:0 0 6px;letter-spacing:-.02em;color:var(--vp-c-text-1)}
.loader p{font-size:13px;color:var(--vp-c-text-3);margin:0 0 20px}
.loader-bar{width:200px;height:3px;background:var(--vp-c-divider);border-radius:2px;overflow:hidden;margin:0 auto}
.loader-fill{height:100%;background:var(--vp-c-brand-1);border-radius:2px;transition:width .3s ease}
.loader-pct{font-size:11px;color:var(--vp-c-text-3);margin-top:8px;display:block;font-variant-numeric:tabular-nums}

/* === MAIN === */
.cb{display:flex;flex-direction:column;height:100vh;max-height:100vh;overflow:hidden;background:var(--vp-c-bg)}

/* Header */
.cb-hd{display:flex;align-items:center;justify-content:space-between;padding:10px 16px;background:var(--vp-c-bg);border-bottom:1px solid var(--vp-c-divider);flex-shrink:0;gap:12px;flex-wrap:wrap}
.cb-hd-l{display:flex;align-items:center;gap:10px}
.cb-logo{width:36px;height:36px;display:flex;align-items:center;justify-content:center;background:var(--vp-c-brand-1);border-radius:10px}
.cb-title{font-size:15px;font-weight:600;margin:0;letter-spacing:-.02em;color:var(--vp-c-text-1)}
.cb-sub{font-size:11px;color:var(--vp-c-text-3);margin:1px 0 0;letter-spacing:-.01em}

.cb-pills{display:flex;gap:6px;align-items:center}
.pill{display:flex;align-items:center;gap:5px;padding:4px 10px;background:var(--vp-c-bg-soft);border:1px solid var(--vp-c-divider);border-radius:8px;font-size:11px}
.pill-lang{cursor:pointer;font-weight:600;transition:background .2s}
.pill-lang:hover{background:var(--vp-c-brand-soft)}
.pill-l{color:var(--vp-c-text-3);font-weight:500}
.pill-v{color:var(--vp-c-text-1);font-weight:600;font-variant-numeric:tabular-nums}
.cbar{width:40px;height:3px;background:var(--vp-c-divider);border-radius:2px;overflow:hidden}
.cfill{height:100%;background:var(--vp-c-brand-1);border-radius:2px;transition:width .4s}

/* Body */
.cb-body{display:flex;flex:1;overflow:hidden}

/* Sidebars */
.cb-side{width:200px;flex-shrink:0;padding:16px;overflow-y:auto;display:flex;flex-direction:column}
.cb-side-l{border-right:1px solid var(--vp-c-divider)}
.cb-side-r{border-left:1px solid var(--vp-c-divider)}
.side-hd{font-size:10px;font-weight:600;text-transform:uppercase;letter-spacing:.1em;color:var(--vp-c-text-3);margin-bottom:12px}

.area-list{display:flex;flex-direction:column;gap:2px}
.area-item{display:flex;align-items:center;gap:8px;padding:5px 8px;border-radius:6px;font-size:12px;color:var(--vp-c-text-3);transition:all .2s;letter-spacing:-.01em}
.area-item.visited{color:var(--vp-c-text-1)}
.area-check{width:18px;height:18px;display:flex;align-items:center;justify-content:center;border-radius:50%;flex-shrink:0}
.area-check.on{color:var(--vp-c-brand-1)}
.area-name{text-transform:capitalize}

.side-links{margin-top:auto;padding-top:16px;border-top:1px solid var(--vp-c-divider);display:flex;flex-direction:column;gap:8px}
.side-link{font-size:12px;color:var(--vp-c-text-3);text-decoration:none;display:flex;align-items:center;gap:6px;transition:color .2s;letter-spacing:-.01em}
.side-link:hover{color:var(--vp-c-brand-1)}

/* Session sidebar */
.ss{display:flex;flex-direction:column;gap:8px}
.ss-row{display:flex;justify-content:space-between;font-size:12px;color:var(--vp-c-text-3);letter-spacing:-.01em}
.ss-val{font-weight:600;color:var(--vp-c-text-1);font-variant-numeric:tabular-nums}

.trail{display:flex;flex-direction:column;gap:2px}
.trail-item{font-size:11px;color:var(--vp-c-text-3);padding:5px 8px;border-radius:6px;cursor:pointer;transition:all .15s;display:flex;align-items:center;gap:4px;letter-spacing:-.01em}
.trail-item:hover{background:var(--vp-c-bg-soft);color:var(--vp-c-text-1)}
.trail-item svg{flex-shrink:0;opacity:.4}

/* Center */
.cb-center{flex:1;display:flex;flex-direction:column;overflow:hidden;min-width:0}

/* Welcome */
.welcome{display:flex;flex-direction:column;align-items:center;justify-content:center;flex:1;text-align:center;gap:12px;animation:fadeIn .8s ease}
.w-shield{color:var(--vp-c-brand-1);opacity:.15;margin-bottom:-4px}
.welcome h2{font-size:22px;font-weight:600;margin:0;letter-spacing:-.03em;color:var(--vp-c-text-1)}
.welcome p{color:var(--vp-c-text-3);font-size:13px;margin:0;letter-spacing:-.01em}
.suggestions{display:flex;flex-wrap:wrap;gap:6px;justify-content:center;max-width:560px;margin-top:8px}
.sug-btn{padding:8px 14px;border:1px solid var(--vp-c-divider);border-radius:20px;background:var(--vp-c-bg);color:var(--vp-c-text-2);font-size:13px;cursor:pointer;transition:all .2s;letter-spacing:-.01em}
.sug-btn:hover{border-color:var(--vp-c-brand-1);color:var(--vp-c-text-1);transform:translateY(-1px);box-shadow:0 2px 8px rgba(0,0,0,.04)}

@keyframes fadeIn{from{opacity:0;transform:translateY(8px)}to{opacity:1;transform:translateY(0)}}
@keyframes loaderIn{from{opacity:0;transform:scale(.96)}to{opacity:1;transform:scale(1)}}
@keyframes pulse{0%,100%{opacity:.6}50%{opacity:1}}

@media(max-width:900px){.cb-side{display:none}.cb-pills .pill:not(.pill-lang){display:none}}
</style>
