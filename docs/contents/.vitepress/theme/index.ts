import type { EnhanceAppContext } from 'vitepress'
import TwoslashFloatingVue from '@shikijs/vitepress-twoslash/client'
import DefaultTheme from 'vitepress/theme'
import { useMediumZoomProvider } from '../hooks'
import Layout from './Layout.vue'
import '@shikijs/vitepress-twoslash/style.css'
import './rainbow.css'
import './vars.css'
import './medium-zoom.css'
import 'virtual:uno.css'

export default {
  ...DefaultTheme,
  Layout,
  enhanceApp(ctx: EnhanceAppContext) {
    const { app, router, siteData } = ctx
    app.use(TwoslashFloatingVue)
    useMediumZoomProvider(app, router)
  },
}
