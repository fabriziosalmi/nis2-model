<script setup>
import { computed } from 'vue'
import { getStrings } from './i18n.js'

const props = defineProps({ messages: Array, isLoading: Boolean, lang: { type: String, default: 'en' } })
defineEmits(['followUp'])

const t = computed(() => getStrings(props.lang))
</script>

<template>
<div class="cb-chat">
  <slot name="welcome" />

  <div v-for="(m, i) in messages" :key="i" :class="['msg', m.role]">

    <!-- USER -->
    <div v-if="m.role === 'user'" class="bubble bubble-user">
      <div class="msg-txt">{{ m.text }}</div>
    </div>

    <!-- ASSISTANT — column layout: tab + card -->
    <div v-else class="msg-col">
      <!-- Category tab (sits above bubble) -->
      <div v-if="m.category && m.category !== 'miss'" class="cat-tab">
        <span :class="['sev-dot', m.severity || 'info']"></span>
        <span>{{ m.category }}</span>
      </div>

      <!-- Card body -->
      <div class="bubble bubble-assistant" :class="{ 'has-tab': m.category && m.category !== 'miss' }">
        <!-- Main answer -->
        <div :class="['section-answer', { 'fade-up': !m.typing }]">
          <div class="msg-rich" v-html="m.typing ? m.displayHtml : m.html"></div>
          <span v-if="m.typing" class="cursor-blink">▋</span>
        </div>

        <!-- Follow-up suggestions -->
        <div v-if="m.followUps?.length && !m.typing" class="section-followups">
          <div class="section-label">{{ t.sectionLabels?.followUps || 'Related questions' }}</div>
          <div class="fups-list">
            <button v-for="f in m.followUps" :key="f" @click="$emit('followUp', f)" class="fup-btn">
              <svg class="fup-chevron" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="9 18 15 12 9 6"/></svg>
              <span>{{ f }}</span>
              <svg class="fup-arrow" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><line x1="5" y1="12" x2="19" y2="12"/><polyline points="12 5 19 12 12 19"/></svg>
            </button>
          </div>
        </div>

        <!-- References & Standards -->
        <div v-if="!m.typing && (m.refs?.length || m.catLink || m.standards?.length)" class="section-refs">
          <div class="section-label">{{ t.sectionLabels?.references || 'References' }}</div>
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

        <!-- Glossary -->
        <div v-if="m.glossary?.length && !m.typing" class="section-glossary">
          <div class="section-label">{{ t.sectionLabels?.glossary || 'Glossary' }}</div>
          <div class="gloss-grid">
            <div class="gloss-row" v-for="g in m.glossary" :key="g.term">
              <span class="gloss-term">{{ g.term }}</span>
              <span class="gloss-def">{{ g.def }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>

  </div>

  <div v-if="isLoading" class="msg assistant">
    <div class="msg-col"><div class="bubble bubble-assistant"><div class="dots"><span/><span/><span/></div></div></div>
  </div>
</div>
</template>

<style scoped>
/* ══════════════════════════════════════════════
   DESIGN SYSTEM — 8pt Grid · WCAG AAA · Gestalt
   ══════════════════════════════════════════════ */

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

/* ── Column wrapper (tab + card) ── */
.msg-col{display:flex;flex-direction:column;max-width:640px;animation:slideIn .2s ease-out}

/* ── BUBBLE ── */
.bubble{overflow:hidden}
.bubble-user{
  max-width:640px;
  background:#1e40af;color:#e2e8f0;
  border-radius:16px 16px 4px 16px;
  padding:12px 20px;
  animation:slideIn .2s ease-out;
}
.bubble-assistant{
  background:var(--vp-c-bg-soft);
  border:1px solid rgba(255,255,255,.06);
  border-radius:16px;
  box-shadow:0 1px 3px rgba(0,0,0,.08);
}
/* When tab is present, card gets square top-left to connect */
.bubble-assistant.has-tab{
  border-top-left-radius:0;
  border-radius:0 16px 16px 4px;
}

/* ── CATEGORY TAB — physical folder tab ── */
.cat-tab{
  display:inline-flex;align-items:center;gap:6px;align-self:flex-start;
  padding:5px 14px 5px 10px;
  font-size:10px;font-weight:700;text-transform:uppercase;letter-spacing:.1em;
  color:#94a3b8;
  background:var(--vp-c-bg-soft);
  border:1px solid rgba(255,255,255,.06);
  border-bottom:1px solid var(--vp-c-bg-soft); /* hides card top border */
  border-radius:8px 8px 0 0;
  margin-bottom:-1px; /* overlap card top border */
  position:relative;z-index:1;
}
.sev-dot{width:6px;height:6px;border-radius:50%;flex-shrink:0}
.sev-dot.critical{background:#f87171;box-shadow:0 0 6px rgba(248,113,113,.5)}
.sev-dot.high{background:#fbbf24;box-shadow:0 0 5px rgba(251,191,36,.4)}
.sev-dot.medium{background:#a78bfa}
.sev-dot.info{background:#60a5fa}

/* ── SECTION LABELS ── */
.section-label{
  font-size:10px;font-weight:700;text-transform:uppercase;
  letter-spacing:.12em;color:var(--vp-c-text-3);
  margin-bottom:8px;opacity:.55;
}

/* ── MAIN ANSWER ── */
.section-answer{padding:16px 24px}
.msg-txt{font-size:15px;line-height:1.55;white-space:pre-wrap;word-break:break-word}
.msg.user .msg-txt,.bubble-user .msg-txt{color:#e2e8f0}
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
.cursor-blink{animation:blink .8s infinite;color:#60a5fa;display:block;padding-top:4px}

/* ── FOLLOW-UPS — full-width interactive ── */
.section-followups{padding:16px 24px;border-top:1px solid rgba(255,255,255,.06)}
.fups-list{display:flex;flex-direction:column;gap:2px}
.fup-btn{
  display:flex;align-items:center;gap:10px;width:100%;
  text-align:left;padding:10px 12px;border:none;border-radius:8px;
  background:transparent;color:#94a3b8;
  font-size:14px;cursor:pointer;transition:all .15s ease-out;
}
.fup-btn:hover{background:rgba(255,255,255,.05);color:#e2e8f0}
.fup-btn:active{background:rgba(255,255,255,.07)}
.fup-btn svg.fup-chevron{opacity:.3;flex-shrink:0;transition:all .15s ease-out}
.fup-btn:hover svg.fup-chevron{opacity:.7;transform:translateX(2px)}
.fup-btn span{flex:1;line-height:1.45}
.fup-btn .fup-arrow{opacity:0;flex-shrink:0;transition:all .15s ease-out;margin-left:auto}
.fup-btn:hover .fup-arrow{opacity:.4}

/* ── REFERENCES — Pill style ── */
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

/* ── GLOSSARY — CSS Grid ── */
.section-glossary{padding:16px 24px;border-top:1px solid rgba(255,255,255,.06)}
.gloss-grid{
  display:grid;grid-template-columns:minmax(100px,max-content) 1fr;
  gap:6px 16px;align-items:baseline;
}
.gloss-row{display:contents}
.gloss-term{
  font-weight:700;color:#93c5fd;white-space:nowrap;
  font-size:9px;text-transform:uppercase;letter-spacing:.1em;
  padding:3px 10px;border-radius:4px;
  background:rgba(96,165,250,.08);
  border:1px solid rgba(96,165,250,.12);
  justify-self:start;
}
.gloss-def{color:#94a3b8;font-size:12px;line-height:1.5}

/* ── LOADING ── */
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
  .bubble,.msg-col{max-width:95%}
  .section-answer,.section-followups,.section-refs,.section-glossary{padding-left:16px;padding-right:16px}
  .gloss-grid{grid-template-columns:1fr;gap:4px 0}
  .gloss-row{display:flex;flex-direction:column;gap:2px;margin-bottom:6px}
}
</style>
