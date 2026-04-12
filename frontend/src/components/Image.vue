<template>
  <div class="relative w-full h-full overflow-hidden">
    <img
      v-show="isLoaded"
      :src="src || fallback"
      :alt="alt"
      class="w-full h-full object-cover transition-opacity duration-300"
      @load="isLoaded = true"
      @error="handleError"
    />

    <div
      v-if="!isLoaded"
      class="skeleton absolute inset-0 w-full h-full z-0"
      aria-hidden="true"
    ></div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';

const props = defineProps<{
  src: string | undefined | null;
  alt: string;
  fallback?: string;
}>();

const isLoaded = ref(false);
const fallback = props.fallback ?? "/missingImage.jpg";

watch(() => props.src, () => {
  isLoaded.value = false;
});

const handleError = () => {
  isLoaded.value = true; 
};
</script>