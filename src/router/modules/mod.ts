const routes: Route.Config = {
  path: '/mod',
  name: 'mod',
  redirect: '/mod/index',
  component: () => import('@/layout/index.vue'),
  meta: {
    sort: 2,
    isRoot: true,
    icon: 'mdi:account-edit-outline',
  },
  children: [
    {
      path: 'index',
      name: 'mod_index',
      component: () => import('@/views/mod/index.vue'),
    },
  ],
}

export default routes
