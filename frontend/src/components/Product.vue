<template>
  <article class="card bg-base-100 shadow-sm group hover:shadow-md transition-all duration-300">
    <div class="w-full aspect-square bg-base-200 relative overflow-hidden rounded-t-xl">
      <button type="button"
        class="w-full h-full p-0 border-0 bg-transparent cursor-pointer focus:outline-none focus-visible:ring-4 focus-visible:ring-primary focus-visible:z-20"
        :aria-label="`Ver galería de imágenes de ${product.title}`"
        @click.stop.prevent="imageStore.showImages(images, imageIndex)">
        <Image :src="currentImage" :alt="product.title" />
      </button>

      <div v-if="images.length > 1"
        class="absolute z-10 left-2 right-2 top-1/2 flex -translate-y-1/2 justify-between pointer-events-none opacity-0 group-hover:opacity-100 transition-opacity">
        <button type="button" :disabled="imageIndex === 0" @click.stop.prevent="imageIndex--"
          class="btn btn-circle btn-xs sm:btn-sm shadow-md pointer-events-auto bg-base-100/90 border-none disabled:invisible">
          <span class="material-symbols-outlined text-base" aria-hidden="true">arrow_left</span>
        </button>

        <button type="button" :disabled="imageIndex === images.length - 1" @click.stop.prevent="imageIndex++"
          class="btn btn-circle btn-xs sm:btn-sm shadow-md pointer-events-auto bg-base-100/90 border-none disabled:invisible">
          <span class="material-symbols-outlined text-base" aria-hidden="true">arrow_right</span>
        </button>
      </div>
    </div>

    <RouterLink :to="{ name: 'product', params: { product_id: product.id } }" class="card-body p-4 cursor-pointer">
      <h2 class="card-title text-sm sm:text-base line-clamp-1 group-hover:text-primary transition-colors">
        {{ product.title }}
      </h2>
      <p class="text-xs opacity-60 line-clamp-2 h-8">{{ product.description }}</p>
      
      <div class="flex justify-between items-center mt-2">
        <span class="text-lg font-bold text-primary">C${{ product.price }}</span>
        <span class="material-symbols-outlined opacity-0 group-hover:opacity-100 group-hover:translate-x-1 transition-all text-primary">
          arrow_forward
        </span>
      </div>
    </RouterLink>
  </article>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { getImageUrl } from '../api';
import { useImageStore } from '../stores/useImageStore';
import type { Product } from '../api/schemas';
import Image from './Image.vue';

const props = defineProps<{
  product: Product
}>()

const imageStore = useImageStore()

const images = props.product.images?.map(getImageUrl) || [];
const imageIndex = ref(0);

const currentImage = computed(() => {
  return images.length < 1 ? "/missingImage.jpg" : images[imageIndex.value];
});
</script>