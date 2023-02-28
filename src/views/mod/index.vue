<template>
  <div class="flex">
    <div class="w-20% h-screen sticky top-0 flex-col of-auto">
      <div class="flex p-2">
        <enhanced-button @click="settingRef?.show">
          <icon-ic-outline-settings />
        </enhanced-button>
        <setting-modal ref="settingRef" />
        <n-input />
      </div>
      <n-tree :data="treeData" checkable expand-on-click selectable check-strategy="parent" />
    </div>
    <div
      class="grow my-2 gap-2 grid grid-cols-3 sm:grid-cols-4 md:grid-cols-5 lg:grid-cols-6 xl:grid-cols-7 2xl:grid-cols-8"
    >
      <div
        v-for="item in modShowList"
        :key="item.id"
        class="flex-col items-center"
        :style="{ aspectRatio: `${mod.width}/${mod.height}` }"
      >
        <div
          class="w-full grow bg-center bg-no-repeat bg-contain border-slate-400 border-1 cursor-pointer"
          :style="{ backgroundImage: `url(${item.localImages[0]})` }"
        ></div>
      </div>
    </div>
    <!-- <mod-info ref="modInfoRef" :show="showInfo" :mod="currentMod" /> -->
  </div>
</template>

<script setup lang="ts">
  import { useI18n } from 'vue-i18n'
  import { uniq } from 'lodash-es'
  import { useSettingStore } from '@/stores'
  import { get_mod_list } from '@/utils'
  import { ModInfo, SettingModal } from './components'

  const { t } = useI18n()
  const { mod } = useSettingStore()

  const settingRef = ref<InstanceType<typeof SettingModal>>()
  const modInfoRef = ref<InstanceType<typeof ModInfo>>()

  const modList = ref<ModInfo[]>([])
  const modShowList = computed(() => modList.value)
  const showInfo = ref(true)
  const currentMod = ref<ModInfo>()

  const treeData = computed(() => [
    {
      key: 'Character',
      label: t('common.character'),
      children: uniq(
        modList.value
          .filter(({ metadata }) => metadata.categories[0] === 'Character')
          .map(({ metadata }) => metadata.categories[1])
      ).map(name => ({ key: name, label: t(`Character.${name.toLocaleLowerCase()}`) })),
    },
    { key: 'NPC', label: t('common.NPC'), children: [] },
    { key: 'Enemy', label: t('common.enemy'), children: [] },
    { key: 'Weapon', label: t('common.weapon'), children: [] },
    { key: 'Entities', label: t('common.entities'), children: [] },
    { key: 'Object', label: t('common.object'), children: [] },
    { key: 'TCG_Card', label: t('common.TCG_card'), children: [] },
    { key: 'Other', label: t('common.other'), children: [] },
  ])

  const loadModList = async () => {
    modList.value = []
    try {
      const originModList = await get_mod_list(mod.path)
      modList.value = originModList.flatMap(item =>
        item.deepChildren.length > 0 ? item.deepChildren : [item]
      )
    } catch (e) {
      console.log(e)
      window.$message?.warning(t('not found mod path'))
    }
  }

  loadModList()
</script>
