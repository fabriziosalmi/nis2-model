<script setup>
defineProps({ messages: Array, isLoading: Boolean })
defineEmits(['followUp'])
</script>

<template>
<div class="cb-chat">
  <slot name="welcome" />

  <div v-for="(m, i) in messages" :key="i" :class="['msg', m.role]">
    <div class="bubble">
      <div v-if="m.role === 'assistant' && m.category !== 'miss'" class="meta">
        <span class="m-cat">{{ m.category.replace(/_/g, ' ') }}</span>
        <span class="m-score">{{ m.score?.toFixed(2) }}</span>
        <span class="m-ms">{{ m.elapsed?.toFixed(1) }}ms</span>
      </div>
      <div v-if="m.role === 'user'" class="msg-txt">{{ m.text }}</div>
      <div v-else class="msg-rich" v-html="m.typing ? m.displayHtml : m.html"></div>
      <span v-if="m.typing" class="cursor">▋</span>
      <div v-if="m.followUps?.length && !m.typing" class="fups">
        <button v-for="f in m.followUps" :key="f" @click="$emit('followUp', f)" class="fup-btn">→ {{ f }}</button>
      </div>
    </div>
  </div>

  <div v-if="isLoading" class="msg assistant">
    <div class="bubble"><div class="dots"><span/><span/><span/></div></div>
  </div>
</div>
</template>

<style scoped>
.cb-chat{flex:1;overflow-y:auto;padding:20px;display:flex;flex-direction:column;gap:16px;scroll-behavior:smooth}
.cb-chat::-webkit-scrollbar{width:6px}.cb-chat::-webkit-scrollbar-thumb{background:var(--vp-c-divider);border-radius:3px}

.msg{display:flex}.msg.user{justify-content:flex-end}.msg.assistant{justify-content:flex-start}
.bubble{max-width:100%;padding:12px 16px;border-radius:16px;animation:slideIn .3s}
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

@keyframes slideIn{from{opacity:0;transform:translateY(8px)}to{opacity:1;transform:translateY(0)}}
@keyframes blink{0%,100%{opacity:1}50%{opacity:0}}
@keyframes bounce{0%,60%,100%{transform:translateY(0)}30%{transform:translateY(-6px)}}
</style>
