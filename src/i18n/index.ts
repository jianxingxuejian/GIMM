import type { App } from 'vue'
import { createI18n } from 'vue-i18n'
import messages from '@intlify/unplugin-vue-i18n/messages'

const i18n = createI18n({
  legacy: false,
  messages,
  locale: 'en',
  fallbackLocale: 'en',
})

/** setup i18n */
export function setupI18n(app: App) {
  app.use(i18n)
}

export default i18n
