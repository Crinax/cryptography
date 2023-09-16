<script setup lang="ts">
import { ref } from 'vue';
import AppButton from '@/components/AppButton.vue';
import AppText from '@/components/AppText.vue';
import AppInput from '@/components/AppInput.vue';

const textComponent = ref('');
const inputText = ref('');
const clickEvent = (text: string) => (textComponent.value = text);
</script>

<template>
  <div class="app-components simple-text">
    <app-text kind="primary">Default text: {{ textComponent }}</app-text>
    <app-text kind="success">Success text: {{ textComponent }}</app-text>
    <app-text kind="accent">Accent text: {{ textComponent }}</app-text>
    <app-text kind="error">Error text: {{ textComponent }}</app-text>
    <app-text kind="info">Info text: {{ textComponent }}</app-text>
  </div>

  <div class="app-components buttons">
    <app-button kind="primary" @click="clickEvent('Primary button')">Primary button</app-button>
    <app-button kind="success" @click="clickEvent('Success button')">Success button</app-button>
    <app-button kind="accent" @click="clickEvent('Accent button')">Accent button</app-button>
    <app-button kind="error" @click="clickEvent('Error button')">Error button</app-button>
    <app-button kind="info" @click="clickEvent('Info button')">Info button</app-button>
  </div>

  <div class="app-components buttons">
    <app-button kind="primary" @click="clickEvent('Primary button')" block>
      Primary button
    </app-button>
    <app-button kind="success" @click="clickEvent('Success button')" block>
      Success button
    </app-button>
    <app-button kind="accent" @click="clickEvent('Accent button')" block>
      Accent button
    </app-button>
    <app-button kind="error" @click="clickEvent('Error button')" block>Error button</app-button>
    <app-button kind="info" @click="clickEvent('Info button')" block>Info button</app-button>
  </div>

  <div class="app-components input">
    <app-input v-model="inputText" placeholder="Enter a text">
      <template #label>
        {{ inputText }}
      </template>
    </app-input>
  </div>
</template>

<style scoped>
.app-components {
  margin-bottom: 8px;
}

.app-components.buttons {
  display: flex;
  gap: 20px;
}
</style>
