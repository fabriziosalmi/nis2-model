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
              <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="9 18 15 12 9 6"/></svg>
              <span>{{ f }}</span>
            </button>
          </div>
        </div>

        <!-- Section 4: References & Standards -->
        <div v-if="!m.typing && (m.refs?.length || m.catLink || m.standards?.length)" class="section-refs">
          <div class="section-label">Riferimenti</div>
          <div class="refs-row">
            <a v-for="r in (m.refs || [])" :key="r.num" :href="r.url" target="_blank" class="ref-link">
              <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/><polyline points="15 3 21 3 21 9"/><line x1="10" y1="14" x2="21" y2="3"/></svg>
              Art. {{ r.num }}
            </a>
            <span v-for="s in (m.standards || [])" :key="s" class="std-chip">{{ s }}</span>
            <a v-if="m.catLink" :href="m.catLink.url" target="_blank" class="ref-link ref-cat">
              <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z"/><path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z"/></svg>
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
/* ── CHAT CONTAINER ── */
.cb-chat{
  flex:1;overflow-y:auto;padding:20px 28px 32px;
  display:flex;flex-direction:column;gap:16px;
  scroll-behavior:smooth;
  font-family:-apple-system,BlinkMacSystemFont,'SF Pro Text','Inter',sans-serif;
}
.cb-chat::-webkit-scrollbar{width:5px}
.cb-chat::-webkit-scrollbar-thumb{background:var(--vp-c-divider);border-radius:3px}

/* ── MESSAGE ROW ── */
.msg{display:flex;max-width:100%}
.msg.user{justify-content:flex-end}
.msg.assistant{justify-content:flex-start}

/* ── BUBBLE ── */
.bubble{
  max-width:680px;border-radius:16px;
  animation:slideIn .25s ease;overflow:hidden;
}
.msg.user .bubble{
  background:var(--vp-c-brand-1);color:#fff;
  border-bottom-right-radius:4px;padding:10px 18px;
}
.msg.assistant .bubble{
  background:var(--vp-c-bg-soft);
  border:1px solid var(--vp-c-divider);
  border-bottom-left-radius:4px;
  padding:0; /* sections handle their own padding */
}

/* ── SECTION LABELS ── */
.section-label{
  font-size:10.5px;font-weight:700;text-transform:uppercase;
  letter-spacing:.1em;color:var(--vp-c-text-3);
  padding:0 2px 5px;opacity:.6;
}

/* ── SECTION 1: META / CATEGORY ── */
.section-meta{
  padding:12px 18px 0;
  display:flex;align-items:center;gap:7px;flex-wrap:wrap;
}
.sev-dot{width:8px;height:8px;border-radius:50%;flex-shrink:0}
.sev-dot.critical{background:#ef4444;box-shadow:0 0 5px rgba(239,68,68,.4)}
.sev-dot.high{background:#f59e0b;box-shadow:0 0 4px rgba(245,158,11,.3)}
.sev-dot.medium{background:#6366f1}
.sev-dot.info{background:var(--vp-c-brand-1)}
.m-cat{
  font-size:11.5px;padding:2px 10px;border-radius:5px;
  font-weight:600;text-transform:uppercase;letter-spacing:.06em;
  background:rgba(59,130,246,.08);color:var(--vp-c-brand-1);
}
.m-deadline{
  font-size:11px;color:var(--vp-c-text-3);
  font-weight:500;letter-spacing:.02em;
}
.meta-spacer{flex:1}
.confidence-badge{
  font-size:10.5px;font-weight:700;padding:2px 8px;border-radius:4px;
  font-variant-numeric:tabular-nums;letter-spacing:.02em;
}
.conf-high{background:rgba(34,197,94,.08);color:rgba(34,197,94,.85)}
.conf-med{background:rgba(245,158,11,.08);color:rgba(245,158,11,.85)}
.conf-low{background:rgba(239,68,68,.08);color:rgba(239,68,68,.85)}

/* ── SECTION 2: MAIN ANSWER ── */
.section-answer{
  padding:10px 18px 14px;
}

/* User text */
.msg-txt{
  font-size:16px;line-height:1.5;white-space:pre-wrap;
  word-break:break-word;letter-spacing:-.01em;
}
.msg.user .msg-txt{color:#fff}

/* Rich answer content */
.msg-rich{
  font-size:15px;line-height:1.7;
  color:var(--vp-c-text-1);word-break:break-word;
  letter-spacing:-.005em;
}

/* Article header inside answer */
.msg-rich :deep(.ans-header){
  display:flex;align-items:center;gap:6px;
  padding-bottom:6px;
}
.msg-rich :deep(.ans-art){
  font-size:12.5px;font-weight:700;letter-spacing:.04em;
  color:var(--vp-c-brand-1);
  background:rgba(59,130,246,.06);
  padding:3px 11px;border-radius:6px;
  display:inline-block;
}

/* Intro text */
.msg-rich :deep(.ans-intro){
  color:var(--vp-c-text-1);line-height:1.7;
  padding-bottom:8px;
}

/* Body (no steps) */
.msg-rich :deep(.ans-body){
  color:var(--vp-c-text-1);line-height:1.7;
}

/* Steps */
.msg-rich :deep(.ans-steps){
  display:flex;flex-direction:column;gap:4px;
  padding-top:4px;
}
.msg-rich :deep(.ans-step){
  display:flex;align-items:flex-start;gap:8px;
  padding:5px 6px;border-radius:8px;
  transition:background .15s;
}
.msg-rich :deep(.ans-step:hover){
  background:rgba(59,130,246,.03);
}
.msg-rich :deep(.step-num){
  width:24px;height:24px;border-radius:6px;
  background:var(--vp-c-brand-1);color:#fff;
  font-size:12.5px;font-weight:700;
  display:flex;align-items:center;justify-content:center;
  flex-shrink:0;margin-top:2px;
}
.msg-rich :deep(.step-text){
  flex:1;line-height:1.65;font-size:15px;
  color:var(--vp-c-text-1);
}

/* Inline article refs */
.msg-rich :deep(.art-ref){
  font-weight:600;color:var(--vp-c-brand-1);
  font-size:14px;white-space:nowrap;
}

/* Cursor */
.cursor-blink{
  animation:blink .8s infinite;color:var(--vp-c-brand-1);
  display:block;padding-top:2px;
}

/* ── SECTION 3: FOLLOW-UPS ── */
.section-followups{
  padding:10px 18px 12px;
  border-top:1px solid var(--vp-c-divider);
}
.fups-list{
  display:flex;flex-direction:column;gap:1px;
}
.fup-btn{
  text-align:left;padding:6px 10px;border:none;border-radius:6px;
  background:transparent;color:var(--vp-c-text-2);
  font-size:14px;cursor:pointer;transition:all .15s;
  display:flex;align-items:center;gap:7px;letter-spacing:-.01em;
}
.fup-btn:hover{background:rgba(59,130,246,.05);color:var(--vp-c-brand-1)}
.fup-btn svg{opacity:.3;flex-shrink:0;transition:opacity .15s}
.fup-btn:hover svg{opacity:.7}
.fup-btn span{line-height:1.4}

/* ── SECTION 4: REFERENCES ── */
.section-refs{
  padding:10px 18px 12px;
  border-top:1px solid var(--vp-c-divider);
}
.refs-row{
  display:flex;flex-wrap:wrap;gap:5px;
}
.ref-link{
  display:inline-flex;align-items:center;gap:5px;
  padding:4px 10px;border-radius:5px;
  font-size:12px;font-weight:500;
  color:var(--vp-c-text-3);text-decoration:none;
  background:rgba(59,130,246,.04);
  transition:all .12s;letter-spacing:-.01em;
}
.ref-link:hover{color:var(--vp-c-brand-1);background:rgba(59,130,246,.08)}
.ref-link svg{flex-shrink:0;opacity:.4}
.ref-link:hover svg{opacity:.7}
.ref-cat{border-left:2px solid var(--vp-c-brand-1)}
.std-chip{
  font-size:11px;padding:3px 9px;border-radius:4px;
  background:rgba(99,102,241,.06);color:var(--vp-c-text-3);
  font-weight:500;letter-spacing:.01em;white-space:nowrap;
}

/* ── SECTION 5: GLOSSARY ── */
.section-glossary{
  padding:10px 18px 14px;
  border-top:1px solid var(--vp-c-divider);
}
.gloss-grid{
  display:flex;flex-direction:column;gap:3px;
}
.gloss-row{
  display:flex;align-items:baseline;gap:8px;
  font-size:12px;line-height:1.5;
}
.gloss-term{
  font-weight:700;color:var(--vp-c-text-2);white-space:nowrap;
  letter-spacing:.03em;flex-shrink:0;
  font-size:11px;
  padding:2px 6px;border-radius:3px;
  background:rgba(59,130,246,.05);
}
.gloss-def{color:var(--vp-c-text-3);font-size:12px;}

/* ── LOADING DOTS ── */
.dots{display:flex;gap:5px;padding:14px 18px}
.dots span{
  width:6px;height:6px;border-radius:50%;
  background:var(--vp-c-text-3);
  animation:bounce 1.2s infinite ease-in-out;
}
.dots span:nth-child(2){animation-delay:.15s}
.dots span:nth-child(3){animation-delay:.3s}

/* ── ANIMATIONS ── */
@keyframes slideIn{from{opacity:0;transform:translateY(4px)}to{opacity:1;transform:translateY(0)}}
@keyframes blink{0%,100%{opacity:1}50%{opacity:0}}
@keyframes bounce{0%,60%,100%{transform:translateY(0)}30%{transform:translateY(-4px)}}
@keyframes fadeBlur{from{opacity:0;filter:blur(2px);transform:translateY(6px)}to{opacity:1;filter:blur(0);transform:translateY(0)}}
.fade-up{animation:fadeBlur .4s ease-out}

/* ── RESPONSIVE ── */
@media(max-width:768px){
  .cb-chat{padding:14px 16px 24px}
  .bubble{max-width:95%}
  .section-answer,.section-followups,.section-refs,.section-glossary,.section-meta{
    padding-left:14px;padding-right:14px;
  }
}
</style>
