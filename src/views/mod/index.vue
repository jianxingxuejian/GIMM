<template>
  <div class="h-screen">
    <div class="flex-center">
      <n-space>
        <enhanced-button @click="settingRef?.show">
          <icon-ic-outline-settings />
        </enhanced-button>
        <!-- <n-cascader
          :options="options"
          multiple
          clearable
          filterable
          trigger="hover"
          check-strategy="parent"
          max-tag-count="responsive"
          class="w-50"
        /> -->
      </n-space>
    </div>
    <div
      class="grid gap-1 py-1 px-4 sm:grid-cols-5 md:grid-cols-6 lg:grid-cols-7 xl:grid-cols-8 2xl:grid-cols-9"
    >
      <div></div>
    </div>
    <setting-modal ref="settingRef" />
  </div>
</template>

<script setup lang="ts">
  import { useI18n } from 'vue-i18n'
  // import { uniq } from 'lodash-es'
  import { useSettingStore } from '@/stores'
  import { get_mod_list } from '@/utils'
  import { SettingModal } from './components'
  import { getCategories } from './handle'

  console.log(getCategories('AlbedoMod'))

  const { t } = useI18n()
  const settingStore = useSettingStore()

  const settingRef = ref<InstanceType<typeof SettingModal>>()

  const modList = ref<ModInfo[]>([])
  // const options = computed(() => [
  //   {
  //     value: 'character',
  //     label: t('characters'),
  //     children: uniq(modList.value.map(item => item.name)).map(item => ({
  //       value: item,
  //       label: t(item!),
  //     })),
  //   },
  // ])

  const loadModList = async () => {
    modList.value = []
    try {
      modList.value = await get_mod_list(settingStore.mod.path)
      console.log(modList.value)
      // modList.value.sort((next, pre) => {
      //   if (next.enabled && !pre.enabled) {
      //     return -1
      //   } else if (next.enabled == pre.enabled) {
      //     return next.name.localeCompare(pre.name, settingStore.locale)
      //   }
      //   return 0
      // })
    } catch (e) {
      console.log(e)
      window.$message?.warning(t('not found mod path'))
    }
  }

  loadModList()
</script>
