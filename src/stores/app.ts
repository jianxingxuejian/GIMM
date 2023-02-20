import { defineStore } from 'pinia'

interface AppState {
  fontSize: number
}

export const useAppStore = defineStore('app-store', {
  state: (): AppState => ({
    fontSize: 14,
  }),
  actions: {
    updateFontSize(fontSize: number) {
      this.fontSize = fontSize
    },
  },
})
