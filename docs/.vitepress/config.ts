import { defineConfig } from 'vitepress';
import { presetUno } from 'unocss';
import UnoCSS from 'unocss/vite';

export default defineConfig({
  base: '/miho/',
  title: 'Miho',
  description: 'Easily bump your package version',
  lang: 'en',
  lastUpdated: true,

  sitemap: {
    hostname: 'https://tb.dev.br'
  },

  vite: {
    plugins: [UnoCSS({ presets: [presetUno()] })]
  },

  themeConfig: {
    nav: [
      {
        text: 'API',
        link: 'https://tb.dev.br/miho/api/index.html'
      }
    ],

    sidebar: {
      '/': [
        {
          text: 'Usage',
          collapsed: false,
          items: [
            { text: 'CLI ', link: '/usage/cli' },
            { text: 'Node ', link: '/usage/node' }
          ]
        }
      ]
    },

    editLink: {
      pattern: 'https://github.com/ferreira-tb/miho/edit/main/docs/:path'
    },

    search: {
      provider: 'local'
    },

    socialLinks: [
      {
        icon: 'github',
        link: 'https://github.com/ferreira-tb/miho'
      }
    ],

    footer: {
      copyright:
        'Copyright © 2023 <a href="https://github.com/ferreira-tb">Andrew Ferreira</a>'
    }
  }
});
