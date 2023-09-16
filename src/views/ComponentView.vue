<script setup lang="ts">
import AppButton from '@/components/AppButton.vue';
</script>

<template>
  <div class="app-components buttons">
    <AppButton kind="primary">Primary button</AppButton>
    <AppButton kind="success">Success button</AppButton>
    <AppButton kind="accent">Accent button</AppButton>
    <AppButton kind="error">Error button</AppButton>
    <AppButton kind="info">Info button</AppButton>
  </div>
  <div class="app-components buttons">
    <AppButton kind="primary" block>Primary button</AppButton>
    <AppButton kind="success" block>Success button</AppButton>
    <AppButton kind="accent" block>Accent button</AppButton>
    <AppButton kind="error" block>Error button</AppButton>
    <AppButton kind="info" block>Info button</AppButton>
  </div>
</template>

<style scoped>
.app-components.buttons {
  display: flex;
  gap: 20px;
  margin-bottom: 8px;
}
</style>
