import { defineStore } from 'pinia'

export const useSettingStore = defineStore('setting-store', {
  state: (): Setting => ({
    locale: 'en',
  }),
})
