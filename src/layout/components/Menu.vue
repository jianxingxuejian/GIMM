<template>
  <n-menu
    :value="activeKey"
    :options="menus"
    :collapsed-width="15"
    @update:value="handleUpdateMenu"
  />
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
      if (route.meta?.isRoot && route.children && route.children.length == 1) {
        const child = route.children[0]
        const menu: Route.Menu = {
          key: child.name,
          label,
          icon: route.meta?.icon || child.meta?.icon,
        }
        menus.push(menu)
      } else {
        const menu: Route.Menu = {
          key: route.name,
          label,
          icon: route.meta?.icon,
          children: route.children && transformRouteToMenu(route.children),
        }
        menus.push(menu)
      }
    })
    return menus
  }

  const router = useRouter()
  const route = useRoute()

  const activeKey = computed(() => route.matched[1].name as string)

  function handleUpdateMenu(key: string) {
    router.push({ name: key })
  }
</script>
