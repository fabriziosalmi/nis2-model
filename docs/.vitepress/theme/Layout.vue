<script setup>
import { useData } from 'vitepress'
import DefaultTheme from 'vitepress/theme'
import ChatBot from './ChatBot.vue'

const { frontmatter } = useData()
</script>

<template>
  <ChatBot v-if="frontmatter.chatbot" />
  <DefaultTheme.Layout v-else />
</template>

<style>
/* Hide VitePress chrome when chatbot is active */
.chatbot-root ~ .VPNav,
.chatbot-root ~ .VPSidebar {
  display: none;
}
</style>
