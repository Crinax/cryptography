<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { invoke } from '@tauri-apps/api';

import AppText from '@/components/AppText.vue';
import AppHeader from '@/components/AppHeader.vue';
import AppInput from '@/components/AppInput.vue';
import AppSelect from '@/components/AppSelect.vue';
import AppButton from '@/components/AppButton.vue';
import type { AppSelectList } from '@/components/AppSelect.vue';

const alphabet = ref('');
const message = ref('');
const shift = ref('3');
const result = ref('');
const ignoreUnexistingSymbols = ref(false);
const alphabetType = ref('english-lower');

const alphabets: AppSelectList[] = [
  { key: 'english-lower', value: 'English alphabet (lower case)', default: true },
  { key: 'english-upper', value: 'English alphabet (uppers case)' },
  { key: 'english-both', value: 'English alphabet (both lower and upper cases)' },
  { key: 'russian-lower', value: 'Russian alphabet (lower case)' },
  { key: 'russian-upper', value: 'Russian alphabet (upper case)' },
  { key: 'russian-both', value: 'Russian alphabet (both lower and upper cases)' },
  { key: 'russian+english-lower', value: 'Russian and English alphabets (lower case)' },
  { key: 'russian+english-upper', value: 'Russian and English alphabets (upper case)' },
  {
    key: 'russian+english-both',
    value: 'Russian and English alphabets (both lower and upper cases)',
  },
];
const ignoreSymbols: AppSelectList[] = [
  { key: 'not-ignore', value: 'Do not ignore non-existent characters', default: true },
  { key: 'ignore', value: 'Ignore non-existent characters' },
];
const russianAlphabet = 'абвгдеёжзийклмнопрстуфхцчшщъыьэюя';
const englishAlphabet = 'abcdefghijklmnopqrstuvwxyz';

const formErrorResult = computed(() => {
  if (alphabet.value.length < 3) {
    return 'Alphabet should have 3 or more characters';
  }

  if (
    !ignoreUnexistingSymbols.value &&
    message.value.split('').some((item) => !alphabet.value.includes(item))
  ) {
    return 'Message containse symbols that alphabet have not';
  }

  return '';
});

const sendInvoke = async () => {
  if (formErrorResult.value.length === 0) {
    result.value = await invoke('cesar_solve', {
      alphabet: alphabet.value,
      message: message.value,
      shift: Number(shift.value),
      ignore: ignoreUnexistingSymbols.value,
    });
  }
};

const selectAlphabet = (key: string) => (alphabetType.value = key);
const selectIgnoring = (key: string) => (ignoreUnexistingSymbols.value = key === 'ignore');

const useAlphabet = () => {
  let alphabetForUsage = null;
  const [alphabetKind, caseType] = alphabetType.value.split('-');

  switch (alphabetKind) {
    case 'english': {
      alphabetForUsage = englishAlphabet;
      break;
    }
    case 'russian': {
      alphabetForUsage = russianAlphabet;
      break;
    }
    case 'russian+english': {
      alphabetForUsage = russianAlphabet + englishAlphabet;
      break;
    }
  }

  if (!alphabetForUsage) {
    return;
  }

  switch (caseType) {
    case 'upper': {
      alphabetForUsage = alphabetForUsage.toUpperCase();
      break;
    }
    case 'both': {
      alphabetForUsage = alphabetForUsage + alphabetForUsage.toUpperCase();
      break;
    }
  }

  alphabet.value = alphabetForUsage;
};

watch(message, sendInvoke);
watch(shift, sendInvoke);
watch(alphabet, sendInvoke);
</script>

<template>
  <div class="app-cesar-cipher">
    <div class="app-cesar-cipher__header">
      <app-header level="1">Cesar's cipher encryption / decryption</app-header>
    </div>

    <div class="app-cesar-cipher-manipulate">
      <app-select
        :list="alphabets"
        @select="selectAlphabet"
        class="app-cesar-cipher-manipulate__select"
      />
      <app-button @click="useAlphabet">Use alphabet</app-button>
    </div>

    <app-select
      :list="ignoreSymbols"
      @select="selectIgnoring"
      class="app-cesar-cipher-manipulate__select"
    />

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

        <app-input class="result-input" v-model="result" />
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.app-cesar-cipher {
  display: flex;
  flex-direction: column;
  gap: 8px;

  &-manipulate {
    display: flex;
    gap: 8px;

    &__select {
      width: 70%;
    }
  }

  &-content {
    display: flex;
    gap: 50px;

    &__form {
      @extend .app-cesar-cipher;
    }

    &__result {
      display: flex;
      flex-direction: column;
      gap: 8px;
      width: 100%;

      .result-input {
        width: 100%;
      }
    }
  }
}
</style>
