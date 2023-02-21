interface Setting {
  locale: LocaleCode
}

type LocaleCode = 'en' | 'zh-CN'

type LocaleModule = Record<LocaleCode, { default: unknown }>
