import type { DefaultTheme, LocaleSpecificConfig } from 'vitepress'
import getNavs from '../navs/en'
import sidebar from '../sidebars/en'

export const enConfig: LocaleSpecificConfig<DefaultTheme.Config> = {
  themeConfig: {
    logo: '/logo/pycn-logo.png',
    outlineTitle: 'Contents',
    outline: 'deep',
    sidebar,
    nav: getNavs(),
    socialLinks: [
      { icon: 'github', link: 'https://github.com/Vincent-the-gamer/pycn' },
    ],
    docFooter: {
      prev: '← Previous',
      next: 'Next →',
    },
    footer: {
      message: `Document by Vincent-the-gamer | MIT Licensed`,
      copyright: 'Copyright © 2025-PRESENT Vincent-the-gamer',
    },
    lightModeSwitchTitle: 'Switch to light mode.',
    darkModeSwitchTitle: 'Switch to dark mode.',
    lastUpdatedText: 'Last updated',
  },
}
