import type { PluginOption } from 'vite'
import html from './html'
import i18n from './i18n'
import unocss from './unocss'
import unplugin from './unplugin'
import vue from './vue'

/**
 * setup vite plugins
 */
export function setupVitePlugins(env: ImportMetaEnv): PluginOption[] {
  return [html(env), i18n, unocss, ...unplugin, vue]
}
