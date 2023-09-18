<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api';

import AppHeader from '@/components/AppHeader.vue';
import AppInput from '@/components/AppInput.vue';
import AppButton from '@/components/AppButton.vue';
import AppSelect, { type AppSelectList } from '@/components/AppSelect.vue';

const encryptedText = ref('');
const decryptionKind = ref('brute-force');
const decryptionResult = ref('');
const decryptionRunning = ref(false);
const decryptionKinds: AppSelectList[] = [
  { key: 'brute_force', value: 'Brute force decryption', default: true },
  { key: 'frequency_anlysis', value: 'Frequency analysis' },
];

const selectDecryptionKind = (key: string) => (decryptionKind.value = key);
const decryptMessage = async () => {
  decryptionRunning.value = true;
  decryptionResult.value = await invoke(decryptionKind.value, {
    message: encryptedText.value,
  });
  decryptionRunning.value = false;
};
</script>

<template>
  <div class="app-cesar-cipher-decription">
    <app-header>Cesar's cipher decription</app-header>
    <app-input v-model="encryptedText" class="app-cesar-cipher-decription__encrypted-text">
      <template #label>Enter the encrypted text</template>
    </app-input>
    <div class="app-cesar-cipher-decription__select-decryption">
      <app-select
        class="select-decryption__select"
        :list="decryptionKinds"
        @select="selectDecryptionKind"
      />
      <app-button
        class="select-decryption__button"
        :block="decryptionRunning"
        @click="decryptMessage"
      >
        Decrypt message
      </app-button>
    </div>
  </div>
</template>

<style scoped lang="scss">
.app-cesar-cipher-decription {
  display: flex;
  flex-direction: column;
  gap: 8px;

  &__encrypted-text {
    width: 100%;
  }

  &__select-decryption {
    display: flex;
    gap: 8px;

    .select-decryption__button {
      flex-shrink: 0;
    }

    .select-decryption__select {
      flex-grow: 1;
    }
  }
}
</style>
