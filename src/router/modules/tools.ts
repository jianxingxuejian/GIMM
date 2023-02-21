const routes: Route.Config = {
  path: '/tools',
  name: 'tools',
  redirect: '/tools/index',
  component: () => import('@/layout/index.vue'),
  meta: {
    sort: 3,
    isRoot: true,
    icon: 'mdi:tools',
  },
  children: [
    {
      path: 'index',
      name: 'tools_index',
      component: () => import('@/views/tools/index.vue'),
    },
  ],
}

export default routes
