<script setup lang="ts">
import { RouterView } from 'vue-router';

import AppTitleBar from '@/components/AppTitleBar.vue';
</script>

<template>
  <app-title-bar />
  <div id="app-root">
    <div id="app-root__view">
      <router-view />
    </div>
  </div>
</template>

<style scoped lang="scss">
#app-root {
  width: 100vw;
  height: 100vh;
  box-shadow: 0px 0px 12px 0px #0003;
  gap: 12px;
  border-radius: 8px;
  border: 2px solid #fff3;
  overflow: hidden;
  position: relative;
  background-color: var(--color__background);

  &__view {
    padding: 8px;
    width: 100%;
    height: 100%;
    overflow: auto;
  }
}
</style>
