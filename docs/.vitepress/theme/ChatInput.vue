<script setup>
defineProps({ input: String, disabled: Boolean, placeholder: { type: String, default: 'Ask about NIS2 compliance...' } })
defineEmits(['update:input', 'send'])
</script>

<template>
<div class="cb-in">
  <input
    :value="input"
    @input="$emit('update:input', $event.target.value)"
    @keyup.enter="$emit('send')"
    :placeholder="placeholder"
    :disabled="disabled"
    autocomplete="off"
  />
  <button @click="$emit('send')" :disabled="disabled || !input?.trim()" class="send">
    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <path d="M22 2L11 13M22 2l-7 20-4-9-9-4 20-7z"/>
    </svg>
  </button>
</div>
</template>

<style scoped>
.cb-in{display:flex;gap:8px;padding:12px 16px;background:var(--vp-c-bg);border-top:1px solid var(--vp-c-divider);flex-shrink:0;font-family:-apple-system,BlinkMacSystemFont,'SF Pro Text',sans-serif}
.cb-in input{flex:1;padding:10px 16px;border:1px solid var(--vp-c-divider);border-radius:20px;background:var(--vp-c-bg-soft);color:var(--vp-c-text-1);font-size:14px;outline:none;transition:border-color .2s;letter-spacing:-.01em}
.cb-in input:focus{border-color:var(--vp-c-brand-1);box-shadow:0 0 0 3px rgba(59,130,246,.1)}
.cb-in input::placeholder{color:var(--vp-c-text-3)}
.send{width:40px;height:40px;border:none;border-radius:50%;background:var(--vp-c-brand-1);color:white;cursor:pointer;display:flex;align-items:center;justify-content:center;transition:transform .15s,opacity .15s;flex-shrink:0}
.send:hover:not(:disabled){transform:scale(1.05)}.send:disabled{opacity:.3;cursor:not-allowed}
</style>
