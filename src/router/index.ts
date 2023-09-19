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
      meta: {
        title: 'Components',
      },
      component: () => import('@/views/ComponentView.vue'),
      beforeEnter: () => import.meta.env.DEV,
    },
    {
      path: '/cesar',
      name: 'cesarCipher',
      meta: {
        // eslint-disable-next-line quotes
        title: "Encryption & Decryption | Cesar's cipher",
      },
      component: () => import('@/views/CesarCipherView.vue'),
    },
    {
      path: '/cesar/decrypt',
      name: 'cesarCipherDecryption',
      meta: {
        // eslint-disable-next-line quotes
        title: "Hacking | Cesar's cipher",
      },
      component: () => import('@/views/CesarCipherDecryptView.vue'),
    },
  ],
});

export default router;
