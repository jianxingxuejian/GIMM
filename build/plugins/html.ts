import type { PluginOption } from 'vite'
import { createHtmlPlugin } from 'vite-plugin-html'

export default (env: ImportMetaEnv): PluginOption[] => {
  return createHtmlPlugin({
    minify: true,
    inject: {
      data: {
        appName: env.VITE_APP_NAME,
        appTitle: env.VITE_APP_TITLE,
      },
    },
  })
}
