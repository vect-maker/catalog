<template>
  <div v-if="!backendStore.isInitialized"
    class="h-screen w-screen flex flex-col items-center justify-center bg-base-100">
    <span class="loading loading-infinity loading-lg text-primary"></span>
    <p class="mt-4 text-lg font-medium animate-pulse">Estableciendo conexión con el servidor...</p>
  </div>

  <div v-else-if="!backendStore.isBackendReady"
    class="h-screen w-screen flex flex-col items-center justify-center bg-base-200 p-6 text-center">
    <div class="max-w-md flex flex-col items-center gap-6">
      <span class="material-symbols-outlined  opacity-20"
        style="font-size: 128px; width: 128px; height: 128px;">
        cloud_off
      </span>
      <h1 class="text-3xl font-bold">Servidor no disponible</h1>
      <p class="text-base-content/70">
        No pudimos conectarnos con el sistema de backend. Esto puede deberse a mantenimiento o problemas de red.
        Por favor, intenta recargar la página en unos minutos.
      </p>
      <button class="btn btn-primary px-8" @click="reloadPage">
        <span class="material-symbols-outlined">refresh</span>
        Recargar ahora
      </button>
    </div>
  </div>

  <template v-else>
    <Navbar class="fixed top-0 w-full z-50" />
    <main class="mt-22">
      <RouterView />
    </main>
    <NotificationTrail />
    <VueEasyLightbox :visible="imageStore.showLightbox" :imgs="imageStore.images" :index="imageStore.imageIndex"
      @hide="imageStore.showLightbox = false" />
  </template>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';
import VueEasyLightbox from 'vue-easy-lightbox';
import Navbar from './components/Navbar.vue';
import NotificationTrail from './components/NotificationTrail.vue';
import { useImageStore } from './stores/useImageStore';
import { useBackendStore } from './stores/useBackendStore';

const backendStore = useBackendStore();
const imageStore = useImageStore();

const reloadPage = () => window.location.reload();

const handleVisibilityChange = () => {
  if (document.visibilityState === 'visible') {
    backendStore.startHeartbeat();
  } else {
    backendStore.stopHeartbeat();
  }
};

onMounted(() => {
  backendStore.startHeartbeat();
  document.addEventListener('visibilitychange', handleVisibilityChange);
});

onUnmounted(() => {
  backendStore.stopHeartbeat();
  document.removeEventListener('visibilitychange', handleVisibilityChange);
});
</script>