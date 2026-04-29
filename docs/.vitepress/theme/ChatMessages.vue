<script setup>
defineProps({ messages: Array, isLoading: Boolean })
defineEmits(['followUp'])
</script>

<template>
<div class="cb-chat">
  <slot name="welcome" />

  <div v-for="(m, i) in messages" :key="i" :class="['msg', m.role]">
    <div class="bubble">
      <div v-if="m.role === 'assistant' && m.category && m.category !== 'miss'" class="meta">
        <span class="m-cat">{{ m.category.replace(/_/g, ' ') }}</span>
      </div>
      <div v-if="m.role === 'user'" class="msg-txt">{{ m.text }}</div>
      <div v-else class="msg-rich" v-html="m.typing ? m.displayHtml : m.html"></div>
      <span v-if="m.typing" class="cursor">▋</span>
      <div v-if="m.followUps?.length && !m.typing" class="fups">
        <button v-for="f in m.followUps" :key="f" @click="$emit('followUp', f)" class="fup-btn">
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="9 18 15 12 9 6"/></svg>
          {{ f }}
        </button>
      </div>
    </div>
  </div>

  <div v-if="isLoading" class="msg assistant">
    <div class="bubble"><div class="dots"><span/><span/><span/></div></div>
  </div>
</div>
</template>

<style scoped>
.cb-chat{flex:1;overflow-y:auto;padding:20px 24px;display:flex;flex-direction:column;gap:12px;scroll-behavior:smooth;font-family:-apple-system,BlinkMacSystemFont,'SF Pro Text',sans-serif}
.cb-chat::-webkit-scrollbar{width:5px}.cb-chat::-webkit-scrollbar-thumb{background:var(--vp-c-divider);border-radius:3px}

.msg{display:flex}.msg.user{justify-content:flex-end}.msg.assistant{justify-content:flex-start}
.bubble{max-width:72%;padding:10px 14px;border-radius:14px;animation:slideIn .25s ease}
.msg.user .bubble{background:var(--vp-c-brand-1);color:#fff;border-bottom-right-radius:4px}
.msg.assistant .bubble{background:var(--vp-c-bg-soft);border:1px solid var(--vp-c-divider);border-bottom-left-radius:4px}

.meta{margin-bottom:6px}
.m-cat{font-size:10px;padding:2px 7px;border-radius:4px;font-weight:600;text-transform:uppercase;letter-spacing:.06em;background:rgba(59,130,246,.08);color:var(--vp-c-brand-1)}
.dark .m-cat{background:rgba(59,130,246,.12)}

.msg-txt{font-size:14px;line-height:1.55;white-space:pre-wrap;word-break:break-word;letter-spacing:-.01em}
.msg.user .msg-txt{color:#fff}
.msg-rich{font-size:14px;line-height:1.7;color:var(--vp-c-text-1);word-break:break-word;letter-spacing:-.01em}
.msg-rich :deep(.art-badge){display:inline-block;padding:1px 6px;border-radius:4px;font-size:11px;font-weight:600;background:rgba(59,130,246,.08);color:var(--vp-c-brand-1);margin:0 1px;white-space:nowrap}
.msg-rich :deep(.step-num){display:inline-flex;align-items:center;justify-content:center;width:18px;height:18px;border-radius:50%;background:var(--vp-c-brand-1);color:#fff;font-size:10px;font-weight:700;margin-right:5px;flex-shrink:0}
.cursor{animation:blink .8s infinite;color:var(--vp-c-brand-1)}

.fups{display:flex;flex-direction:column;gap:2px;margin-top:8px;padding-top:8px;border-top:1px solid var(--vp-c-divider)}
.fup-btn{text-align:left;padding:5px 8px;border:none;border-radius:6px;background:transparent;color:var(--vp-c-text-2);font-size:12.5px;cursor:pointer;transition:all .15s;display:flex;align-items:center;gap:5px;letter-spacing:-.01em}
.fup-btn:hover{background:rgba(59,130,246,.06);color:var(--vp-c-brand-1)}
.fup-btn svg{opacity:.4;flex-shrink:0;transition:opacity .15s}
.fup-btn:hover svg{opacity:.8}

.dots{display:flex;gap:4px;padding:4px}
.dots span{width:6px;height:6px;border-radius:50%;background:var(--vp-c-text-3);animation:bounce 1.2s infinite ease-in-out}
.dots span:nth-child(2){animation-delay:.15s}.dots span:nth-child(3){animation-delay:.3s}

@keyframes slideIn{from{opacity:0;transform:translateY(4px)}to{opacity:1;transform:translateY(0)}}
@keyframes blink{0%,100%{opacity:1}50%{opacity:0}}
@keyframes bounce{0%,60%,100%{transform:translateY(0)}30%{transform:translateY(-4px)}}
</style>
