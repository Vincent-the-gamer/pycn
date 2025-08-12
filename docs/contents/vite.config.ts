import { presetAttributify, presetWind3 } from 'unocss'
import UnoCSS from 'unocss/vite'
import { defineConfig } from 'vite'

export default defineConfig({
  plugins: [
    UnoCSS({
      presets: [
        presetWind3(),
        presetAttributify(),
      ],
    }),
  ],
  server: {
    host: '0.0.0.0',
    port: 8080,
  },
})
