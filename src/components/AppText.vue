<script setup lang="ts">
export interface AppTextSlots {
  default?(): string;
}
</script>

<template>
  <p class="app-text">
    <slot />
  </p>
</template>

<style scoped lang="scss">
.app-text {
  margin: 0;
  padding: 0;
  font-size: 16px;
}
</style>
