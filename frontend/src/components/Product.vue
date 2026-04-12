<template>
  <article class="card bg-base-100 shadow-sm group">
    <div class="w-full aspect-square bg-base-200 relative overflow-hidden">
      <button type="button"
        class="w-full h-full p-0 border-0 bg-transparent cursor-pointer focus:outline-none focus-visible:ring-4 focus-visible:ring-primary focus-visible:z-20"
        :aria-label="`Ver galería de imágenes de ${product.title}`"
        @click.stop.prevent="imageStore.showImages(images, imageIndex)">
        
        <Image 
          :src="currentImage" 
          :alt="product.title" 
        />
      </button>

      <div v-if="images.length > 1"
        class="absolute z-10 left-2 right-2 top-1/2 flex -translate-y-1/2 justify-between pointer-events-none">
        
        <button type="button" :disabled="imageIndex === 0" @click.stop.prevent="imageIndex--"
          aria-label="Ver imagen anterior"
          class="btn btn-circle btn-sm shadow-md pointer-events-auto bg-base-100/80 hover:bg-base-100 border-none disabled:opacity-0 disabled:cursor-default focus-visible:ring-2 focus-visible:ring-primary">
          <span class="material-symbols-outlined" aria-hidden="true">arrow_left</span>
        </button>

        <button type="button" :disabled="imageIndex === images.length - 1" @click.stop.prevent="imageIndex++"
          aria-label="Ver siguiente imagen"
          class="btn btn-circle btn-sm shadow-md pointer-events-auto bg-base-100/80 hover:bg-base-100 border-none disabled:opacity-0 disabled:cursor-default focus-visible:ring-2 focus-visible:ring-primary">
          <span class="material-symbols-outlined" aria-hidden="true">arrow_right</span>
        </button>
      </div>
    </div>

    <div class="card-body">
      <h2 class="card-title text-left">{{ product.title }}</h2>
      <p class="line-clamp-3">{{ product.description }}</p>
      <p class="text-xl font-semibold text-right" aria-label="Precio:">C${{ product.price }}</p>

      <div class="card-actions justify-end mt-4">
        <a :href="whatsappLink" target="_blank" rel="noopener noreferrer" class="btn btn-success wa-button text-white">
          Preguntar
        </a>
      </div>

      <div v-if="authStore.isAuthenticated" class="card-actions justify-end mt-2">
        <button type="button" class="btn btn-warning btn-sm" @click.stop.prevent="">
          <span class="material-symbols-outlined text-sm">edit</span>
        </button>
        <button type="button" class="btn btn-error btn-sm" @click.stop.prevent="deleteProduct(product.id)">
          <span class="material-symbols-outlined text-sm">delete</span>
        </button>
      </div>
    </div>
  </article>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { getImageUrl } from '../api';
import { useAuthStore } from '../stores/useAuthStore';
import { useProductStore } from '../stores/useProductStore';
import { useAlertStore } from '../stores/useAlertStore';
import { useImageStore } from '../stores/useImageStore';
import type { Product } from '../api/schemas';
import Image from './Image.vue';

const props = defineProps<{
  product: Product
}>()

const authStore = useAuthStore()
const productStore = useProductStore()
const alertStore = useAlertStore()
const imageStore = useImageStore()

const images = props.product.images?.map(getImageUrl) || [];
const imageIndex = ref(0);

const currentImage = computed(() => {
  return images.length < 1 ? "/missingImage.jpg" : images[imageIndex.value];
});

const deleteProduct = (id: string) => {
  productStore.deleteProduct(id)
  alertStore.pushAlert(`Se borro producto con id(${id})`)
}

// WhatsApp logic
const storePhone = import.meta.env.VITE_STORE_PHONE_NUMBER;
const whatsappLink = computed(() => {
  const rawMessage = `¡Hola! Tengo una pregunta sobre el producto: ${props.product.title} (SKU: ${props.product.id}). ¿Está disponible?`;
  return `https://wa.me/${storePhone}?text=${encodeURIComponent(rawMessage)}`;
});
</script>