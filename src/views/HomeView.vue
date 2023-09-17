<script setup lang="ts">
import { RouterLink } from 'vue-router';
import { useAppSettinsStore } from '@/stores/app-settings';

const appSettings = useAppSettinsStore();
</script>

<template>
  <main class="app-main">
    <router-link v-if="appSettings.isDevMode" :to="{ name: 'components' }">Компоненты</router-link>
    <router-link :to="{ name: 'cesarCipher' }">Шифр Цезаря</router-link>
  </main>
</template>

<style scoped lang="scss">
.app-main {
  display: flex;
  flex-direction: column;
  gap: 8px;
}
</style>
