<script setup>
import { ref, onMounted, nextTick, computed } from 'vue'
import ChatWelcome from './ChatWelcome.vue'
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

const visitedCategories = ref(new Set())
const AREAS = [
  'governance','access_control','encryption','incident_response',
  'business_continuity','supply_chain','vulnerability_mgmt','risk_assessment',
  'network_security','detection','email_security','documentation',
  'remote_work','physical','legal','sanctions'
]
const coverage = computed(() => {
  const explored = AREAS.filter(a => visitedCategories.value.has(a)).length
  return { explored, total: AREAS.length }
})
const areaStatus = computed(() =>
  AREAS.map(a => ({ id: a, name: a.replace(/_/g, ' '), visited: visitedCategories.value.has(a) }))
)

function search(query) {
  const t0 = performance.now()
  const results = bm25Search(dataset.value, query, 3)
  const elapsed = performance.now() - t0
  const lang = isItalian(query) ? 'it' : 'en'

  if (results.length > 0 && results[0].score > 0.5) {
    const entry = dataset.value[results[0].idx]
    const cat = entry.c.replace(/_impl$/, '')
    visitedCategories.value = new Set([...visitedCategories.value, cat])
    stats.value.hits++
    return {
      hit: true, answer: entry.a, html: formatAnswer(entry.a),
      category: entry.c, score: results[0].score,
      followUps: findFollowUps(dataset.value, entry.c, lang), elapsed,
    }
  }
  const miss = lang === 'it'
    ? ['La mia azienda rientra nella NIS2?','Quali sono gli obblighi NIS2?','Da dove iniziare con NIS2?','Quanto costa adeguarsi?']
    : ['What are the NIS2 obligations?','Do we need a risk assessment?','What are the NIS2 sanctions?','When does NIS2 come into effect?']
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
  await new Promise(r => setTimeout(r, 60))

  const result = search(query)
  totalMs.value += result.elapsed
  stats.value.avgMs = totalMs.value / stats.value.queries

  if (result.hit) {
    const msg = { role:'assistant', text:result.answer, html:result.html, category:result.category, score:result.score, elapsed:result.elapsed, followUps:result.followUps, typing:true, displayHtml:'' }
    messages.value.push(msg)
    const words = result.answer.split(' ')
    for (let i = 0; i < words.length; i++) {
      msg.displayHtml = formatAnswer(words.slice(0, i + 1).join(' '))
      if (i % 3 === 0) { await new Promise(r => setTimeout(r, 10)); scrollBottom() }
    }
    msg.typing = false; msg.displayHtml = result.html
  } else {
    messages.value.push({ role:'assistant', text:'Domanda non trovata. Prova a riformulare.', html:'Domanda non trovata. Prova a riformulare.', category:'miss', elapsed:result.elapsed, followUps:result.followUps, typing:false, displayHtml:'' })
  }
  isLoading.value = false
  await nextTick(); scrollBottom()
}

function scrollBottom() {
  const el = chatEl.value?.$el || chatEl.value
  if (el) el.scrollTop = el.scrollHeight
}

const suggestions = [
  { text: 'La mia azienda rientra nella NIS2?', icon: '🏢' },
  { text: 'Da dove iniziare con NIS2?', icon: '🚀' },
  { text: 'Ci hanno hackerato, cosa facciamo?', icon: '🚨' },
  { text: 'What are the NIS2 obligations?', icon: '📋' },
  { text: 'Quanto costa adeguarsi a NIS2?', icon: '💰' },
  { text: 'Serve la crittografia?', icon: '🔐' },
]

onMounted(async () => {
  try {
    const base = import.meta.env.BASE_URL || '/'
    const res = await fetch(`${base}dataset.json`)
    dataset.value = await res.json()
    stats.value.entries = dataset.value.length
  } catch (e) { console.error('Dataset load failed:', e) }
})
</script>

<template>
<div class="cb">
  <header class="cb-hd">
    <div class="cb-hd-l">
      <div class="cb-logo">⚡</div>
      <div>
        <h1 class="cb-title">NIS2 Compliance Engine</h1>
        <p class="cb-sub">{{ stats.entries }} entries · deterministic · zero LLM ·
          <a href="https://github.com/fabriziosalmi/nis2-public" target="_blank" class="cb-link">nis2-public</a>
        </p>
      </div>
    </div>
    <div class="cb-pills">
      <div class="pill">
        <span class="pill-l">Coverage</span>
        <span class="pill-v">{{ coverage.explored }}/{{ coverage.total }}</span>
        <div class="cbar"><div class="cfill" :style="{width:(coverage.explored/coverage.total*100)+'%'}"></div></div>
      </div>
      <div class="pill" v-if="stats.queries > 0">
        <span class="pill-l">Avg</span>
        <span class="pill-v">{{ stats.avgMs.toFixed(1) }}ms</span>
      </div>
      <div class="pill" v-if="stats.queries > 0">
        <span class="pill-l">Hit</span>
        <span class="pill-v">{{ Math.round(stats.hits / stats.queries * 100) }}%</span>
      </div>
    </div>
  </header>

  <div class="cb-body">
    <!-- Left sidebar: coverage map -->
    <aside class="cb-side cb-side-l">
      <div class="side-title">NIS2 Art. 21 Coverage</div>
      <div class="area-list">
        <div v-for="a in areaStatus" :key="a.id" :class="['area-item', { visited: a.visited }]">
          <span class="area-dot">{{ a.visited ? '✓' : '○' }}</span>
          <span class="area-name">{{ a.name }}</span>
        </div>
      </div>
      <div class="side-footer">
        <a href="https://github.com/fabriziosalmi/nis2-public" target="_blank" class="side-link">
          📘 nis2-public
        </a>
        <a href="https://github.com/fabriziosalmi/nis2-model" target="_blank" class="side-link">
          ⚙️ nis2-model
        </a>
      </div>
    </aside>

    <!-- Center: chat -->
    <div class="cb-center">
      <ChatMessages ref="chatEl" :messages="messages" :isLoading="isLoading" @followUp="sendMessage">
        <template #welcome>
          <ChatWelcome v-if="messages.length === 0" :suggestions="suggestions" :entries="stats.entries" @select="sendMessage" />
        </template>
      </ChatMessages>
      <ChatInput v-model:input="input" :disabled="isLoading || !stats.entries" @send="sendMessage()" />
    </div>

    <!-- Right sidebar: session -->
    <aside class="cb-side cb-side-r">
      <div class="side-title">Session</div>
      <div class="session-stats">
        <div class="ss-row"><span>Queries</span><span class="ss-val">{{ stats.queries }}</span></div>
        <div class="ss-row"><span>Hits</span><span class="ss-val">{{ stats.hits }}</span></div>
        <div class="ss-row"><span>Areas</span><span class="ss-val">{{ coverage.explored }}/{{ coverage.total }}</span></div>
      </div>
      <div v-if="messages.length" class="side-title" style="margin-top:16px">Recent</div>
      <div class="trail">
        <div v-for="(m, i) in messages.filter(m => m.role === 'user').slice(-8)" :key="i" class="trail-item" @click="sendMessage(m.text)">
          {{ m.text.length > 40 ? m.text.slice(0,40) + '…' : m.text }}
        </div>
      </div>
    </aside>
  </div>
</div>
</template>

<style scoped>
.cb{display:flex;flex-direction:column;height:100vh;max-height:100vh;overflow:hidden;background:var(--vp-c-bg);font-family:var(--vp-font-family-base)}

.cb-hd{display:flex;align-items:center;justify-content:space-between;padding:10px 20px;background:var(--vp-c-bg-soft);border-bottom:1px solid var(--vp-c-divider);flex-shrink:0;gap:16px;flex-wrap:wrap}
.cb-hd-l{display:flex;align-items:center;gap:12px}
.cb-logo{font-size:24px;width:40px;height:40px;display:flex;align-items:center;justify-content:center;background:linear-gradient(135deg,var(--vp-c-brand-1),var(--vp-c-brand-3));border-radius:10px;box-shadow:0 4px 12px rgba(26,86,219,0.3)}
.cb-title{font-size:16px;font-weight:700;margin:0;background:linear-gradient(135deg,var(--vp-c-brand-1),#7c3aed);-webkit-background-clip:text;-webkit-text-fill-color:transparent;background-clip:text}
.cb-sub{font-size:11px;color:var(--vp-c-text-2);margin:2px 0 0}
.cb-link{color:var(--vp-c-brand-1);text-decoration:none;font-weight:600}
.cb-link:hover{text-decoration:underline}
.cb-pills{display:flex;gap:8px;flex-wrap:wrap}
.pill{display:flex;align-items:center;gap:6px;padding:4px 10px;background:var(--vp-c-bg);border:1px solid var(--vp-c-divider);border-radius:20px;font-size:12px}
.pill-l{color:var(--vp-c-text-3);font-weight:500}.pill-v{color:var(--vp-c-brand-1);font-weight:700;font-variant-numeric:tabular-nums}
.cbar{width:48px;height:4px;background:var(--vp-c-divider);border-radius:2px;overflow:hidden}
.cfill{height:100%;background:linear-gradient(90deg,var(--vp-c-brand-1),#7c3aed);border-radius:2px;transition:width .4s}

.cb-body{display:flex;flex:1;overflow:hidden}

/* Sidebars */
.cb-side{width:220px;flex-shrink:0;padding:16px;overflow-y:auto;border-color:var(--vp-c-divider);background:var(--vp-c-bg-soft)}
.cb-side-l{border-right:1px solid var(--vp-c-divider)}
.cb-side-r{border-left:1px solid var(--vp-c-divider)}
.side-title{font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.08em;color:var(--vp-c-text-3);margin-bottom:10px}

.area-list{display:flex;flex-direction:column;gap:4px}
.area-item{display:flex;align-items:center;gap:8px;padding:4px 8px;border-radius:6px;font-size:12px;color:var(--vp-c-text-2);transition:all .2s}
.area-item.visited{color:var(--vp-c-brand-1);font-weight:600}
.area-item.visited .area-dot{color:#059669}
.area-dot{font-size:12px;width:16px;text-align:center}
.area-name{text-transform:capitalize}

.side-footer{margin-top:auto;padding-top:16px;border-top:1px solid var(--vp-c-divider);display:flex;flex-direction:column;gap:6px}
.side-link{font-size:12px;color:var(--vp-c-text-2);text-decoration:none;transition:color .2s}
.side-link:hover{color:var(--vp-c-brand-1)}

.session-stats{display:flex;flex-direction:column;gap:6px}
.ss-row{display:flex;justify-content:space-between;font-size:12px;color:var(--vp-c-text-2)}
.ss-val{font-weight:700;color:var(--vp-c-brand-1);font-variant-numeric:tabular-nums}

.trail{display:flex;flex-direction:column;gap:4px}
.trail-item{font-size:12px;color:var(--vp-c-text-2);padding:4px 8px;border-radius:6px;cursor:pointer;transition:background .15s;white-space:nowrap;overflow:hidden;text-overflow:ellipsis}
.trail-item:hover{background:var(--vp-c-brand-soft);color:var(--vp-c-brand-1)}

/* Center */
.cb-center{flex:1;display:flex;flex-direction:column;overflow:hidden;min-width:0}

@media(max-width:900px){
  .cb-side{display:none}
  .cb-hd{padding:10px 14px}
  .cb-pills{display:none}
}
</style>
