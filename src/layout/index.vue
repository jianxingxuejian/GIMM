<template>
  <n-layout has-sider class="h-screen">
    <n-layout-sider
      v-model:collapsed="appStore.collapsed"
      collapse-mode="width"
      width="9rem"
      :collapsed-width="appStore.fontSize * 4"
      show-trigger="arrow-circle"
      trigger-style="left: 7rem;opacity: 0.5;"
      collapsed-trigger-style="left: 0rem;opacity: 0.5;"
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
