<template>
  <div class="h-screen">
    <div>
      <n-space>
        <my-button @click="settingRef?.show">
          <icon-ic-outline-settings />
        </my-button>
      </n-space>
    </div>
    <div
      class="grid gap-1 py-1 px-4 sm:grid-cols-5 md:grid-cols-6 lg:grid-cols-7 xl:grid-cols-8 2xl:grid-cols-9"
    >
      <div></div>
    </div>
  </div>
  <setting-modal ref="settingRef" />
</template>

<script setup lang="ts">
  import { SettingModal } from './components'
  import { CharacterEnum } from './constant'

  const settingRef = ref<InstanceType<typeof SettingModal>>()

  const modList = ref<Mod[]>([])
</script>
