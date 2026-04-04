<template>
  <div class="card bg-base-100 shadow-sm group">
    <div class="rounded relative w-full h-60">

      <img v-show="isImageLoaded" @load="isImageLoaded = true" @error="isImageLoaded = true" :src="currentImage"
        :alt="product.title" class="w-full h-full object-cover absolute top-0 left-0" />

      <div v-show="!isImageLoaded" class="skeleton w-full h-full absolute top-0 left-0"></div>

      <div v-if="images.length > 1"
        class="absolute z-10 left-5 right-5 top-1/2 flex -translate-y-1/2 transform justify-between">

        <button type="button" :class="{ 'invisible': imageIndex <= 0 }" @click.stop.prevent="imageIndex--"
          class="btn btn-circle ">
          ❮
        </button>

        <button type="button" :class="{ 'invisible': imageIndex >= images.length - 1 }"
          @click.stop.prevent=" imageIndex++" class="btn btn-circle">
          ❯
        </button>

      </div>
    </div>

    <div class="card-body">
      <h2 class="card-title text-left">{{ product.title }}</h2>
      <p>{{ product.description }}</p>
      <p class="text-xl font-semibold text-right">C${{ product.price }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { getImageUrl, type Product } from '../api';

const props = defineProps<{
  product: Product
}>()

const images = props.product.images?.map(getImageUrl) || [];
const imageIndex = ref(0);
const isImageLoaded = ref(false);

const currentImage = computed(() => {
  return images.length < 1 ? "/missingImage.jpg" : images[imageIndex.value];
});

watch(currentImage, () => {
  isImageLoaded.value = false;
});
</script>