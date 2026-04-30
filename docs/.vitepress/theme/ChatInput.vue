<script setup>
defineProps({
  input: String,
  disabled: Boolean,
  placeholder: { type: String, default: 'Ask about NIS2 compliance...' },
  disclaimer: { type: String, default: '' },
  sendTooltip: { type: String, default: 'Send (Enter)' }
})
defineEmits(['update:input', 'send'])
</script>

<template>
<div class="cb-in">
  <div class="in-wrap">
    <input
      :value="input"
      @input="$emit('update:input', $event.target.value)"
      @keyup.enter="$emit('send')"
      :placeholder="placeholder"
      :disabled="disabled"
      autocomplete="off"
    />
    <button @click="$emit('send')" :disabled="disabled || !input?.trim()" :class="['send', { active: input?.trim() }]" :aria-label="sendTooltip" :title="sendTooltip">
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><line x1="22" y1="2" x2="11" y2="13"/><polygon points="22 2 15 22 11 13 2 9 22 2"/></svg>
    </button>
  </div>
  <div v-if="disclaimer" class="disclaimer">{{ disclaimer }}</div>
</div>
</template>

<style scoped>
.cb-in{
  padding:12px 32px 16px;
  background:var(--vp-c-bg);
  flex-shrink:0;
  font-family:-apple-system,BlinkMacSystemFont,'SF Pro Text','Inter',system-ui,sans-serif;
  -webkit-font-smoothing:antialiased;
  -moz-osx-font-smoothing:grayscale;
  border-top:1px solid var(--vp-c-divider);
}
.in-wrap{
  display:flex;gap:8px;max-width:640px;
}
.in-wrap input{
  flex:1;padding:12px 20px;
  border:1px solid var(--vp-c-divider);border-radius:24px;
  background:var(--vp-c-bg-soft);color:var(--vp-c-text-1);
  font-size:15px;outline:none;
  transition:border-color .2s ease-out,box-shadow .2s ease-out,background .2s ease-out;
}
.in-wrap input:focus{
  border-color:var(--vp-c-brand-1);
  box-shadow:0 0 0 3px rgba(37,99,235,.12);
  background:var(--vp-c-bg);
}
.in-wrap input::placeholder{color:var(--vp-c-text-3);opacity:1}
/* Send button — 6 states */
.send{
  width:44px;height:44px;border:none;border-radius:50%;
  background:var(--vp-c-bg-alt);color:var(--vp-c-text-3);
  cursor:pointer;display:flex;align-items:center;justify-content:center;
  transition:all .2s ease-out;flex-shrink:0;
}
.send.active{background:var(--vp-c-brand-1);color:#fff}
.send:hover:not(:disabled){transform:scale(1.06)}
.send:active:not(:disabled){transform:scale(.95)}
.send:disabled{opacity:.3;cursor:not-allowed}
.send:focus-visible{outline:2px solid var(--vp-c-brand-1);outline-offset:2px}
.disclaimer{
  max-width:640px;margin:8px 0 0;
  font-size:12.5px;color:var(--vp-c-text-2);
  letter-spacing:.01em;line-height:1.4;
  padding:6px 0;
}
</style>
