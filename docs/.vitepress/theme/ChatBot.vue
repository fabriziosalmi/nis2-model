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

// Coverage tracking
const visitedCategories = ref(new Set())
const OBLIGATION_AREAS = [
  'governance','access_control','encryption','incident_response',
  'business_continuity','supply_chain','vulnerability_mgmt','risk_assessment',
  'network_security','detection','email_security','documentation',
  'remote_work','physical','legal','sanctions'
]
const coverage = computed(() => {
  const explored = OBLIGATION_AREAS.filter(a => visitedCategories.value.has(a)).length
  return { explored, total: OBLIGATION_AREAS.length }
})

// BM25-lite search
function tokenize(text) {
  return text.toLowerCase()
    .replace(/[''`]/g, "'")
    .replace(/[^a-z0-9àèéìòùäöüß\s']/g, ' ')
    .split(/\s+/)
    .filter(t => t.length > 1)
}

function bm25Search(query, topN = 5) {
  const qTokens = tokenize(query)
  if (!qTokens.length) return []

  const N = dataset.value.length
  const k1 = 1.5, b = 0.75
  const avgDl = dataset.value.reduce((s, d) => s + tokenize(d.q).length, 0) / N

  // IDF
  const df = {}
  for (const t of qTokens) {
    df[t] = dataset.value.filter(d => tokenize(d.q).includes(t)).length
  }

  const scored = dataset.value.map((doc, idx) => {
    const docTokens = tokenize(doc.q)
    const dl = docTokens.length
    let score = 0
    for (const t of qTokens) {
      const tf = docTokens.filter(dt => dt === t).length
      const idf = Math.log((N - (df[t] || 0) + 0.5) / ((df[t] || 0) + 0.5) + 1)
      score += idf * (tf * (k1 + 1)) / (tf + k1 * (1 - b + b * dl / avgDl))
    }
    return { score, idx }
  })

  return scored
    .sort((a, b) => b.score - a.score)
    .slice(0, topN)
    .filter(s => s.score > 0)
}

function search(query) {
  const t0 = performance.now()
  const results = bm25Search(query, 3)
  const elapsed = performance.now() - t0

  if (results.length > 0 && results[0].score > 0.5) {
    const entry = dataset.value[results[0].idx]
    const cat = entry.c.replace(/_impl$/, '')
    visitedCategories.value = new Set([...visitedCategories.value, cat])
    const charCount = entry.a.length
    const tokPerSec = Math.round((charCount / 4) / (elapsed / 1000))
    stats.value.hits++
    return {
      hit: true,
      answer: entry.a,
      category: entry.c,
      score: results[0].score,
      followUps: entry.f || [],
      elapsed,
      tokPerSec,
    }
  }
  return { hit: false, elapsed, tokPerSec: 0 }
}

async function sendMessage(text) {
  const query = (text || input.value).trim()
  if (!query || isLoading.value) return
  input.value = ''

  messages.value.push({ role: 'user', text: query })
  isLoading.value = true
  stats.value.queries++

  await nextTick()
  scrollToBottom()

  // Small delay for visual effect
  await new Promise(r => setTimeout(r, 80))

  const result = search(query)
  totalMs.value += result.elapsed
  stats.value.avgMs = totalMs.value / stats.value.queries

  if (result.hit) {
    messages.value.push({
      role: 'assistant',
      text: result.answer,
      category: result.category,
      score: result.score,
      elapsed: result.elapsed,
      tokPerSec: result.tokPerSec,
      followUps: result.followUps.slice(0, 4),
      typing: true,
    })
    // Typewriter effect
    await typewriter(messages.value.length - 1, result.answer)
  } else {
    messages.value.push({
      role: 'assistant',
      text: 'This question is not in the knowledge base. Try asking about NIS2 applicability, obligations, sanctions, or incident reporting.',
      category: 'miss',
      elapsed: result.elapsed,
      tokPerSec: 0,
      followUps: [
        'What are the NIS2 obligations?',
        'La mia azienda rientra nella NIS2?',
        'What are the NIS2 sanctions?',
        'Da dove iniziare con NIS2?',
      ],
      typing: false,
    })
  }

  isLoading.value = false
  await nextTick()
  scrollToBottom()
}

async function typewriter(idx, fullText) {
  const msg = messages.value[idx]
  msg.displayText = ''
  const words = fullText.split(' ')
  for (let i = 0; i < words.length; i++) {
    msg.displayText = words.slice(0, i + 1).join(' ')
    if (i % 3 === 0) {
      await new Promise(r => setTimeout(r, 12))
      scrollToBottom()
    }
  }
  msg.typing = false
  msg.displayText = fullText
}

function scrollToBottom() {
  if (chatEl.value) {
    chatEl.value.scrollTop = chatEl.value.scrollHeight
  }
}

function handleFollowUp(text) {
  sendMessage(text)
}

const suggestions = [
  'La mia azienda rientra nella NIS2?',
  'Da dove iniziare con NIS2?',
  'Ci hanno hackerato, cosa facciamo?',
  'What are the NIS2 obligations?',
  'Quanto costa adeguarsi a NIS2?',
  'Do we need encryption at rest?',
]

onMounted(async () => {
  try {
    const base = import.meta.env.BASE_URL || '/'
    const res = await fetch(`${base}dataset.json`)
    dataset.value = await res.json()
    stats.value.entries = dataset.value.length
  } catch (e) {
    console.error('Failed to load dataset:', e)
  }
  inputEl.value?.focus()
})
</script>

<template>
  <div class="chatbot-root">
    <!-- Header bar -->
    <div class="chatbot-header">
      <div class="header-left">
        <div class="header-icon">⚡</div>
        <div>
          <h1 class="header-title">NIS2 Compliance Engine</h1>
          <p class="header-sub">{{ stats.entries }} entries · BM25 search · deterministic · zero LLM</p>
        </div>
      </div>
      <div class="header-stats">
        <div class="stat-pill">
          <span class="stat-label">Coverage</span>
          <span class="stat-value">{{ coverage.explored }}/{{ coverage.total }}</span>
          <div class="coverage-bar">
            <div class="coverage-fill" :style="{ width: (coverage.explored / coverage.total * 100) + '%' }"></div>
          </div>
        </div>
        <div class="stat-pill" v-if="stats.queries > 0">
          <span class="stat-label">Avg</span>
          <span class="stat-value">{{ stats.avgMs.toFixed(1) }}ms</span>
        </div>
        <div class="stat-pill" v-if="stats.queries > 0">
          <span class="stat-label">Hit rate</span>
          <span class="stat-value">{{ Math.round(stats.hits / stats.queries * 100) }}%</span>
        </div>
      </div>
    </div>

    <!-- Chat area -->
    <div class="chatbot-messages" ref="chatEl">
      <!-- Welcome message -->
      <div v-if="messages.length === 0" class="welcome">
        <div class="welcome-icon">🇪🇺</div>
        <h2>Ask anything about NIS2</h2>
        <p>462 pre-computed answers · EN/IT/DE/FR/ES · Art. 20, 21, 23, 34</p>
        <div class="suggestions">
          <button v-for="s in suggestions" :key="s" @click="sendMessage(s)" class="suggestion-btn">
            {{ s }}
          </button>
        </div>
      </div>

      <!-- Messages -->
      <div v-for="(msg, i) in messages" :key="i" :class="['msg', msg.role]">
        <div class="msg-bubble">
          <div v-if="msg.role === 'assistant'" class="msg-meta">
            <span v-if="msg.category && msg.category !== 'miss'" class="msg-cat">{{ msg.category }}</span>
            <span v-if="msg.score" class="msg-score">score {{ msg.score.toFixed(2) }}</span>
            <span class="msg-time">{{ msg.elapsed?.toFixed(1) }}ms</span>
            <span v-if="msg.tokPerSec" class="msg-tps">{{ msg.tokPerSec.toLocaleString() }} tok/s</span>
          </div>
          <div class="msg-text">{{ msg.typing ? msg.displayText : msg.text }}<span v-if="msg.typing" class="cursor">▋</span></div>
          <div v-if="msg.followUps?.length && !msg.typing" class="follow-ups">
            <button v-for="fu in msg.followUps" :key="fu" @click="handleFollowUp(fu)" class="follow-btn">
              → {{ fu }}
            </button>
          </div>
        </div>
      </div>

      <div v-if="isLoading" class="msg assistant">
        <div class="msg-bubble">
          <div class="typing-indicator"><span></span><span></span><span></span></div>
        </div>
      </div>
    </div>

    <!-- Input -->
    <div class="chatbot-input">
      <input
        ref="inputEl"
        v-model="input"
        @keyup.enter="sendMessage()"
        placeholder="Ask about NIS2 compliance..."
        :disabled="isLoading || !stats.entries"
        autocomplete="off"
      />
      <button @click="sendMessage()" :disabled="isLoading || !input.trim()" class="send-btn">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M22 2L11 13M22 2l-7 20-4-9-9-4 20-7z"/>
        </svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
.chatbot-root {
  display: flex;
  flex-direction: column;
  height: 100vh;
  max-height: 100vh;
  overflow: hidden;
  background: var(--vp-c-bg);
  font-family: var(--vp-font-family-base);
}

/* Header */
.chatbot-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 20px;
  background: var(--vp-c-bg-soft);
  border-bottom: 1px solid var(--vp-c-divider);
  flex-shrink: 0;
  gap: 16px;
  flex-wrap: wrap;
}
.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}
.header-icon {
  font-size: 28px;
  width: 44px;
  height: 44px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--vp-c-brand-1), var(--vp-c-brand-3));
  border-radius: 12px;
  box-shadow: 0 4px 12px rgba(26, 86, 219, 0.3);
}
.header-title {
  font-size: 17px;
  font-weight: 700;
  margin: 0;
  background: linear-gradient(135deg, var(--vp-c-brand-1), #7c3aed);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}
.header-sub {
  font-size: 12px;
  color: var(--vp-c-text-2);
  margin: 2px 0 0;
}
.header-stats {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}
.stat-pill {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 10px;
  background: var(--vp-c-bg);
  border: 1px solid var(--vp-c-divider);
  border-radius: 20px;
  font-size: 12px;
}
.stat-label { color: var(--vp-c-text-3); font-weight: 500; }
.stat-value { color: var(--vp-c-brand-1); font-weight: 700; font-variant-numeric: tabular-nums; }
.coverage-bar {
  width: 48px;
  height: 4px;
  background: var(--vp-c-divider);
  border-radius: 2px;
  overflow: hidden;
}
.coverage-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--vp-c-brand-1), #7c3aed);
  border-radius: 2px;
  transition: width 0.4s ease;
}

/* Messages */
.chatbot-messages {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  scroll-behavior: smooth;
}
.chatbot-messages::-webkit-scrollbar { width: 6px; }
.chatbot-messages::-webkit-scrollbar-thumb {
  background: var(--vp-c-divider);
  border-radius: 3px;
}

/* Welcome */
.welcome {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  flex: 1;
  text-align: center;
  gap: 12px;
  animation: fadeIn 0.6s ease;
}
.welcome-icon { font-size: 56px; }
.welcome h2 {
  font-size: 24px;
  font-weight: 700;
  margin: 0;
  background: linear-gradient(135deg, var(--vp-c-brand-1), #7c3aed, #db2777);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}
.welcome p {
  color: var(--vp-c-text-2);
  font-size: 14px;
  margin: 0;
}
.suggestions {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  justify-content: center;
  max-width: 640px;
  margin-top: 8px;
}
.suggestion-btn {
  padding: 8px 16px;
  border: 1px solid var(--vp-c-divider);
  border-radius: 20px;
  background: var(--vp-c-bg-soft);
  color: var(--vp-c-text-1);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s ease;
}
.suggestion-btn:hover {
  border-color: var(--vp-c-brand-1);
  background: var(--vp-c-brand-soft);
  transform: translateY(-1px);
}

/* Message bubbles */
.msg { display: flex; }
.msg.user { justify-content: flex-end; }
.msg.assistant { justify-content: flex-start; }

.msg-bubble {
  max-width: 85%;
  padding: 12px 16px;
  border-radius: 16px;
  animation: slideIn 0.3s ease;
}
.msg.user .msg-bubble {
  background: linear-gradient(135deg, var(--vp-c-brand-1), var(--vp-c-brand-2));
  color: #fff;
  border-bottom-right-radius: 4px;
}
.msg.assistant .msg-bubble {
  background: var(--vp-c-bg-soft);
  border: 1px solid var(--vp-c-divider);
  border-bottom-left-radius: 4px;
}

.msg-meta {
  display: flex;
  gap: 8px;
  margin-bottom: 6px;
  flex-wrap: wrap;
}
.msg-meta span {
  font-size: 11px;
  padding: 1px 8px;
  border-radius: 10px;
  font-weight: 600;
  font-variant-numeric: tabular-nums;
}
.msg-cat {
  background: var(--vp-c-brand-soft);
  color: var(--vp-c-brand-1);
}
.msg-score {
  background: rgba(124, 58, 237, 0.1);
  color: #7c3aed;
}
.msg-time {
  background: rgba(16, 185, 129, 0.1);
  color: #059669;
}
.msg-tps {
  background: linear-gradient(90deg, rgba(219, 39, 119, 0.1), rgba(124, 58, 237, 0.1));
  color: #db2777;
}
.dark .msg-score { color: #a78bfa; background: rgba(124, 58, 237, 0.15); }
.dark .msg-time { color: #34d399; background: rgba(16, 185, 129, 0.15); }
.dark .msg-tps { color: #f472b6; }

.msg-text {
  font-size: 14px;
  line-height: 1.6;
  color: var(--vp-c-text-1);
  white-space: pre-wrap;
  word-break: break-word;
}
.msg.user .msg-text { color: #fff; }

.cursor {
  animation: blink 0.8s infinite;
  color: var(--vp-c-brand-1);
}

/* Follow-ups */
.follow-ups {
  display: flex;
  flex-direction: column;
  gap: 4px;
  margin-top: 10px;
  padding-top: 10px;
  border-top: 1px solid var(--vp-c-divider);
}
.follow-btn {
  text-align: left;
  padding: 6px 12px;
  border: none;
  border-radius: 8px;
  background: transparent;
  color: var(--vp-c-brand-1);
  font-size: 13px;
  cursor: pointer;
  transition: background 0.15s;
}
.follow-btn:hover {
  background: var(--vp-c-brand-soft);
}

/* Typing indicator */
.typing-indicator {
  display: flex;
  gap: 4px;
  padding: 4px;
}
.typing-indicator span {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--vp-c-text-3);
  animation: bounce 1.2s infinite ease-in-out;
}
.typing-indicator span:nth-child(2) { animation-delay: 0.15s; }
.typing-indicator span:nth-child(3) { animation-delay: 0.3s; }

/* Input area */
.chatbot-input {
  display: flex;
  gap: 8px;
  padding: 14px 20px;
  background: var(--vp-c-bg-soft);
  border-top: 1px solid var(--vp-c-divider);
  flex-shrink: 0;
}
.chatbot-input input {
  flex: 1;
  padding: 12px 18px;
  border: 1px solid var(--vp-c-divider);
  border-radius: 24px;
  background: var(--vp-c-bg);
  color: var(--vp-c-text-1);
  font-size: 14px;
  outline: none;
  transition: border-color 0.2s;
}
.chatbot-input input:focus {
  border-color: var(--vp-c-brand-1);
  box-shadow: 0 0 0 3px var(--vp-c-brand-soft);
}
.chatbot-input input::placeholder {
  color: var(--vp-c-text-3);
}
.send-btn {
  width: 44px;
  height: 44px;
  border: none;
  border-radius: 50%;
  background: linear-gradient(135deg, var(--vp-c-brand-1), #7c3aed);
  color: white;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: transform 0.15s, opacity 0.15s;
  flex-shrink: 0;
}
.send-btn:hover:not(:disabled) {
  transform: scale(1.05);
}
.send-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

@keyframes fadeIn { from { opacity: 0; transform: translateY(12px); } to { opacity: 1; transform: translateY(0); } }
@keyframes slideIn { from { opacity: 0; transform: translateY(8px); } to { opacity: 1; transform: translateY(0); } }
@keyframes blink { 0%, 100% { opacity: 1; } 50% { opacity: 0; } }
@keyframes bounce {
  0%, 60%, 100% { transform: translateY(0); }
  30% { transform: translateY(-6px); }
}

@media (max-width: 640px) {
  .chatbot-header { padding: 10px 14px; }
  .header-stats { display: none; }
  .msg-bubble { max-width: 92%; }
  .chatbot-input { padding: 10px 14px; }
  .welcome h2 { font-size: 20px; }
  .suggestions { gap: 6px; }
  .suggestion-btn { font-size: 12px; padding: 6px 12px; }
}
</style>
