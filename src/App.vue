<template>
  <n-config-provider :locale="locale" :date-locale="dateLocale" :theme-overrides="themeOverrides">
    <naive-provider>
      <router-view />
    </naive-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
  import { zhCN, enUS, dateZhCN, dateEnUS } from 'naive-ui'
  import { useEventListener } from '@vueuse/core'
  import { useAppStore, useSettingStore } from '@/stores'

  const appStore = useAppStore()
  const settingStore = useSettingStore()

  const locale = computed(() => {
    switch (settingStore.locale) {
      case 'en':
        return enUS
      case 'zh-CN':
        return zhCN
      default:
        return enUS
    }
  })
  const dateLocale = computed(() => {
    switch (settingStore.locale) {
      case 'en':
        return dateEnUS
      case 'zh-CN':
        return dateZhCN
      default:
        return dateEnUS
    }
  })

  const themeOverrides = reactive({
    common: {
      borderRadius: '0.3rem',
      fontSize: '1rem',
      fontSizeSmall: '0.8rem',
      fontSizeMedium: '1rem',
      fontSizeLarge: '1.2rem',
      heightSmall: '1.5rem',
      heightMedium: '2rem',
      heightLarge: '2.5rem',
    },
    Button: {
      paddingMedium: '0 0.5rem',
      iconMarginMedium: '0.25rem',
      iconSizeMedium: '1.5rem',
    },
    Card: {
      titleFontSizeMedium: '1.5rem',
      closeIconSize: '1.5rem',
      closeSize: '1.75rem',
    },
    Checkbox: {
      sizeMedium: '0.875rem',
      fontSizeMedium: '1rem',
    },
    Menu: {
      fontSize: '1.25rem',
      itemHeight: '4rem',
    },
  })

  function resizeFontSize() {
    let width = document.body.clientWidth
    let fontSize = (width / 1920) * 30 <= 12.5 ? 12.5 : (width / 1920) * 30
    document.documentElement.style.fontSize = fontSize + 'px'
    appStore.updateFontSize(fontSize)
  }

  useEventListener(window, 'resize', () => resizeFontSize())

  onMounted(() => resizeFontSize())
</script>

<style lang="scss">
  @import '@/styles/index.scss';
</style>
