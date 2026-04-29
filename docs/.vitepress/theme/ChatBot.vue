<script setup>
import { ref, onMounted, nextTick, computed } from 'vue'

const dataset = ref([])
const messages = ref([])
const input = ref('')
const isLoading = ref(false)
const inputEl = ref(null)
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

function tokenize(t) {
  return t.toLowerCase().replace(/[''`]/g,"'").replace(/[^a-z0-9àèéìòùäöüß\s']/g,' ').split(/\s+/).filter(t=>t.length>1)
}

function isItalian(text) {
  const it = ['serve','cosa','come','quali','siamo','nostri','possiamo','rientra','dobbiamo','quanto','NIS2','azienda','cifrat','incidente','fornitor','sicurezza','password','backup','firewall','audit']
  const low = text.toLowerCase()
  return it.filter(w => low.includes(w)).length >= 2 || /[àèéìòù]/.test(low)
}

function bm25Search(query, topN = 5) {
  const qT = tokenize(query)
  if (!qT.length) return []
  const N = dataset.value.length, k1 = 1.5, b = 0.75
  const avgDl = dataset.value.reduce((s,d) => s + tokenize(d.q).length, 0) / N
  const df = {}
  for (const t of qT) df[t] = dataset.value.filter(d => tokenize(d.q).includes(t)).length
  return dataset.value.map((doc, idx) => {
    const dT = tokenize(doc.q), dl = dT.length
    let score = 0
    for (const t of qT) {
      const tf = dT.filter(dt => dt === t).length
      const idf = Math.log((N - (df[t]||0) + 0.5) / ((df[t]||0) + 0.5) + 1)
      score += idf * (tf * (k1+1)) / (tf + k1*(1-b+b*dl/avgDl))
    }
    return { score, idx }
  }).sort((a,b) => b.score - a.score).slice(0, topN).filter(s => s.score > 0)
}

function findFollowUps(category, lang) {
  // Find entries in the same or related categories matching the detected language
  const baseCat = category.replace(/_impl$/, '')
  const related = [category, baseCat, baseCat + '_impl']
  const candidates = dataset.value.filter(d => {
    const dCat = d.c.replace(/_impl$/, '')
    return related.includes(d.c) || related.includes(dCat)
  })
  // Filter by language
  const langFiltered = candidates.filter(d => lang === 'it' ? isItalian(d.q) : !isItalian(d.q))
  // If not enough in target language, fall back to any language
  const pool = langFiltered.length >= 3 ? langFiltered : candidates
  // Deduplicate and limit
  const seen = new Set()
  const result = []
  for (const d of pool) {
    if (!seen.has(d.q) && result.length < 4) {
      seen.add(d.q)
      result.push(d.q)
    }
  }
  return result
}

function formatAnswer(text) {
  // Parse article references into badges
  let html = text.replace(/Art\.\s*(\d+)(\([^)]+\))*/g, '<span class="art-badge">Art. $1$2</span>')
  // Parse numbered items: (1) ... (2) ... or 1. ... 2. ...
  html = html.replace(/\((\d+)\)\s/g, '<br><span class="step-num">$1</span> ')
  html = html.replace(/(\d+)\.\s(?=[A-Z])/g, '<br><span class="step-num">$1</span> ')
  // Parse colons followed by lists
  html = html.replace(/:\s*([A-Z])/g, ':<br>$1')
  return html
}

function search(query) {
  const t0 = performance.now()
  const results = bm25Search(query, 3)
  const elapsed = performance.now() - t0
  const lang = isItalian(query) ? 'it' : 'en'

  if (results.length > 0 && results[0].score > 0.5) {
    const entry = dataset.value[results[0].idx]
    const cat = entry.c.replace(/_impl$/, '')
    visitedCategories.value = new Set([...visitedCategories.value, cat])
    stats.value.hits++
    const followUps = findFollowUps(entry.c, lang)
    return { hit: true, answer: entry.a, html: formatAnswer(entry.a), category: entry.c, score: results[0].score, followUps, elapsed }
  }

  const missSuggestions = lang === 'it'
    ? ['La mia azienda rientra nella NIS2?','Quali sono gli obblighi NIS2?','Da dove iniziare con NIS2?','Quanto costa adeguarsi?']
    : ['What are the NIS2 obligations?','Do we need a risk assessment?','What are the NIS2 sanctions?','When does NIS2 come into effect?']
  return { hit: false, elapsed, followUps: missSuggestions }
}

async function sendMessage(text) {
  const query = (text || input.value).trim()
  if (!query || isLoading.value) return
  input.value = ''
  messages.value.push({ role: 'user', text: query })
  isLoading.value = true
  stats.value.queries++
  await nextTick(); scrollToBottom()
  await new Promise(r => setTimeout(r, 60))

  const result = search(query)
  totalMs.value += result.elapsed
  stats.value.avgMs = totalMs.value / stats.value.queries

  if (result.hit) {
    const msg = { role:'assistant', text:result.answer, html:result.html, category:result.category, score:result.score, elapsed:result.elapsed, followUps:result.followUps, typing:true, displayHtml:'' }
    messages.value.push(msg)
    // Typewriter
    const words = result.answer.split(' ')
    for (let i = 0; i < words.length; i++) {
      msg.displayHtml = formatAnswer(words.slice(0,i+1).join(' '))
      if (i % 3 === 0) { await new Promise(r=>setTimeout(r,10)); scrollToBottom() }
    }
    msg.typing = false; msg.displayHtml = result.html
  } else {
    messages.value.push({ role:'assistant', text:'Domanda non trovata. Prova a riformulare o usa uno dei suggerimenti.', html:'Domanda non trovata. Prova a riformulare o usa uno dei suggerimenti.', category:'miss', elapsed:result.elapsed, followUps:result.followUps, typing:false, displayHtml:'' })
  }
  isLoading.value = false
  await nextTick(); scrollToBottom()
}

function scrollToBottom() { if (chatEl.value) chatEl.value.scrollTop = chatEl.value.scrollHeight }

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
  inputEl.value?.focus()
})
</script>

<template>
<div class="cb">
  <!-- Header -->
  <header class="cb-hd">
    <div class="cb-hd-l">
      <div class="cb-logo">⚡</div>
      <div>
        <h1 class="cb-title">NIS2 Compliance Engine</h1>
        <p class="cb-sub">{{ stats.entries }} entries · deterministic · zero LLM</p>
      </div>
    </div>
    <div class="cb-pills">
      <div class="pill">
        <span class="pill-l">Coverage</span>
        <span class="pill-v">{{ coverage.explored }}/{{ coverage.total }}</span>
        <div class="cbar"><div class="cfill" :style="{width:(coverage.explored/coverage.total*100)+'%'}"></div></div>
      </div>
      <div class="pill" v-if="stats.queries>0">
        <span class="pill-l">Avg</span>
        <span class="pill-v">{{ stats.avgMs.toFixed(1) }}ms</span>
      </div>
      <div class="pill" v-if="stats.queries>0">
        <span class="pill-l">Hit</span>
        <span class="pill-v">{{ Math.round(stats.hits/stats.queries*100) }}%</span>
      </div>
    </div>
  </header>

  <!-- Chat -->
  <div class="cb-chat" ref="chatEl">
    <div v-if="messages.length===0" class="welcome">
      <div class="w-icon">🇪🇺</div>
      <h2>Ask anything about NIS2</h2>
      <p>462 pre-computed answers · IT/EN · Art. 20, 21, 23, 34</p>
      <div class="suggestions">
        <button v-for="s in suggestions" :key="s.text" @click="sendMessage(s.text)" class="sug-btn">
          <span class="sug-icon">{{ s.icon }}</span>{{ s.text }}
        </button>
      </div>
    </div>

    <div v-for="(m,i) in messages" :key="i" :class="['msg',m.role]">
      <div class="bubble">
        <div v-if="m.role==='assistant'&&m.category!=='miss'" class="meta">
          <span class="m-cat">{{ m.category.replace(/_/g,' ') }}</span>
          <span class="m-score">{{ m.score?.toFixed(2) }}</span>
          <span class="m-ms">{{ m.elapsed?.toFixed(1) }}ms</span>
        </div>
        <div v-if="m.role==='user'" class="msg-txt">{{ m.text }}</div>
        <div v-else class="msg-rich" v-html="m.typing ? m.displayHtml : m.html"></div>
        <span v-if="m.typing" class="cursor">▋</span>
        <div v-if="m.followUps?.length && !m.typing" class="fups">
          <button v-for="f in m.followUps" :key="f" @click="sendMessage(f)" class="fup-btn">→ {{ f }}</button>
        </div>
      </div>
    </div>

    <div v-if="isLoading" class="msg assistant"><div class="bubble"><div class="dots"><span/><span/><span/></div></div></div>
  </div>

  <!-- Input -->
  <div class="cb-in">
    <input ref="inputEl" v-model="input" @keyup.enter="sendMessage()" placeholder="Ask about NIS2..." :disabled="isLoading||!stats.entries" autocomplete="off"/>
    <button @click="sendMessage()" :disabled="isLoading||!input.trim()" class="send">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 2L11 13M22 2l-7 20-4-9-9-4 20-7z"/></svg>
    </button>
  </div>
</div>
</template>

<style scoped>
.cb{display:flex;flex-direction:column;height:100vh;max-height:100vh;overflow:hidden;background:var(--vp-c-bg);font-family:var(--vp-font-family-base)}

.cb-hd{display:flex;align-items:center;justify-content:space-between;padding:12px 20px;background:var(--vp-c-bg-soft);border-bottom:1px solid var(--vp-c-divider);flex-shrink:0;gap:16px;flex-wrap:wrap}
.cb-hd-l{display:flex;align-items:center;gap:12px}
.cb-logo{font-size:28px;width:44px;height:44px;display:flex;align-items:center;justify-content:center;background:linear-gradient(135deg,var(--vp-c-brand-1),var(--vp-c-brand-3));border-radius:12px;box-shadow:0 4px 12px rgba(26,86,219,0.3)}
.cb-title{font-size:17px;font-weight:700;margin:0;background:linear-gradient(135deg,var(--vp-c-brand-1),#7c3aed);-webkit-background-clip:text;-webkit-text-fill-color:transparent;background-clip:text}
.cb-sub{font-size:12px;color:var(--vp-c-text-2);margin:2px 0 0}
.cb-pills{display:flex;gap:8px;flex-wrap:wrap}
.pill{display:flex;align-items:center;gap:6px;padding:4px 10px;background:var(--vp-c-bg);border:1px solid var(--vp-c-divider);border-radius:20px;font-size:12px}
.pill-l{color:var(--vp-c-text-3);font-weight:500}.pill-v{color:var(--vp-c-brand-1);font-weight:700;font-variant-numeric:tabular-nums}
.cbar{width:48px;height:4px;background:var(--vp-c-divider);border-radius:2px;overflow:hidden}
.cfill{height:100%;background:linear-gradient(90deg,var(--vp-c-brand-1),#7c3aed);border-radius:2px;transition:width .4s}

.cb-chat{flex:1;overflow-y:auto;padding:20px;display:flex;flex-direction:column;gap:16px;scroll-behavior:smooth}
.cb-chat::-webkit-scrollbar{width:6px}.cb-chat::-webkit-scrollbar-thumb{background:var(--vp-c-divider);border-radius:3px}

.welcome{display:flex;flex-direction:column;align-items:center;justify-content:center;flex:1;text-align:center;gap:12px;animation:fadeIn .6s}
.w-icon{font-size:56px}
.welcome h2{font-size:24px;font-weight:700;margin:0;background:linear-gradient(135deg,var(--vp-c-brand-1),#7c3aed,#db2777);-webkit-background-clip:text;-webkit-text-fill-color:transparent;background-clip:text}
.welcome p{color:var(--vp-c-text-2);font-size:14px;margin:0}
.suggestions{display:flex;flex-wrap:wrap;gap:8px;justify-content:center;max-width:640px;margin-top:8px}
.sug-btn{padding:8px 16px;border:1px solid var(--vp-c-divider);border-radius:20px;background:var(--vp-c-bg-soft);color:var(--vp-c-text-1);font-size:13px;cursor:pointer;transition:all .2s;display:flex;align-items:center;gap:6px}
.sug-btn:hover{border-color:var(--vp-c-brand-1);background:var(--vp-c-brand-soft);transform:translateY(-1px)}
.sug-icon{font-size:16px}

.msg{display:flex}.msg.user{justify-content:flex-end}.msg.assistant{justify-content:flex-start}
.bubble{max-width:85%;padding:12px 16px;border-radius:16px;animation:slideIn .3s}
.msg.user .bubble{background:linear-gradient(135deg,var(--vp-c-brand-1),var(--vp-c-brand-2));color:#fff;border-bottom-right-radius:4px}
.msg.assistant .bubble{background:var(--vp-c-bg-soft);border:1px solid var(--vp-c-divider);border-bottom-left-radius:4px}

.meta{display:flex;gap:8px;margin-bottom:8px;flex-wrap:wrap}
.meta span{font-size:11px;padding:2px 8px;border-radius:10px;font-weight:600;font-variant-numeric:tabular-nums}
.m-cat{background:var(--vp-c-brand-soft);color:var(--vp-c-brand-1);text-transform:capitalize}
.m-score{background:rgba(124,58,237,0.1);color:#7c3aed}
.m-ms{background:rgba(16,185,129,0.1);color:#059669}
.dark .m-score{color:#a78bfa;background:rgba(124,58,237,0.15)}
.dark .m-ms{color:#34d399;background:rgba(16,185,129,0.15)}

.msg-txt{font-size:14px;line-height:1.6;white-space:pre-wrap;word-break:break-word}
.msg.user .msg-txt{color:#fff}
.msg-rich{font-size:14px;line-height:1.7;color:var(--vp-c-text-1);word-break:break-word}
.msg-rich :deep(.art-badge){display:inline-block;padding:1px 7px;border-radius:6px;font-size:12px;font-weight:700;background:var(--vp-c-brand-soft);color:var(--vp-c-brand-1);margin:0 2px;white-space:nowrap}
.msg-rich :deep(.step-num){display:inline-flex;align-items:center;justify-content:center;width:20px;height:20px;border-radius:50%;background:var(--vp-c-brand-1);color:#fff;font-size:11px;font-weight:700;margin-right:6px;flex-shrink:0}
.cursor{animation:blink .8s infinite;color:var(--vp-c-brand-1)}

.fups{display:flex;flex-direction:column;gap:4px;margin-top:10px;padding-top:10px;border-top:1px solid var(--vp-c-divider)}
.fup-btn{text-align:left;padding:6px 12px;border:none;border-radius:8px;background:transparent;color:var(--vp-c-brand-1);font-size:13px;cursor:pointer;transition:background .15s}
.fup-btn:hover{background:var(--vp-c-brand-soft)}

.dots{display:flex;gap:4px;padding:4px}
.dots span{width:8px;height:8px;border-radius:50%;background:var(--vp-c-text-3);animation:bounce 1.2s infinite ease-in-out}
.dots span:nth-child(2){animation-delay:.15s}.dots span:nth-child(3){animation-delay:.3s}

.cb-in{display:flex;gap:8px;padding:14px 20px;background:var(--vp-c-bg-soft);border-top:1px solid var(--vp-c-divider);flex-shrink:0}
.cb-in input{flex:1;padding:12px 18px;border:1px solid var(--vp-c-divider);border-radius:24px;background:var(--vp-c-bg);color:var(--vp-c-text-1);font-size:14px;outline:none;transition:border-color .2s}
.cb-in input:focus{border-color:var(--vp-c-brand-1);box-shadow:0 0 0 3px var(--vp-c-brand-soft)}
.cb-in input::placeholder{color:var(--vp-c-text-3)}
.send{width:44px;height:44px;border:none;border-radius:50%;background:linear-gradient(135deg,var(--vp-c-brand-1),#7c3aed);color:white;cursor:pointer;display:flex;align-items:center;justify-content:center;transition:transform .15s,opacity .15s;flex-shrink:0}
.send:hover:not(:disabled){transform:scale(1.05)}.send:disabled{opacity:.4;cursor:not-allowed}

@keyframes fadeIn{from{opacity:0;transform:translateY(12px)}to{opacity:1;transform:translateY(0)}}
@keyframes slideIn{from{opacity:0;transform:translateY(8px)}to{opacity:1;transform:translateY(0)}}
@keyframes blink{0%,100%{opacity:1}50%{opacity:0}}
@keyframes bounce{0%,60%,100%{transform:translateY(0)}30%{transform:translateY(-6px)}}
@media(max-width:640px){.cb-hd{padding:10px 14px}.cb-pills{display:none}.bubble{max-width:92%}.cb-in{padding:10px 14px}.welcome h2{font-size:20px}.suggestions{gap:6px}.sug-btn{font-size:12px;padding:6px 12px}}
</style>
