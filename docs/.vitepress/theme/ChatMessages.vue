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

        <!-- Options grid (for wizard quick select) -->
        <div v-if="m.options?.length && !m.typing" class="section-options">
          <div class="options-grid">
            <button v-for="opt in m.options" :key="opt" @click="$emit('followUp', opt)" class="opt-btn">
              {{ opt }}
            </button>
          </div>
        </div>

        <!-- Follow-up suggestions -->
        <div v-if="m.followUps?.length && !m.typing" class="section-followups">
          <div class="section-label">{{ t.sectionLabels?.followUps || 'Related questions' }}</div>
          <div class="fups-list">
            <button v-for="f in m.followUps" :key="f" @click="$emit('followUp', f)" class="fup-btn" :title="t.tooltips?.followUp">
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
            <a v-for="r in (m.refs || [])" :key="r.num" :href="r.url" target="_blank" class="ref-link" :title="t.tooltips?.refLink + ' — Art. ' + r.num">
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/><polyline points="15 3 21 3 21 9"/><line x1="10" y1="14" x2="21" y2="3"/></svg>
              Art. {{ r.num }}
            </a>
            <a v-for="s in (m.standards || [])" :key="s.label" :href="s.url" target="_blank" class="std-chip">{{ s.label }}</a>
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
              <span class="gloss-term" :title="t.tooltips?.glossaryTerm">{{ g.term }}</span>
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
   Theme-aware via VitePress CSS custom properties
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
.cb-chat::-webkit-scrollbar-thumb{background:var(--vp-c-divider);border-radius:8px}
.cb-chat::-webkit-scrollbar-thumb:hover{background:var(--vp-c-text-3)}

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
  background:#1e40af;color:#f1f5f9;
  border-radius:16px 16px 4px 16px;
  padding:12px 20px;
  animation:slideIn .2s ease-out;
}
.bubble-assistant{
  background:var(--vp-c-bg-soft);
  border:1px solid var(--vp-c-divider);
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
  color:var(--vp-c-text-2);
  background:var(--vp-c-bg-soft);
  border:1px solid var(--vp-c-divider);
  border-bottom:1px solid var(--vp-c-bg-soft); /* hides card top border */
  border-radius:8px 8px 0 0;
  margin-bottom:-1px; /* overlap card top border */
  position:relative;z-index:1;
}
.sev-dot{width:6px;height:6px;border-radius:50%;flex-shrink:0}
.sev-dot.critical{background:#ef4444;box-shadow:0 0 6px rgba(239,68,68,.5)}
.sev-dot.high{background:#f59e0b;box-shadow:0 0 5px rgba(245,158,11,.4)}
.sev-dot.medium{background:#8b5cf6}
.sev-dot.info{background:var(--vp-c-brand-1)}

/* ── SECTION LABELS ── */
.section-label{
  font-size:10px;font-weight:700;text-transform:uppercase;
  letter-spacing:.12em;color:var(--vp-c-text-2);
  margin-bottom:8px;opacity:.7;
}

/* ── MAIN ANSWER ── */
.section-answer{padding:16px 24px}
.msg-txt{font-size:15px;line-height:1.55;white-space:pre-wrap;word-break:break-word}
.bubble-user .msg-txt{color:#f1f5f9}
.msg-rich{font-size:15px;line-height:1.72;color:var(--vp-c-text-1);word-break:break-word;max-width:640px}

.msg-rich :deep(.ans-header){display:flex;align-items:center;gap:8px;padding-bottom:8px}
.msg-rich :deep(.ans-art){
  font-size:12px;font-weight:700;letter-spacing:.04em;
  color:var(--vp-c-brand-2);background:var(--vp-c-brand-soft);
  padding:4px 12px;border-radius:6px;
}
.msg-rich :deep(.ans-intro){color:var(--vp-c-text-1);line-height:1.72;padding-bottom:8px}
.msg-rich :deep(.ans-body){color:var(--vp-c-text-1);line-height:1.72}

/* Steps */
.msg-rich :deep(.ans-steps){display:flex;flex-direction:column;gap:4px;padding-top:8px}
.msg-rich :deep(.ans-step){
  display:flex;align-items:flex-start;gap:10px;
  padding:6px 8px;border-radius:8px;transition:background .15s ease-out;
}
.msg-rich :deep(.ans-step:hover){background:var(--vp-c-bg-alt)}
.msg-rich :deep(.step-num){
  width:24px;height:24px;border-radius:6px;
  background:var(--vp-c-brand-1);color:#fff;font-size:12px;font-weight:700;
  display:flex;align-items:center;justify-content:center;flex-shrink:0;margin-top:2px;
}
.msg-rich :deep(.step-text){flex:1;line-height:1.68;font-size:15px;color:var(--vp-c-text-1)}
.msg-rich :deep(.art-ref){font-weight:600;color:var(--vp-c-brand-2);font-size:14px;white-space:nowrap}
.cursor-blink{animation:blink .8s infinite;color:var(--vp-c-brand-1);display:block;padding-top:4px}

/* ── FOLLOW-UPS — full-width interactive ── */
.section-followups{padding:16px 24px;border-top:1px solid var(--vp-c-divider)}
.fups-list{display:flex;flex-direction:column;gap:2px}
.fup-btn{
  display:flex;align-items:center;gap:10px;width:100%;
  text-align:left;padding:10px 12px;border:none;border-radius:8px;
  background:transparent;color:var(--vp-c-text-2);
  font-size:14px;cursor:pointer;transition:all .15s ease-out;
}
.fup-btn:hover{background:var(--vp-c-bg-alt);color:var(--vp-c-text-1)}
.fup-btn:active{background:var(--vp-c-bg-alt)}
.fup-btn svg.fup-chevron{opacity:.3;flex-shrink:0;transition:all .15s ease-out}
.fup-btn:hover svg.fup-chevron{opacity:.7;transform:translateX(2px)}
.fup-btn span{flex:1;line-height:1.45}
.fup-btn .fup-arrow{opacity:0;flex-shrink:0;transition:all .15s ease-out;margin-left:auto}
.fup-btn:hover .fup-arrow{opacity:.4}

/* ── REFERENCES — Pill style ── */
.section-refs{padding:16px 24px;border-top:1px solid var(--vp-c-divider)}
.refs-row{display:flex;flex-wrap:wrap;gap:6px}
.ref-link{
  display:inline-flex;align-items:center;gap:6px;
  padding:5px 14px;border-radius:9999px;
  font-size:12px;font-weight:500;color:var(--vp-c-text-2);text-decoration:none;
  background:var(--vp-c-bg-alt);transition:all .15s ease-out;
}
.ref-link:hover{color:var(--vp-c-text-1);background:var(--vp-c-bg-soft)}
.ref-link:active{background:var(--vp-c-bg-soft)}
.ref-link svg{flex-shrink:0;opacity:.5}
.ref-link:hover svg{opacity:.8}
.ref-cat{border-left:2px solid var(--vp-c-brand-1);border-radius:9999px}
.std-chip{
  font-size:11px;padding:4px 12px;border-radius:9999px;
  background:var(--vp-c-bg-alt);color:var(--vp-c-text-2);font-weight:500;white-space:nowrap;
  text-decoration:none;transition:all .15s ease-out;display:inline-block;
}
.std-chip:hover{color:var(--vp-c-text-1);background:var(--vp-c-bg-soft)}
.std-chip:active{background:var(--vp-c-bg-soft)}

/* ── GLOSSARY ── */
.section-glossary{padding:16px 24px;border-top:1px solid var(--vp-c-divider)}
.gloss-grid{
  display:flex;flex-direction:column;gap:10px;
}
.gloss-row{display:flex;align-items:flex-start;gap:12px}
.gloss-term{
  font-weight:700;color:var(--vp-c-brand-2);white-space:nowrap;
  font-size:9px;text-transform:uppercase;letter-spacing:.1em;
  padding:4px 10px;border-radius:4px;
  background:var(--vp-c-brand-soft);
  border:1px solid var(--vp-c-divider);
  flex-shrink:0;line-height:1.4;margin-top:1px;
}
.gloss-def{color:var(--vp-c-text-2);font-size:12px;line-height:1.5;flex:1;min-width:0}

/* ── LOADING ── */
.dots{display:flex;gap:6px;padding:16px 24px}
.dots span{width:6px;height:6px;border-radius:50%;background:var(--vp-c-text-3);animation:bounce 1.2s infinite ease-in-out}
.dots span:nth-child(2){animation-delay:.15s}
.dots span:nth-child(3){animation-delay:.3s}

/* ── ANIMATIONS ── */
@keyframes slideIn{from{opacity:0;transform:translateY(6px)}to{opacity:1;transform:translateY(0)}}
@keyframes blink{0%,100%{opacity:1}50%{opacity:0}}
@keyframes bounce{0%,60%,100%{transform:translateY(0)}30%{transform:translateY(-4px)}}
@keyframes fadeBlur{from{opacity:0;filter:blur(2px);transform:translateY(8px)}to{opacity:1;filter:blur(0);transform:translateY(0)}}
.fade-up{animation:fadeBlur .35s ease-out}

/* ── OPTIONS ── */
.section-options{
  padding:12px 24px;
  border-top:1px solid var(--vp-c-divider);
  background:rgba(59,130,246,.02);
}
.options-grid{
  display:flex;
  flex-wrap:wrap;
  gap:8px;
}
.opt-btn{
  padding:6px 14px;
  border:1px solid var(--vp-c-divider);
  border-radius:16px;
  background:var(--vp-c-bg-soft);
  color:var(--vp-c-text-1);
  font-size:13px;
  cursor:pointer;
  transition:all .15s ease-out;
}
.opt-btn:hover{
  border-color:var(--vp-c-brand-1);
  background:var(--vp-c-brand-soft);
  color:var(--vp-c-brand-1);
}

/* ── RESPONSIVE ── */
@media(max-width:768px){
  .cb-chat{padding:16px 16px 32px}
  .bubble,.msg-col{max-width:95%}
  .section-answer,.section-followups,.section-refs,.section-glossary{padding-left:16px;padding-right:16px}
  .gloss-row{flex-direction:column;gap:4px}
}
</style>
