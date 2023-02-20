<template>
  <n-config-provider>
    <naive-provider>
      <router-view />
    </naive-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
  import { useEventListener } from '@vueuse/core'
  import { useAppStore } from '@/stores'

  const appStore = useAppStore()

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
  @import '@/styles/scss/naive-ui.scss';
</style>
