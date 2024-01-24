// uno.config.ts
import { defineConfig } from 'unocss'
import presetUno from '@unocss/preset-uno'

export default defineConfig({
  presets: [
    presetUno(),
  ],
  cli: {
    entry: {
      patterns: ["templates/*.html"],
      outFile: "css/uno.css"
    }
  }
})