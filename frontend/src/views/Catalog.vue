<template>
  <div class="p-4 flex flex-col items-center gap-6 min-h-[60vh]">
    <p class="text-3xl font-bold mb-4">Catálogo</p>

    <div v-if="productStore.isLoading" class="flex flex-col items-center justify-center py-20 gap-4">
      <span class="loading loading-spinner loading-lg text-primary"></span>
      <p class="text-xl font-medium animate-pulse">Cargando productos...</p>
    </div>

    <template v-else-if="productStore.paginatedProducts?.items.length">
      <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 2xl:grid-cols-6 gap-6 w-full">
        <Product 
          v-for="product in productStore.paginatedProducts.items" 
          :key="product.id" 
          :product="product" 
        />
      </div>

      <div class="join mt-8">
        <button 
          class="join-item btn" 
          :disabled="productStore.paginatedProducts.page < 1"
          @click="setPage(productStore.paginatedProducts.page -1)"
        >«</button>
        <button class="join-item btn">
          Página {{ productStore.paginatedProducts.page + 1 }}
        </button>
        <button 
          class="join-item btn" 
          :disabled="productStore.paginatedProducts.is_last"
          @click="setPage(productStore.paginatedProducts.page +1)"
        >»</button>
      </div>
    </template>

    <div v-else class="flex flex-col items-center justify-center py-20 text-center gap-6">
      <div class="relative">
        <span class="material-symbols-outlined text-9xl opacity-10">inventory_2</span>
        <span class="material-symbols-outlined absolute bottom-2 right-2 text-4xl text-warning">search_off</span>
      </div>
      
      <div class="max-w-md">
        <h3 class="text-2xl font-bold">No se encontraron productos</h3>
        <p class="text-base-content/60 mt-2">
          Parece que no hay artículos registrados o has navegado a una página vacía.
        </p>
      </div>

      <button class="btn btn-outline" @click="setPage(0)">
        <span class="material-symbols-outlined">first_page</span>
        Ir a la primera página
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, watch } from 'vue';
import Product from '../components/Product.vue';
import { useProductStore } from '../stores/useProductStore';
import { useRoute, useRouter } from 'vue-router';

const productStore = useProductStore()
const route = useRoute();
const router = useRouter();

const currentPage = computed(() => {
  const p = route.query.p;
  return p ? parseInt(p as string) : 0;
});

const setPage = (newPage: number) => {
  router.push({
    query: {
      ...route.query,
      p: newPage.toString()
    }
  });
};

onMounted(async () => {
  await productStore.loadProducts(currentPage.value)
})

watch(
  () => route.query.p,
  (newPage) => {
    const pageNumber = parseInt(newPage as string) || 0;
    productStore.loadProducts(pageNumber);
  }
);
</script>