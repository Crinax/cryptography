<script setup lang="ts">
import { useRouter } from 'vue-router';
import AppArrowEndIcon from './icons/AppArrowEndIcon.vue';

const router = useRouter();
</script>

<template>
  <div class="app-return-button" @click="router.back()" role="button">
    <app-arrow-end-icon fill="#cecece" />
    <p class="app-return-button__text">Back</p>
  </div>
</template>

<style lang="scss">
.app-return-button {
  display: flex;
  gap: 8px;
  align-items: center;
  width: 100%;
  background-color: var(--color__background);
  color: var(--color__text);
  padding: 16px 8px;
  margin-bottom: 16px;
  border-radius: 8px;

  &:hover {
    filter: brightness(80%);
  }

  &__text {
    margin: 0;
    padding: 0;
  }
}
</style>
