<script setup lang="ts">
import { ref } from 'vue';
import AppButton from '@/components/AppButton.vue';
import AppText from '@/components/AppText.vue';

const textComponent = ref('');
const clickEvent = (text: string) => (textComponent.value = text);
</script>

<template>
  <div class="app-components simple-text">
    <AppText>Текст: {{ textComponent }}</AppText>
  </div>
  <div class="app-components buttons">
    <AppButton kind="primary" @click="clickEvent('Primary button')">Primary button</AppButton>
    <AppButton kind="success" @click="clickEvent('Success button')">Success button</AppButton>
    <AppButton kind="accent" @click="clickEvent('Accent button')">Accent button</AppButton>
    <AppButton kind="error" @click="clickEvent('Error button')">Error button</AppButton>
    <AppButton kind="info" @click="clickEvent('Info button')">Info button</AppButton>
  </div>
  <div class="app-components buttons">
    <AppButton kind="primary" @click="clickEvent('Primary button')" block>Primary button</AppButton>
    <AppButton kind="success" @click="clickEvent('Success button')" block>Success button</AppButton>
    <AppButton kind="accent" @click="clickEvent('Accent button')" block>Accent button</AppButton>
    <AppButton kind="error" @click="clickEvent('Error button')" block>Error button</AppButton>
    <AppButton kind="info" @click="clickEvent('Info button')" block>Info button</AppButton>
  </div>
</template>

<style scoped>
.app-components.buttons {
  display: flex;
  gap: 20px;
  margin-bottom: 8px;
}

.app-components.simple-text {
  margin-bottom: 4px;
}
</style>
