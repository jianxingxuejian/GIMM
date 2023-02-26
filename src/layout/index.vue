<template>
  <n-layout has-sider class="h-screen">
    <n-layout-sider
      v-model:collapsed="appStore.collapsed"
      collapse-mode="width"
      width="9rem"
      :collapsed-width="appStore.fontSize * 4"
      show-trigger="arrow-circle"
      class="h-screen"
    >
      <layout-menu class="h-90%" />
      <language class="h-10%" />
    </n-layout-sider>
    <n-layout-content>
      <layout-main />
    </n-layout-content>
  </n-layout>
</template>

<script setup lang="ts">
  import { useAppStore } from '@/stores'
  import { LayoutMain, LayoutMenu, Language } from './components'

  const appStore = useAppStore()
</script>

<style lang="scss" scoped>
  :deep(.n-layout-sider) {
    .n-layout-toggle-button {
      width: 2rem;
      height: 2rem;
      font-size: 1.5rem;
      right: 10%;
      opacity: 0.5;
    }
  }
</style>
