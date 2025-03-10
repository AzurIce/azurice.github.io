import { defineConfig } from 'unocss'
import presetIcons from '@unocss/preset-icons'
import presetWebFonts from '@unocss/preset-web-fonts'
import presetWind4 from '@unocss/preset-wind4'

export default defineConfig({
  cli: {
    entry: [{
      patterns: ["templates/**/*.html", "public/**/*.html"],
      outFile: "static/uno.css"
    }],
  },
  presets: [
    presetIcons({}),
    presetWebFonts({
      provider: 'none',
      fonts: {
        header: ['LXGW Bright'],
        mono: ['JetBrainsMono NFM', 'Consolas']
      }
    }),
    presetWind4(),
  ],
  rules: [
    ['backdrop-blur-md', { 'backdrop-filter': 'blur(var(--blur-md))' }],
  ],
  shortcuts: {
    'nav-btn': 'flex items-center justify-center rounded outline-transparent outline outline-solid hover:outline-slate-200 transition-[outline] duration-300 ease-in-out',
  }
})
