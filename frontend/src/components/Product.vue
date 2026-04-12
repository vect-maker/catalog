<template>
  <div class="card bg-base-100 shadow-sm group">

    <div class="w-full h-64 bg-base-200 rounded overflow-hidden flex items-center justify-center">

      <img v-show="isImageLoaded" @load="isImageLoaded = true" @error="isImageLoaded = true" :src="currentImage"
        :alt="product.title" class="w-full h-full object-contain hover:cursor-pointer"
        @click="imageStore.showImages(images, imageIndex)" />

      <div v-show="!isImageLoaded" class="skeleton w-full h-full top-0 left-0"></div>

      <div v-if="images.length > 1" class="absolute z-10 left-5 right-5 top-1/2 flex -translate-y-1/2  justify-between">

        <button type="button" :class="{ 'invisible': imageIndex <= 0 }" @click.stop.prevent="imageIndex--"
          class="btn btn-circle ">
          <span class="material-symbols-outlined">
            arrow_left
          </span>
        </button>

        <button type="button" :class="{ 'invisible': imageIndex >= images.length - 1 }"
          @click.stop.prevent="imageIndex++" class="btn btn-circle">

          <span class="material-symbols-outlined">
            arrow_right
          </span>
        </button>

      </div>
    </div>
    <div class="card-body">
      <h2 class="card-title text-left">{{ product.title }}</h2>
      <p>{{ product.description }}</p>
      <p class="text-xl font-semibold text-right">C${{ product.price }}</p>
      <div class="card-actions justify-end mt-4">
        <a :href="whatsappLink" target="_blank" rel="noopener noreferrer" class="btn btn-success wa-button">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="w-5 h-5 fill-current mr-2">
            <path
              d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51-.173-.008-.371-.01-.57-.01-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 01-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 01-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 012.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0012.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 005.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 00-3.48-8.413z" />
          </svg>
          Preguntar por WhatsApp
        </a>

      </div>
      <div v-if="authStore.isAuthenticated" class="card-actions justify-end mt-4">

        <button type="button" class="btn btn-warning" @click.stop.prevent="">
          <span class="material-symbols-outlined">edit</span>
          Editar
        </button>
        <button type="button" class="btn btn-error" @click.stop.prevent="deleteProduct(product.id)">
          <span class="material-symbols-outlined">delete</span>
          Borrar
        </button>
      </div>

    </div>

  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { getImageUrl, type Product } from '../api';
import { useAuthStore } from '../stores/useAuthStore';
import { useProductStore } from '../stores/useProductStore';
import { useAlertStore } from '../stores/useAlertStore';
import { useImageStore } from '../stores/useImageStore';


const props = defineProps<{
  product: Product
}>()

const authStore = useAuthStore()
const productStore = useProductStore()
const alertStore = useAlertStore()
const imageStore = useImageStore()


const images = props.product.images?.map(getImageUrl) || [];
const imageIndex = ref(0);
const isImageLoaded = ref(false);

const deleteProduct = (id: string) => {
  productStore.deleteProduct(id)
  alertStore.pushAlert(`Se borro producto con id(${id})`)
}

const currentImage = computed(() => {
  return images.length < 1 ? "/missingImage.jpg" : images[imageIndex.value];
});

// whatsapp 
const storePhone = import.meta.env.VITE_STORE_PHONE_NUMBER;


const whatsappLink = computed(() => {
  const rawMessage = `¡Hola! Tengo una pregunta sobre el producto: ${props.product.title} (SKU: ${props.product.id}). ¿Está disponible?`;
  const encodedMessage = encodeURIComponent(rawMessage);

  return `https://wa.me/${storePhone}?text=${encodedMessage}`;
});

watch(currentImage, () => {
  isImageLoaded.value = false;
});
</script>