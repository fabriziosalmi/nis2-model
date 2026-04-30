<script setup>
defineProps({ messages: Array, isLoading: Boolean })
defineEmits(['followUp'])
</script>

<template>
<div class="cb-chat">
  <slot name="welcome" />

  <div v-for="(m, i) in messages" :key="i" :class="['msg', m.role]">
    <div class="bubble">

      <!-- ── USER MESSAGE ── -->
      <div v-if="m.role === 'user'" class="msg-txt">{{ m.text }}</div>

      <!-- ── ASSISTANT MESSAGE ── -->
      <template v-else>

        <!-- Section 1: Category badge + confidence -->
        <div v-if="m.category && m.category !== 'miss'" class="section-meta">
          <span :class="['sev-dot', m.severity || 'info']"></span>
          <span class="m-cat">{{ m.category }}</span>
          <span v-if="m.deadline" class="m-deadline">{{ m.deadline }}</span>
          <span class="meta-spacer"></span>
          <span v-if="m.confidence && !m.typing" class="confidence-badge" :class="m.confidence >= 80 ? 'conf-high' : m.confidence >= 50 ? 'conf-med' : 'conf-low'">
            {{ m.confidence }}% match
          </span>
        </div>

        <!-- Section 2: Main answer -->
        <div :class="['section-answer', { 'fade-up': !m.typing }]">
          <div class="msg-rich" v-html="m.typing ? m.displayHtml : m.html"></div>
          <span v-if="m.typing" class="cursor-blink">▋</span>
        </div>

        <!-- Section 3: Follow-up suggestions -->
        <div v-if="m.followUps?.length && !m.typing" class="section-followups">
          <div class="section-label">Domande correlate</div>
          <div class="fups-list">
            <button v-for="f in m.followUps" :key="f" @click="$emit('followUp', f)" class="fup-btn">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="9 18 15 12 9 6"/></svg>
              <span>{{ f }}</span>
            </button>
          </div>
        </div>

        <!-- Section 4: References & Standards -->
        <div v-if="!m.typing && (m.refs?.length || m.catLink || m.standards?.length)" class="section-refs">
          <div class="section-label">Riferimenti</div>
          <div class="refs-row">
            <a v-for="r in (m.refs || [])" :key="r.num" :href="r.url" target="_blank" class="ref-link">
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/><polyline points="15 3 21 3 21 9"/><line x1="10" y1="14" x2="21" y2="3"/></svg>
              Art. {{ r.num }}
            </a>
            <span v-for="s in (m.standards || [])" :key="s" class="std-chip">{{ s }}</span>
            <a v-if="m.catLink" :href="m.catLink.url" target="_blank" class="ref-link ref-cat">
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z"/><path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z"/></svg>
              {{ m.catLink.label }}
            </a>
          </div>
        </div>

        <!-- Section 5: Glossary -->
        <div v-if="m.glossary?.length && !m.typing" class="section-glossary">
          <div class="section-label">Glossario</div>
          <div class="gloss-grid">
            <div class="gloss-row" v-for="g in m.glossary" :key="g.term">
              <span class="gloss-term">{{ g.term }}</span>
              <span class="gloss-def">{{ g.def }}</span>
            </div>
          </div>
        </div>

      </template>
    </div>
  </div>

  <div v-if="isLoading" class="msg assistant">
    <div class="bubble"><div class="dots"><span/><span/><span/></div></div>
  </div>
</div>
</template>

<style scoped>
/* ══════════════════════════════════════════════
   DESIGN SYSTEM — 8pt Grid · WCAG AAA · Gestalt
   ══════════════════════════════════════════════ */

/* ── CHAT CONTAINER ── */
.cb-chat{
  flex:1;overflow-y:auto;padding:24px 32px 40px;
  display:flex;flex-direction:column;gap:16px;
  scroll-behavior:smooth;
  font-family:-apple-system,BlinkMacSystemFont,'SF Pro Text','Inter',system-ui,sans-serif;
  -webkit-font-smoothing:antialiased;
  -moz-osx-font-smoothing:grayscale;
}
.cb-chat::-webkit-scrollbar{width:6px}
.cb-chat::-webkit-scrollbar-track{background:transparent}
.cb-chat::-webkit-scrollbar-thumb{background:rgba(255,255,255,.08);border-radius:8px}
.cb-chat::-webkit-scrollbar-thumb:hover{background:rgba(255,255,255,.15)}

/* ── MESSAGE ROW ── */
.msg{display:flex;max-width:100%}
.msg.user{justify-content:flex-end}
.msg.assistant{justify-content:flex-start}

/* ── BUBBLE ── */
.bubble{max-width:640px;animation:slideIn .2s ease-out;overflow:hidden}

/* User bubble — institutional blue, asymmetric */
.msg.user .bubble{
  background:#1e40af;color:#e2e8f0;
  border-radius:16px 16px 4px 16px;
  padding:12px 20px;
}

/* Assistant bubble — elevated surface */
.msg.assistant .bubble{
  background:var(--vp-c-bg-soft);
  border:1px solid rgba(255,255,255,.06);
  border-radius:16px 16px 16px 4px;
  box-shadow:0 1px 3px rgba(0,0,0,.08);
}

/* ── SECTION LABELS ── */
.section-label{
  font-size:10px;font-weight:700;text-transform:uppercase;
  letter-spacing:.12em;color:var(--vp-c-text-3);
  margin-bottom:8px;opacity:.55;
}

/* ── SECTION 1: META ── */
.section-meta{padding:16px 24px 0;display:flex;align-items:center;gap:8px;flex-wrap:wrap}
.sev-dot{width:8px;height:8px;border-radius:50%;flex-shrink:0}
.sev-dot.critical{background:#ef4444;box-shadow:0 0 6px rgba(239,68,68,.5)}
.sev-dot.high{background:#f59e0b;box-shadow:0 0 5px rgba(245,158,11,.4)}
.sev-dot.medium{background:#818cf8}
.sev-dot.info{background:#60a5fa}
.m-cat{
  font-size:11px;padding:3px 10px;border-radius:6px;
  font-weight:700;text-transform:uppercase;letter-spacing:.08em;
  background:rgba(96,165,250,.1);color:#93c5fd;
}
.m-deadline{font-size:11px;color:var(--vp-c-text-3);font-weight:500}
.meta-spacer{flex:1}
.confidence-badge{
  font-size:10px;font-weight:700;padding:3px 8px;border-radius:6px;
  font-variant-numeric:tabular-nums;letter-spacing:.03em;
}
.conf-high{background:rgba(34,197,94,.1);color:#86efac}
.conf-med{background:rgba(251,191,36,.1);color:#fcd34d}
.conf-low{background:rgba(248,113,113,.1);color:#fca5a5}

/* ── SECTION 2: MAIN ANSWER ── */
.section-answer{padding:16px 24px}

/* User text */
.msg-txt{font-size:15px;line-height:1.55;white-space:pre-wrap;word-break:break-word}
.msg.user .msg-txt{color:#e2e8f0}

/* Rich answer — WCAG AAA contrast */
.msg-rich{font-size:15px;line-height:1.72;color:#e2e8f0;word-break:break-word;max-width:640px}

.msg-rich :deep(.ans-header){display:flex;align-items:center;gap:8px;padding-bottom:8px}
.msg-rich :deep(.ans-art){
  font-size:12px;font-weight:700;letter-spacing:.04em;
  color:#93c5fd;background:rgba(96,165,250,.08);
  padding:4px 12px;border-radius:6px;
}
.msg-rich :deep(.ans-intro){color:#e2e8f0;line-height:1.72;padding-bottom:8px}
.msg-rich :deep(.ans-body){color:#e2e8f0;line-height:1.72}

/* Steps */
.msg-rich :deep(.ans-steps){display:flex;flex-direction:column;gap:4px;padding-top:8px}
.msg-rich :deep(.ans-step){
  display:flex;align-items:flex-start;gap:10px;
  padding:6px 8px;border-radius:8px;transition:background .15s ease-out;
}
.msg-rich :deep(.ans-step:hover){background:rgba(255,255,255,.03)}
.msg-rich :deep(.step-num){
  width:24px;height:24px;border-radius:6px;
  background:#2563eb;color:#fff;font-size:12px;font-weight:700;
  display:flex;align-items:center;justify-content:center;flex-shrink:0;margin-top:2px;
}
.msg-rich :deep(.step-text){flex:1;line-height:1.68;font-size:15px;color:#e2e8f0}
.msg-rich :deep(.art-ref){font-weight:600;color:#93c5fd;font-size:14px;white-space:nowrap}

/* Cursor */
.cursor-blink{animation:blink .8s infinite;color:#60a5fa;display:block;padding-top:4px}

/* ── SECTION 3: FOLLOW-UPS ── */
.section-followups{padding:16px 24px;border-top:1px solid rgba(255,255,255,.06)}
.fups-list{display:flex;flex-direction:column;gap:2px}
.fup-btn{
  text-align:left;padding:8px 12px;border:none;border-radius:8px;
  background:transparent;color:#94a3b8;
  font-size:14px;cursor:pointer;transition:all .15s ease-out;
  display:flex;align-items:center;gap:8px;
}
.fup-btn:hover{background:rgba(255,255,255,.04);color:#e2e8f0}
.fup-btn:active{background:rgba(255,255,255,.06)}
.fup-btn svg{opacity:.4;flex-shrink:0;transition:all .15s ease-out}
.fup-btn:hover svg{opacity:.7;transform:translateX(2px)}
.fup-btn span{line-height:1.45}

/* ── SECTION 4: REFERENCES — Pill style, no borders ── */
.section-refs{padding:16px 24px;border-top:1px solid rgba(255,255,255,.06)}
.refs-row{display:flex;flex-wrap:wrap;gap:6px}
.ref-link{
  display:inline-flex;align-items:center;gap:6px;
  padding:5px 14px;border-radius:9999px;
  font-size:12px;font-weight:500;color:#94a3b8;text-decoration:none;
  background:rgba(255,255,255,.06);transition:all .15s ease-out;
}
.ref-link:hover{color:#e2e8f0;background:rgba(255,255,255,.1)}
.ref-link:active{background:rgba(255,255,255,.12)}
.ref-link svg{flex-shrink:0;opacity:.5}
.ref-link:hover svg{opacity:.8}
.ref-cat{border-left:2px solid #60a5fa;border-radius:9999px}
.std-chip{
  font-size:11px;padding:4px 12px;border-radius:9999px;
  background:rgba(255,255,255,.05);color:#94a3b8;font-weight:500;white-space:nowrap;
}

/* ── SECTION 5: GLOSSARY — CSS Grid 2-column ── */
.section-glossary{padding:16px 24px;border-top:1px solid rgba(255,255,255,.06)}
.gloss-grid{
  display:grid;grid-template-columns:minmax(100px,max-content) 1fr;
  gap:6px 16px;align-items:baseline;
}
.gloss-row{display:contents}
.gloss-term{
  font-weight:700;color:#93c5fd;white-space:nowrap;font-size:10px;
  padding:2px 8px;border-radius:4px;background:rgba(96,165,250,.08);justify-self:start;
}
.gloss-def{color:#94a3b8;font-size:12px;line-height:1.5}

/* ── LOADING DOTS ── */
.dots{display:flex;gap:6px;padding:16px 24px}
.dots span{width:6px;height:6px;border-radius:50%;background:#64748b;animation:bounce 1.2s infinite ease-in-out}
.dots span:nth-child(2){animation-delay:.15s}
.dots span:nth-child(3){animation-delay:.3s}

/* ── ANIMATIONS ── */
@keyframes slideIn{from{opacity:0;transform:translateY(6px)}to{opacity:1;transform:translateY(0)}}
@keyframes blink{0%,100%{opacity:1}50%{opacity:0}}
@keyframes bounce{0%,60%,100%{transform:translateY(0)}30%{transform:translateY(-4px)}}
@keyframes fadeBlur{from{opacity:0;filter:blur(2px);transform:translateY(8px)}to{opacity:1;filter:blur(0);transform:translateY(0)}}
.fade-up{animation:fadeBlur .35s ease-out}

/* ── RESPONSIVE ── */
@media(max-width:768px){
  .cb-chat{padding:16px 16px 32px}
  .bubble{max-width:95%}
  .section-answer,.section-followups,.section-refs,.section-glossary,.section-meta{padding-left:16px;padding-right:16px}
  .gloss-grid{grid-template-columns:1fr;gap:4px 0}
  .gloss-row{display:flex;flex-direction:column;gap:2px;margin-bottom:6px}
}
</style>
