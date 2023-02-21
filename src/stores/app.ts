import { defineStore } from 'pinia'

interface AppState {
  collapsed: boolean
  fontSize: number
}

export const useAppStore = defineStore('app-store', {
  state: (): AppState => ({
    collapsed: false,
    fontSize: 14,
  }),
  actions: {
    updateFontSize(fontSize: number) {
      this.fontSize = fontSize
    },
  },
})
