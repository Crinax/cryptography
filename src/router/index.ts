import { createRouter, createWebHistory } from 'vue-router';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: () => import('@/views/HomeView.vue'),
    },
    {
      path: '/components',
      name: 'components',
      component: () => import('@/views/ComponentView.vue'),
      beforeEnter: () => import.meta.env.DEV,
    },
    {
      path: '/cesar',
      name: 'cesarCipher',
      component: () => import('@/views/CesarCipherView.vue'),
    },
    {
      path: '/cesar/decrypt',
      name: 'cesarCipherDecryption',
      component: () => import('@/views/CesarCipherDecryptView.vue'),
    },
  ],
});

export default router;
