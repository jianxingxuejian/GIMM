<template>
  <div class="flex-center">
    <n-popselect
      v-model:value="settingStore.locale"
      :options="languageOptions"
      @update:value="updateLocale"
    >
      <icon-ion-language class="text-8 outline-unset" />
    </n-popselect>
  </div>
</template>

<script setup lang="ts">
  import { useI18n } from 'vue-i18n'
  import { useSettingStore } from '@/stores'

  const languageOptions: {
    label: string
    value: Setting['locale']
  }[] = [
    { label: 'English', value: 'en' },
    { label: '中文', value: 'zh-CN' },
  ]

  const settingStore = useSettingStore()
  const { updateLocale } = settingStore

  const { locale } = useI18n({ useScope: 'global' })
  watchEffect(() => (locale.value = settingStore.locale))
</script>
