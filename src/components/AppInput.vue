<script setup lang="ts">
import { computed, ref } from 'vue';

export interface AppInputSlots {
  label?(): any;
}

export interface AppInputModifiers {
  upper?: boolean;
  lower?: boolean;
  capitalize?: boolean;
  trim?: boolean;
}

export interface AppInputProps {
  placeholder?: string;
  modelValue?: string;
  modelModifiers?: AppInputModifiers;
}

export interface AppInputEmits {
  (e: 'update:modelValue', value: string): void;
}

const slots = defineSlots<AppInputSlots>();
const emit = defineEmits<AppInputEmits>();

const props = withDefaults(defineProps<AppInputProps>(), {
  modelModifiers: () => ({}),
});

const inputText = ref(props.modelValue ?? '');
const inputElement = ref<HTMLInputElement | null>(null);
const showPlaceholder = computed(
  () => inputText.value.length === 0 && props.placeholder !== undefined,
);

const focusOnInput = () => {
  if (inputElement.value) {
    inputElement.value.focus();
  }
};

const updateModelValue = (event: Event) => {
  const target = event.target as HTMLInputElement;

  inputText.value = target.value;

  if (props.modelModifiers.trim) {
    inputText.value = inputText.value.trim();
  }

  if (props.modelModifiers.lower) {
    inputText.value = inputText.value.toLowerCase();
  }

  if (props.modelModifiers.upper) {
    inputText.value = inputText.value.toUpperCase();
  }

  if (props.modelModifiers.capitalize) {
    inputText.value = inputText.value[0].toUpperCase() + inputText.value.slice(1);
  }

  emit('update:modelValue', inputText.value);
};
</script>

<template>
  <div class="app-input">
    <label v-if="slots.label" class="app-input__label" @click="focusOnInput">
      <slot name="label" />
    </label>

    <div class="app-input-text-field" @click="focusOnInput">
      <div class="app-input-text-field__wrapper">
        <p v-show="showPlaceholder" class="app-input-text-field__placeholder">
          {{ props.placeholder }}
        </p>
        <input
          ref="inputElement"
          class="app-input-text-field__text"
          :value="inputText"
          @input="updateModelValue"
        />
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.app-input {
  display: flex;
  flex-direction: column;
  gap: 4px;
  width: max-content;

  &-text-field {
    padding: 8px 12px;
    background: var(--color__input);
    border-radius: 8px;
    border: 2px solid #eee2;
    cursor: text;

    &__wrapper {
      position: relative;
    }

    &__placeholder {
      position: absolute;
      top: 0;
      left: 0;
      margin: 0;
      padding: 0;
      opacity: 0.25;
      font-size: 16px;
    }

    &__text {
      width: 100%;
      color: var(--color__text);
      border: none;
      outline: none;
      background: transparent;
      font-size: 16px;
    }
  }
}
</style>