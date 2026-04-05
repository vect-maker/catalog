<template>

  <Navbar class="fixed top-0 w-full z-50 "></Navbar>
  <RouterView class="mt-22"></RouterView>
  <AlertBox></AlertBox>
   <VueEasyLightbox
      :visible="imageStore.showLightbox"
      :imgs="imageStore.images"
      :index="imageStore.imageIndex"
      @hide="imageStore.showLightbox = false"
    />

</template>

<script setup lang="ts">

import VueEasyLightbox from 'vue-easy-lightbox'
import AlertBox from './components/AlertBox.vue';
import Navbar from './components/Navbar.vue';
import { useImageStore } from './stores/useImageStore';



const imageStore = useImageStore();
</script>
