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
  return { explored, total: AREA_IDS.length }
})
const areaStatus = computed(() =>
  AREA_IDS.map(id => ({
    id,
    name: getCategoryName(id, lang.value),
    visited: visitedCategories.value.has(id),
  }))
)

function filterAsked(items) {
  return items.filter(q => !askedQuestions.value.has(q.toLowerCase()))
}

function exploreSuggestions(qLang, limit = 4) {
  const explored = visitedCategories.value
  const unexplored = AREA_IDS.filter(a => !explored.has(a))
  if (!unexplored.length) return []
  const candidates = []
  for (const cat of unexplored) {
    const entries = dataset.value.filter(d => {
      const dc = d.c.replace(/_impl$/, '')
      return dc === cat && (qLang === 'it' ? isItalian(d.q) : !isItalian(d.q))
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
  // UI language is the source of truth for follow-ups and labels
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

<div v-else class="cb">
  <header class="cb-hd">
    <div class="cb-hd-l">
      <div class="cb-logo">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
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
        <span class="pill-l">{{ t.coverageLabel }}</span>
        <span class="pill-v">{{ coverage.explored }}/{{ coverage.total }}</span>
      </div>
    </div>
  </header>

  <div class="cb-body">
    <aside class="cb-side cb-side-l">
      <div class="side-hd">{{ t.coverageLabel }}</div>
      <div class="area-list">
        <div v-for="a in areaStatus" :key="a.id" :class="['area-item', { visited: a.visited }]">
          <span :class="['area-dot', { on: a.visited }]"></span>
          <span class="area-name">{{ a.name }}</span>
        </div>
      </div>
      <div class="side-links">
        <a href="https://github.com/fabriziosalmi/nis2-public" target="_blank" class="side-link">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z"/><path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z"/></svg>
          {{ t.linkProject }}
        </a>
        <a href="https://github.com/fabriziosalmi/nis2-model" target="_blank" class="side-link">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"/></svg>
          {{ t.linkEngine }}
        </a>
      </div>
    </aside>

    <div class="cb-center">
      <ChatMessages ref="chatEl" :messages="messages" :isLoading="isLoading" @followUp="sendMessage">
        <template #welcome>
          <div v-if="messages.length === 0" class="welcome">
            <div class="w-shield">
              <svg width="56" height="56" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
            </div>
            <h2>{{ t.welcome }}</h2>
            <p>{{ stats.entries }} {{ t.welcomeSub }}</p>
            <div class="suggestions">
              <button v-for="s in t.suggestions" :key="s" @click="sendMessage(s)" class="sug-btn">{{ s }}</button>
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
      <template v-if="messages.filter(m=>m.role==='user').length">
        <div class="side-hd" style="margin-top:20px">{{ t.recent }}</div>
        <div class="trail">
          <div v-for="(m, i) in messages.filter(m => m.role === 'user').slice(-10)" :key="i" class="trail-item" @click="sendMessage(m.text)">
            <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="9 18 15 12 9 6"/></svg>
            <span class="trail-text">{{ m.text }}</span>
          </div>
        </div>
      </template>
    </aside>
  </div>
</div>
</template>

<style scoped>
*{box-sizing:border-box}
.cb,.loader{font-family:-apple-system,BlinkMacSystemFont,'SF Pro Text','SF Pro Display','Helvetica Neue',sans-serif;-webkit-font-smoothing:antialiased;-moz-osx-font-smoothing:grayscale}

.loader{position:fixed;inset:0;display:flex;align-items:center;justify-content:center;background:var(--vp-c-bg);z-index:999}
.loader-inner{text-align:center;animation:loaderIn .6s ease}
.loader-icon{margin:0 auto 20px;color:var(--vp-c-brand-1);animation:pulse 2s infinite}
.loader h2{font-size:20px;font-weight:600;margin:0 0 6px;letter-spacing:-.03em;color:var(--vp-c-text-1)}
.loader p{font-size:13px;color:var(--vp-c-text-3);margin:0 0 20px}
.loader-bar{width:180px;height:3px;background:var(--vp-c-divider);border-radius:2px;overflow:hidden;margin:0 auto}
.loader-fill{height:100%;background:var(--vp-c-brand-1);border-radius:2px;transition:width .3s}
.loader-pct{font-size:11px;color:var(--vp-c-text-3);margin-top:8px;display:block;font-variant-numeric:tabular-nums}

.cb{display:flex;flex-direction:column;height:100vh;max-height:100vh;overflow:hidden;background:var(--vp-c-bg)}

.cb-hd{display:flex;align-items:center;justify-content:space-between;padding:8px 16px;border-bottom:1px solid var(--vp-c-divider);flex-shrink:0;gap:12px}
.cb-hd-l{display:flex;align-items:center;gap:10px}
.cb-logo{width:32px;height:32px;display:flex;align-items:center;justify-content:center;background:var(--vp-c-brand-1);border-radius:8px}
.cb-title{font-size:14px;font-weight:600;margin:0;letter-spacing:-.02em;color:var(--vp-c-text-1)}
.cb-sub{font-size:11px;color:var(--vp-c-text-3);margin:0}

.cb-pills{display:flex;gap:6px;align-items:center}
.pill{display:flex;align-items:center;gap:5px;padding:3px 8px;background:var(--vp-c-bg-soft);border:1px solid var(--vp-c-divider);border-radius:6px;font-size:11px}
.pill-lang{cursor:pointer;font-weight:600;transition:background .15s}
.pill-lang:hover{background:var(--vp-c-brand-soft)}
.pill-l{color:var(--vp-c-text-3);font-weight:500}
.pill-v{color:var(--vp-c-text-1);font-weight:600;font-variant-numeric:tabular-nums}

.cb-body{display:flex;flex:1;overflow:hidden}

.cb-side{width:190px;flex-shrink:0;padding:14px;overflow-y:auto;display:flex;flex-direction:column;background:var(--vp-c-bg-soft)}
.cb-side-l{border-right:1px solid var(--vp-c-divider)}
.cb-side-r{border-left:1px solid var(--vp-c-divider)}
.side-hd{font-size:10px;font-weight:600;text-transform:uppercase;letter-spacing:.1em;color:var(--vp-c-text-3);margin-bottom:10px}

.area-list{display:flex;flex-direction:column;gap:1px}
.area-item{display:flex;align-items:center;gap:8px;padding:4px 6px;border-radius:4px;font-size:11.5px;color:var(--vp-c-text-3);transition:color .2s}
.area-item.visited{color:var(--vp-c-text-1);font-weight:500}
.area-dot{width:6px;height:6px;border-radius:50%;background:var(--vp-c-divider);flex-shrink:0;transition:all .3s}
.area-dot.on{background:var(--vp-c-brand-1);box-shadow:0 0 6px rgba(59,130,246,.4)}
.area-name{text-transform:capitalize}

.side-links{margin-top:auto;padding-top:12px;border-top:1px solid var(--vp-c-divider);display:flex;flex-direction:column;gap:6px}
.side-link{font-size:11px;color:var(--vp-c-text-3);text-decoration:none;display:flex;align-items:center;gap:6px;transition:color .15s}
.side-link:hover{color:var(--vp-c-brand-1)}

.ss{display:flex;flex-direction:column;gap:6px}
.ss-row{display:flex;justify-content:space-between;font-size:11.5px;color:var(--vp-c-text-3)}
.ss-val{font-weight:600;color:var(--vp-c-text-1);font-variant-numeric:tabular-nums}

.trail{display:flex;flex-direction:column;gap:1px}
.trail-item{font-size:11px;color:var(--vp-c-text-3);padding:4px 6px;border-radius:4px;cursor:pointer;transition:all .15s;display:flex;align-items:flex-start;gap:4px}
.trail-item:hover{background:var(--vp-c-bg);color:var(--vp-c-text-1)}
.trail-item svg{flex-shrink:0;opacity:.3;margin-top:2px}
.trail-text{line-height:1.35;display:-webkit-box;-webkit-line-clamp:2;-webkit-box-orient:vertical;overflow:hidden}

.cb-center{flex:1;display:flex;flex-direction:column;overflow:hidden;min-width:0}

.welcome{display:flex;flex-direction:column;align-items:center;justify-content:center;flex:1;text-align:center;gap:10px;animation:fadeIn .6s ease}
.w-shield{color:var(--vp-c-brand-1);opacity:.12}
.welcome h2{font-size:20px;font-weight:600;margin:0;letter-spacing:-.03em;color:var(--vp-c-text-1)}
.welcome p{color:var(--vp-c-text-3);font-size:13px;margin:0}
.suggestions{display:flex;flex-wrap:wrap;gap:6px;justify-content:center;max-width:520px;margin-top:6px}
.sug-btn{padding:7px 14px;border:1px solid var(--vp-c-divider);border-radius:18px;background:var(--vp-c-bg);color:var(--vp-c-text-2);font-size:13px;cursor:pointer;transition:all .15s}
.sug-btn:hover{border-color:var(--vp-c-brand-1);color:var(--vp-c-text-1);background:var(--vp-c-bg-soft)}

@keyframes fadeIn{from{opacity:0;transform:translateY(6px)}to{opacity:1;transform:translateY(0)}}
@keyframes loaderIn{from{opacity:0;transform:scale(.97)}to{opacity:1;transform:scale(1)}}
@keyframes pulse{0%,100%{opacity:.6}50%{opacity:1}}
@media(max-width:900px){.cb-side{display:none}.cb-pills .pill:not(.pill-lang){display:none}}
</style>
