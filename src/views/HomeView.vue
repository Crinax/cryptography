<script setup lang="ts">
import { RouterLink } from 'vue-router';
import { useAppSettinsStore } from '@/stores/app-settings';

const appSettings = useAppSettinsStore();
</script>

<template>
  <main class="app-main">
    <router-link v-if="appSettings.isDevMode" :to="{ name: 'components' }">Components</router-link>
    <router-link :to="{ name: 'cesarCipher' }">Cesar's cipher</router-link>
    <router-link :to="{ name: 'cesarCipherDecryption' }">Cesar's cipher decryption</router-link>
  </main>
</template>

<style scoped lang="scss">
.app-main {
  display: flex;
  flex-direction: column;
  gap: 8px;
}
</style>
