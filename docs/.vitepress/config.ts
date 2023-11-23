import { defineConfig } from 'vitepress';
import { presetUno } from 'unocss';
import UnoCSS from 'unocss/vite';

export default defineConfig({
  base: '/miho/',
  title: 'Miho',
  description: 'Easily bump your package version',
  lang: 'en',
  lastUpdated: true,
  cleanUrls: true,

  sitemap: {
    hostname: 'https://tb.dev.br'
  },

  vite: {
    plugins: [UnoCSS({ presets: [presetUno()] })]
  },

  head: [
    [
      'meta',
      {
        name: 'google-site-verification',
        content: 'FpKCfhe8tgbogFn89w4fUPpqlYF_Hcrv7h6GpUL8rdE'
      }
    ]
  ],

  themeConfig: {
    nav: [
      {
        text: 'typedoc',
        link: 'https://tb.dev.br/miho/typedoc/index.html'
      }
    ],

    sidebar: [
      { text: 'CLI', link: '/cli/' },
      { text: 'Javascript API', link: '/javascript-api/' },
      { text: 'Hooks', link: '/hooks/' },
      { text: 'Typescript Utilities', link: '/typescript-utilities/' }
    ],

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
