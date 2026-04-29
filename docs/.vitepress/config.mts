import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'NIS2 Compliance Engine',
  description: 'Deterministic, locally-executable compliance engine for EU NIS2/DORA regulation',
  lang: 'it-IT',

  head: [
    ['meta', { name: 'theme-color', content: '#1a56db' }],
    ['meta', { name: 'og:type', content: 'website' }],
    ['meta', { name: 'og:title', content: 'NIS2 Compliance Engine' }],
    ['meta', { name: 'og:description', content: 'Zero-hallucination compliance engine for NIS2 and DORA' }],
    ['link', { rel: 'icon', type: 'image/svg+xml', href: '/logo.svg' }],
  ],

  themeConfig: {
    logo: '/logo.svg',

    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guida', link: '/guide/getting-started' },
      { text: 'Architettura', link: '/architecture/overview' },
      { text: 'API', link: '/api/mcp-tools' },
      {
        text: 'Riferimenti',
        items: [
          { text: 'NIS2 (EU 2022/2555)', link: 'https://eur-lex.europa.eu/eli/dir/2022/2555' },
          { text: 'DORA (EU 2022/2554)', link: 'https://eur-lex.europa.eu/eli/reg/2022/2554' },
        ]
      }
    ],

    sidebar: [
      {
        text: 'Introduzione',
        items: [
          { text: 'Panoramica', link: '/guide/overview' },
          { text: 'Quick Start', link: '/guide/getting-started' },
          { text: 'Concetti chiave', link: '/guide/concepts' },
        ]
      },
      {
        text: 'Architettura',
        items: [
          { text: 'I 5 Pilastri', link: '/architecture/overview' },
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
          { text: 'JSON Schema', link: '/api/schemas' },
          { text: 'CLI', link: '/api/cli' },
        ]
      },
      {
        text: 'Riferimenti Normativi',
        items: [
          { text: 'Obblighi Art. 21', link: '/legal/obligations' },
          { text: 'Sanzioni Art. 34', link: '/legal/sanctions' },
          { text: 'Segnalazione Incidenti', link: '/legal/incidents' },
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/fabriziosalmi/nis2-model' }
    ],

    footer: {
      message: 'Released under the EUPL-1.2 License.',
      copyright: '© 2025 Fabrizio Salmi'
    },

    search: {
      provider: 'local'
    },

    editLink: {
      pattern: 'https://github.com/fabriziosalmi/nis2-model/edit/main/docs/:path',
      text: 'Modifica questa pagina su GitHub'
    },

    outline: {
      level: [2, 3],
      label: 'In questa pagina'
    },

    lastUpdated: {
      text: 'Ultimo aggiornamento'
    },

    docFooter: {
      prev: 'Pagina precedente',
      next: 'Pagina successiva'
    }
  }
})
