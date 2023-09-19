<script setup lang="ts">
import { computed } from 'vue';
import { RouterView, useRoute } from 'vue-router';

import AppReturnButton from '@/components/AppReturnButton.vue';

const route = useRoute();

const showReturnButton = computed(() => route.name !== 'home');
</script>

<template>
  <app-return-button v-if="showReturnButton" />
  <router-view />
</template>

<style scoped></style>
