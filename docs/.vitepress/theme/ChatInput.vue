<script setup>
defineProps({
  input: String,
  disabled: Boolean,
  placeholder: { type: String, default: 'Ask about NIS2 compliance...' },
  disclaimer: { type: String, default: '' }
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
    <button @click="$emit('send')" :disabled="disabled || !input?.trim()" class="send">
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="22" y1="2" x2="11" y2="13"/><polygon points="22 2 15 22 11 13 2 9 22 2"/></svg>
    </button>
  </div>
  <div v-if="disclaimer" class="disclaimer">{{ disclaimer }}</div>
</div>
</template>

<style scoped>
.cb-in{
  padding:10px 24px 14px;
  background:var(--vp-c-bg);
  flex-shrink:0;
  font-family:-apple-system,BlinkMacSystemFont,'SF Pro Text',sans-serif;
  border-top:1px solid var(--vp-c-divider);
}
.in-wrap{
  display:flex;gap:8px;max-width:720px;margin:0 auto;
}
.in-wrap input{
  flex:1;padding:12px 20px;
  border:1px solid var(--vp-c-divider);border-radius:22px;
  background:var(--vp-c-bg-soft);color:var(--vp-c-text-1);
  font-size:15px;outline:none;
  transition:border-color .2s,box-shadow .2s;
  letter-spacing:-.01em;
}
.in-wrap input:focus{
  border-color:var(--vp-c-brand-1);
  box-shadow:0 0 0 3px rgba(59,130,246,.08);
}
.in-wrap input::placeholder{color:var(--vp-c-text-3);opacity:.7}
.send{
  width:42px;height:42px;border:none;border-radius:50%;
  background:var(--vp-c-brand-1);color:#fff;
  cursor:pointer;display:flex;align-items:center;justify-content:center;
  transition:transform .12s,opacity .12s;flex-shrink:0;
}
.send:hover:not(:disabled){transform:scale(1.06)}
.send:active:not(:disabled){transform:scale(.95)}
.send:disabled{opacity:.25;cursor:not-allowed}
.disclaimer{
  max-width:720px;margin:6px auto 0;
  font-size:11px;color:var(--vp-c-text-3);
  opacity:.5;text-align:center;letter-spacing:.01em;
}
</style>
