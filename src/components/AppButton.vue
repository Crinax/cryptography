<script setup lang="ts">
import { computed } from 'vue';

export interface AppButtonProps {
  kind?: 'primary' | 'success' | 'accent' | 'error' | 'info';
  block?: boolean;
}

export interface AppButtonSlots {
  icon?(): any;
}

const props = withDefaults(defineProps<AppButtonProps>(), {
  kind: 'primary',
  block: false,
});

const buttonClasses = computed(() => {
  const blockClass = props.block ? ['app-button__block'] : [];
  if (props.kind === 'primary') {
    return ['app-button', ...blockClass];
  }

  return ['app-button', `app-button__${props.kind}`, ...blockClass];
});
const slots = defineSlots<AppButtonSlots>();
</script>

<template>
  <div :class="buttonClasses" role="button">
    <div class="app-button-icon" v-if="slots.icon">
      <slot name="icon" />
    </div>
    <div class="app-button-text">
      <slot />
    </div>
  </div>
</template>

<style scoped lang="scss">
.app-button {
  width: max-content;
  padding: 8px 12px;
  border-radius: 8px;
  background-color: var(--color__primary);
  box-shadow: 0px 0px 6px 0px #00000075;
  display: flex;
  gap: 4px;
  font-size: 16px;
  transition: 0.1s;

  &-icon {
    max-width: 12px;
    max-height: 12px;
  }

  &:active {
    box-shadow: 0px 0px 0px 0px #00000075;
  }

  &:hover {
    filter: brightness(120%);
  }

  &__success {
    background-color: var(--color__success);
  }

  &__accent {
    background-color: var(--color__accent);
  }

  &__error {
    background-color: var(--color__error);
  }

  &__info {
    background-color: var(--color__info);
  }

  &__block {
    filter: brightness(80%);
    box-shadow: none;
    cursor: not-allowed;

    &:hover {
      filter: brightness(80%);
    }
  }
}
</style>
