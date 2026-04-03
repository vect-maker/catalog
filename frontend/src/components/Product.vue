<template>


  <div class="card bg-base-100 shadow-sm group">

    <div v-if="image_url" class="rounded relative w-full">
      <img v-show="isImageLoaded" @load="isImageLoaded = true" :src="image_url" :alt="title"
        class=" w-full h-full object-cover" />
      <div v-show="!isImageLoaded" class="skeleton w-full h-60 object-cover "></div>
      <div class="absolute left-5 right-5 top-1/2 flex -translate-y-1/2 transform justify-between">
        <button :class="{ 'invisible': false }"  class="btn btn-circle">❮</button>
        <button :class="{ 'invisible': false}"
          class="btn btn-circle">❯</button>
      </div>
    </div>
    <img v-else src="../assets/missingImage.jpg" alt="">
    <div class="card-body">
      <h2 class="card-title text-left">{{ title }}</h2>
      <p>{{ description }}</p>
      <p class="text-xl font-semibold text-right">C${{ price }}</p>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue';


const props = defineProps<{
  title: string,
  description: string,
  image_url: string | null,
  price: number
}>()

const isImageLoaded = ref(false);
</script>