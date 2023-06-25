import { defineValaxyConfig } from 'valaxy'
import type { ThemeConfig } from 'valaxy-theme-aoike'

// add icons what you will need
const safelist = [
  'i-ri-home-line',
]

/**
 * User Config
 */
export default defineValaxyConfig<ThemeConfig>({
  // site config see site.config.ts

  theme: 'aoike',

  themeConfig: {
    colors: {
      primary: '#0078E7',
    },
  
    footer: {
      since: 2021,
      icon: {
        name: 'i-ri-cloud-line',
        animated: true,
        color: 'var(--va-c-primary)',
        url: '',
        title: '',
      },
  
      powered: true,
  
      beian: {
        enable: true,
        icp: '京ICP备2022004147号-1',
      },
    },
  
    nav: [
      {
        link: "/categories",
        text: "分类（WIP）"
      }
    ],
  },

  unocss: { safelist },
})
