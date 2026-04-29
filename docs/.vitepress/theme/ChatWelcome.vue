<script setup>
defineProps({
  suggestions: Array,
  entries: Number,
})
defineEmits(['select'])
</script>

<template>
<div class="welcome">
  <div class="w-icon">🇪🇺</div>
  <h2>NIS2 Compliance Q&A</h2>
  <p>{{ entries }} pre-computed answers · IT/EN · Deterministic · Zero LLM</p>
  <p class="w-link">
    <a href="https://github.com/fabriziosalmi/nis2-public" target="_blank" rel="noopener">
      📘 fabriziosalmi/nis2-public — Full NIS2 reference project
    </a>
  </p>
  <div class="suggestions">
    <button v-for="s in suggestions" :key="s.text" @click="$emit('select', s.text)" class="sug-btn">
      <span class="sug-icon">{{ s.icon }}</span>{{ s.text }}
    </button>
  </div>
</div>
</template>

<style scoped>
.welcome{display:flex;flex-direction:column;align-items:center;justify-content:center;flex:1;text-align:center;gap:12px;animation:fadeIn .6s}
.w-icon{font-size:56px}
.welcome h2{font-size:24px;font-weight:700;margin:0;background:linear-gradient(135deg,var(--vp-c-brand-1),#7c3aed,#db2777);-webkit-background-clip:text;-webkit-text-fill-color:transparent;background-clip:text}
.welcome p{color:var(--vp-c-text-2);font-size:14px;margin:0}
.w-link a{color:var(--vp-c-brand-1);text-decoration:none;font-weight:600;font-size:13px;transition:color .2s}
.w-link a:hover{color:#7c3aed}
.suggestions{display:flex;flex-wrap:wrap;gap:8px;justify-content:center;max-width:640px;margin-top:8px}
.sug-btn{padding:8px 16px;border:1px solid var(--vp-c-divider);border-radius:20px;background:var(--vp-c-bg-soft);color:var(--vp-c-text-1);font-size:13px;cursor:pointer;transition:all .2s;display:flex;align-items:center;gap:6px}
.sug-btn:hover{border-color:var(--vp-c-brand-1);background:var(--vp-c-brand-soft);transform:translateY(-1px)}
.sug-icon{font-size:16px}
@keyframes fadeIn{from{opacity:0;transform:translateY(12px)}to{opacity:1;transform:translateY(0)}}
</style>
