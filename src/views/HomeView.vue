<script setup lang="ts">
import { RouterLink } from 'vue-router';
import { useAppSettinsStore } from '@/stores/app-settings';

const appSettings = useAppSettinsStore();
</script>

<template>
  <main>
    <router-link v-if="appSettings.isDevMode" :to="{ name: 'components' }">Компоненты</router-link>
  </main>
</template>
