interface Setting {
  locale: LocaleCode
  mod: ModSettings
}

interface ModSettings {
  path: string
  width: number
  height: number
}

type LocaleCode = 'en' | 'zh-CN'

type LocaleModule = Record<LocaleCode, { default: unknown }>
