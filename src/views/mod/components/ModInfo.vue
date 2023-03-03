<template>
  <n-drawer
    :show="show"
    placement="bottom"
    :trap-focus="false"
    :block-scroll="false"
    :to="to"
    height="50vh"
    @mask-click="show = false"
  >
    <div></div>
  </n-drawer>
</template>

<script setup lang="ts">
  defineProps<{ to: string }>()
  defineExpose({ open })

  const modInfo = ref<ModInfo>()
  const show = ref(false)

  function open(mod: ModInfo) {
    modInfo.value = mod
    show.value = true
  }
</script>
