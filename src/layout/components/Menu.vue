<template>
  <n-menu :value="activeKey" :options="menus" class="flex-col" @update:value="handleUpdateMenu" />
</template>

<script setup lang="ts">
  import { useI18n } from 'vue-i18n'
  import modules from '@/router/modules'

  const { t } = useI18n()

  const menus = computed(() => transformRouteToMenu(modules))

  function transformRouteToMenu(routes: Route.RecordRaw[]): Route.Menu[] {
    const menus: Route.Menu[] = []
    routes.forEach(route => {
      const { name } = route
      const label = t(`route.${name}`)
      const icon = route.meta?.icon ?? route.children?.[0]?.meta?.icon
      const isRoot = route.meta?.isRoot && route.children && route.children.length == 1
      const children = isRoot ? undefined : route.children && transformRouteToMenu(route.children)
      const key = isRoot ? route.children![0].name : route.name
      menus.push({ key, label, icon, children })
    })
    return menus
  }

  const router = useRouter()
  const route = useRoute()

  const activeKey = computed(() => route.matched[1].name as string)

  const handleUpdateMenu = (key: string) => router.push({ name: key })
</script>

<style lang="scss" scoped>
  :deep(.n-menu-item-content) {
    padding: 0.5rem !important;
  }

  :deep(.n-menu-item-content__icon) {
    width: auto !important;
    font-size: 2rem !important;
    margin-left: 0.5rem !important;
    margin-right: 0.5rem !important;
  }
</style>
