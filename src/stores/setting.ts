import { defineStore } from 'pinia'
import { setSetting } from '@/utils'

export const useSettingStore = defineStore('setting-store', {
  state: (): Setting => ({
    locale: 'en',
    mod: {
      path: '',
    },
  }),
  actions: {
    initSettings(settings: NullablePartial<Setting>) {
      this.$patch(state => {
        let key: keyof Setting
        for (key in settings) {
          const value = settings[key]
          if (!value) continue

          if (typeof value === 'object') {
            for (const childKey in value) {
              //@ts-ignore
              const childValue = value[childKey]
              if (childValue === undefined) continue
              //@ts-ignore
              state[key][childKey] = childValue
            }
          } else {
            //@ts-ignore
            state[key] = value
          }
        }
      })
    },
    async updateModPath(path?: string) {
      if (path) {
        this.mod.path = path
      }
      await setSetting('mod', this.mod)
    },
  },
})
