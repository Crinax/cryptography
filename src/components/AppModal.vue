<script setup lang="ts">
import { computed } from 'vue';

export interface AppModalSlots {
  default?(): any;
}

export interface AppModalEmits {
  (e: 'close'): void;
}

export interface AppModalProps {
  kind?: 'primary' | 'success' | 'accent' | 'error' | 'info';
}

defineSlots<AppModalSlots>();
const emit = defineEmits<AppModalEmits>();
const props = defineProps<AppModalProps>();

const modalClasses = computed(() => ({
  'app-modal-window': true,
  [`app-modal-window__${props.kind}`]: props.kind !== 'primary',
}));

const closeModal = () => emit('close');
</script>

<template>
  <Teleport to="body">
    <div class="app-modal" @click.self="closeModal">
      <div :class="modalClasses">
        <slot />
      </div>
    </div>
  </Teleport>
</template>

<style scoped lang="scss">
@import '@/assets/mixins';

.app-modal {
  position: absolute;
  z-index: 50;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: var(--color__modal-fog);

  @include backdrop-filter(blur(5px));

  &-window {
    padding: 12px 16px;
    border-radius: 8px;
    background-color: var(--color__background);
    border: 2px solid #fff1;
  }
}
</style>
