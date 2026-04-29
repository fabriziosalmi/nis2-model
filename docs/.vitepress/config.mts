import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'nis2-model',
  description: 'Reference implementation for NIS2/DORA compliance logic in Rust',
  lang: 'en-US',

  head: [
    ['meta', { name: 'theme-color', content: '#1a56db' }],
    ['meta', { name: 'og:type', content: 'website' }],
    ['meta', { name: 'og:title', content: 'nis2-model' }],
    ['meta', { name: 'og:description', content: 'Reference implementation for NIS2/DORA compliance logic in Rust' }],
    ['link', { rel: 'icon', type: 'image/svg+xml', href: '/logo.svg' }],
  ],

  themeConfig: {
    logo: '/logo.svg',

    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guide', link: '/guide/getting-started' },
      { text: 'Architecture', link: '/architecture/overview' },
      { text: 'API', link: '/api/mcp-tools' },
      {
        text: 'References',
        items: [
          { text: 'NIS2 (EU 2022/2555)', link: 'https://eur-lex.europa.eu/eli/dir/2022/2555' },
          { text: 'DORA (EU 2022/2554)', link: 'https://eur-lex.europa.eu/eli/reg/2022/2554' },
        ]
      }
    ],

    sidebar: [
      {
        text: 'Guide',
        items: [
          { text: 'Overview', link: '/guide/overview' },
          { text: 'Getting Started', link: '/guide/getting-started' },
          { text: 'Key Concepts', link: '/guide/concepts' },
        ]
      },
      {
        text: 'Architecture',
        items: [
          { text: 'System Overview', link: '/architecture/overview' },
          { text: 'Ingestion Pipeline', link: '/architecture/ingestion' },
          { text: 'Vector Store', link: '/architecture/vectordb' },
          { text: 'Rule Engine', link: '/architecture/rules' },
          { text: 'MCP Server', link: '/architecture/mcp' },
          { text: 'SLM Engine', link: '/architecture/slm' },
        ]
      },
      {
        text: 'API Reference',
        items: [
          { text: 'MCP Tools', link: '/api/mcp-tools' },
          { text: 'REST API', link: '/api/rest' },
          { text: 'JSON Schema', link: '/api/schemas' },
          { text: 'CLI', link: '/api/cli' },
        ]
      },
      {
        text: 'Legal Reference',
        items: [
          { text: 'Obligations (Art. 21)', link: '/legal/obligations' },
          { text: 'Sanctions (Art. 34)', link: '/legal/sanctions' },
          { text: 'Incident Reporting (Art. 23)', link: '/legal/incidents' },
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/fabriziosalmi/nis2-model' }
    ],

    footer: {
      message: 'Released under the EUPL-1.2 License.',
      copyright: '2025 Fabrizio Salmi'
    },

    search: {
      provider: 'local'
    },

    editLink: {
      pattern: 'https://github.com/fabriziosalmi/nis2-model/edit/main/docs/:path',
      text: 'Edit this page on GitHub'
    },

    outline: {
      level: [2, 3],
      label: 'On this page'
    }
  }
})
