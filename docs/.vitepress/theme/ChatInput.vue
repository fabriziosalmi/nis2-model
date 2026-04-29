<script setup>
defineProps({ input: String, disabled: Boolean })
defineEmits(['update:input', 'send'])
</script>

<template>
<div class="cb-in">
  <input
    :value="input"
    @input="$emit('update:input', $event.target.value)"
    @keyup.enter="$emit('send')"
    placeholder="Ask about NIS2 compliance..."
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
.cb-in{display:flex;gap:8px;padding:14px 20px;background:var(--vp-c-bg-soft);border-top:1px solid var(--vp-c-divider);flex-shrink:0}
.cb-in input{flex:1;padding:12px 18px;border:1px solid var(--vp-c-divider);border-radius:24px;background:var(--vp-c-bg);color:var(--vp-c-text-1);font-size:14px;outline:none;transition:border-color .2s}
.cb-in input:focus{border-color:var(--vp-c-brand-1);box-shadow:0 0 0 3px var(--vp-c-brand-soft)}
.cb-in input::placeholder{color:var(--vp-c-text-3)}
.send{width:44px;height:44px;border:none;border-radius:50%;background:linear-gradient(135deg,var(--vp-c-brand-1),#7c3aed);color:white;cursor:pointer;display:flex;align-items:center;justify-content:center;transition:transform .15s,opacity .15s;flex-shrink:0}
.send:hover:not(:disabled){transform:scale(1.05)}.send:disabled{opacity:.4;cursor:not-allowed}
</style>
