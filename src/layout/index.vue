<template>
  <n-layout has-sider class="h-screen">
    <n-layout-sider show-trigger="arrow-circle" class="h-screen">
      <layout-menu />
    </n-layout-sider>
    <n-layout-content>
      <layout-main />
    </n-layout-content>
  </n-layout>
</template>

<script setup lang="ts">
  import { LayoutMain, LayoutMenu } from './components'
</script>
