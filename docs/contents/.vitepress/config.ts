import { transformerTwoslash } from '@shikijs/vitepress-twoslash'
import { defineConfig } from 'vitepress'
import { enConfig } from './configs/en'
import { zhHansConfig } from './configs/zh_hans'
import { docsConfig } from './docs'
import useBaseUrl from './hooks/useBaseUrl'
import { ImagePlugin } from './plugins/markdown/image'

const baseUrl = useBaseUrl()

export default defineConfig({
  base: baseUrl,
  ...docsConfig,
  themeConfig: {
    search: {
      provider: 'local',
      options: {
        locales: {
          zh_hans: {
            translations: {
              button: {
                buttonText: '搜索文档',
                buttonAriaLabel: '搜索文档',
              },
              modal: {
                noResultsText: '无法找到相关结果',
                resetButtonTitle: '清除查询条件',
                footer: {
                  selectText: '选择',
                  navigateText: '切换',
                },
              },
            },
          },
        },
      },
    },
  },
  head: [
    // favicon.ico
    ['link', { rel: 'icon', href: `${baseUrl}/favicon.ico` }],
    // others
    ['link', { rel: 'icon', href: '/logo/pycn-logo.png' }],
  ],
  locales: {
    root: {
      label: 'English',
      lang: 'en',
      link: '/',
      ...enConfig,
    },
    zh_hans: {
      label: '简体中文',
      lang: 'zh_hans',
      link: '/zh_hans/',
      ...zhHansConfig,
    },
  },
  markdown: {
    codeTransformers: [
      transformerTwoslash(),
    ],
    config: (md) => {
      md.use(ImagePlugin)
    },
  },
  lastUpdated: true,
})
