import { Store } from 'tauri-plugin-store-api'
import { useSettingStore } from '@/stores'

const store = new Store('.settings')

export async function setSetting(key: string, value: any) {
  try {
    await store.set(key, value)
    await saveSetting()
  } catch {
    window.$message?.error('set settings failed')
  }
}

export async function getSetting<T>(key: string) {
  try {
    return await store.get<T>(key)
  } catch {
    window.$message?.error('get settings failed')
  }
}

export async function saveSetting() {
  try {
    await store.save()
  } catch {
    window.$message?.error('save settings failed')
  }
}

export async function remove(key: string) {
  try {
    await store.delete(key)
  } catch {
    window.$message?.error('remove settings failed')
  }
}

export async function clear() {
  try {
    await store.clear()
  } catch {
    window.$message?.error('clear settings failed')
  }
}

export async function loadClientSetting() {
  try {
    await store.load()
    const locale = (await getSetting('locale')) as Setting['locale'] | null
    const mod = (await getSetting('mod')) as Setting['mod'] | null

    const settingStore = useSettingStore()
    settingStore.initSettings({
      locale,
      mod,
    })
  } catch {
    window.$message?.error('load settings failed')
  }
}
