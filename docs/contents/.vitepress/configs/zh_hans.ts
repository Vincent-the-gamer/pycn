import type { DefaultTheme, LocaleSpecificConfig } from 'vitepress'
import getNavs from '../navs/zh_hans'
import sidebar from '../sidebars/zh_hans'

export const zhHansConfig: LocaleSpecificConfig<DefaultTheme.Config> = {
  themeConfig: {
    logo: '/logo/pycn-logo.png',
    outlineTitle: '目录',
    outline: 'deep',
    nav: getNavs(),
    sidebar,
    socialLinks: [
      { icon: 'github', link: 'https://github.com/Vincent-the-gamer/pycn' },
    ],
    docFooter: {
      prev: '← 上一篇',
      next: '下一篇 →',
    },
    footer: {
      message: `文档由 Vincent-the-gamer 提供 | 使用 MIT 许可证开源`,
      copyright: '版权所有 © 2025-现在 Vincent-the-gamer',
    },
    lightModeSwitchTitle: '切换至明亮模式',
    darkModeSwitchTitle: '切换至暗黑模式',
    lastUpdatedText: '上次更新',
  },
}
