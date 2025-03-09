import { defineConfig, presetWind3 } from 'unocss'
import presetIcons from '@unocss/preset-icons'
import presetTypography from '@unocss/preset-typography'
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
    presetTypography({
      cssExtend: {
        'hr': {
          position: 'relative',
          'border-bottom': '1px solid #cccccc',
          opacity: '66%',
          'counter-increment': 'hr',
          overflow: 'visible',
          display: 'flex',
          'justify-content': 'center',
          'align-items': 'center',
        },
        'hr::after': {
          'background-color': 'white',
          content: 'counter(hr, emoticon)',
          'line-height': '20px',
          'text-align': 'center',
          padding: '0 4px',
          'letter-spacing': '0.1rem',
        },
        'a:hover': {
          color: '#f43f5e',
        },
      }
    }),
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
