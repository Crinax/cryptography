<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { invoke } from '@tauri-apps/api';

import AppText from '@/components/AppText.vue';
import AppInput from '@/components/AppInput.vue';

const alphabet = ref('');
const message = ref('');
const shift = ref('3');
const result = ref('');

const formErrorResult = computed(() => {
  if (Number(shift.value) < 0) {
    return 'Shift should be a positive number';
  }

  if (alphabet.value.length < 3) {
    return 'Alphabet should have 3 symbols or more';
  }

  if (message.value.split('').some((item) => !alphabet.value.includes(item))) {
    return 'Message containse symbols that alphabet have not';
  }

  return '';
});

watch(formErrorResult, async (value) => {
  if (value.length === 0) {
    result.value = await invoke('');
  }
});
</script>

<template>
  <div class="app-cesar-cipher">
    <div class="app-cesar-cipher__header">
      <app-text>Cesar's cipher</app-text>
    </div>

    <div class="app-cesar-cipher-content">
      <div class="app-cesar-cipher-content__form">
        <app-input v-model.uniqueSymbols="alphabet">
          <template #label>
            <app-text>Enter alphabet</app-text>
          </template>
        </app-input>

        <app-input v-model="message">
          <template #label>
            <app-text>Enter message</app-text>
          </template>
        </app-input>

        <app-input v-model.number="shift">
          <template #label>
            <app-text>Enter shift</app-text>
          </template>
        </app-input>
      </div>

      <div class="app-cesar-cipher-content__result">
        <app-text>Result:</app-text>

        <app-text v-if="formErrorResult.length > 0" kind="error">
          {{ formErrorResult }}
        </app-text>

        <app-text>{{ result }}</app-text>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.app-cesar-cipher {
  display: flex;
  flex-direction: column;
  gap: 8px;

  &-content {
    display: flex;
    gap: 50px;

    &__form {
      @extend .app-cesar-cipher;
    }
  }
}
</style>
