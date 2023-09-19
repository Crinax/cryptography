<script setup lang="ts">
import { appWindow } from '@tauri-apps/api/window';
import { useRouter, useRoute } from 'vue-router';
import { computed } from 'vue';
import AppCrossIcon from './icons/AppCrossIcon.vue';
import AppArrowEndIcon from './icons/AppArrowEndIcon.vue';
import AppText from './AppText.vue';

const router = useRouter();
const route = useRoute();

const isHome = computed(() => route.name === 'home');

const closeWindow = () => appWindow.close();
</script>

<template>
  <div data-tauri-drag-region class="titlebar">
    <div class="__meta-tag">
      <app-arrow-end-icon
        v-show="!isHome"
        class="titlebar-button-back"
        fill="#fff5"
        role="button"
        @click="router.back()"
      />
    </div>
    <app-text data-tauri-drag-region class="titlebar-header">
      {{ route.meta.title || 'Cryptography' }}
    </app-text>
    <app-cross-icon class="titlebar-button" fill="#fff5" role="button" @click="closeWindow" />
  </div>
</template>

<style scoped lang="scss">
.__meta-tag {
  height: 24px;
  width: 24px;
  background: transparent;
  border-radius: 8px;
}

.titlebar {
  display: flex;
  gap: 12px;
  padding: 4px 4px;
  border-radius: 8px;
  background-color: var(--color__background);
  border: 2px solid #fff3;
  display: flex;
  align-items: center;

  &-header {
    flex-grow: 1;
    text-align: center;
    cursor: default;
  }

  &-button {
    border-radius: 100%;
    width: 24px;
    height: 24px;
    margin-left: auto;
    background-color: var(--color__background);

    &-back {
      border-radius: 100%;
      height: 24px;
      background-color: var(--color__background);

      &:hover {
        filter: brightness(160%);
      }
    }

    &:hover {
      filter: brightness(160%);
    }
  }
}
</style>
