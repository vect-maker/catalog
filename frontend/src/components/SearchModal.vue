<template>
  <button class="btn btn-ghost btn-circle" @click="showSearch = true" aria-label="Abrir búsqueda">
    <span class="material-symbols-outlined">search</span>
  </button>

  <BaseModal v-model="showSearch">
    <template #title>Buscar Productos</template>

    <div class="flex flex-col gap-4 h-full max-h-[85vh] sm:max-h-none">
      <div class="relative w-full">
        <input v-model.trim="searchQuery" type="text" inputmode="search" placeholder="Escribe para buscar..."
          class="input input-bordered w-full pr-12 text-base focus:input-primary"
          :class="{ 'input-primary': isSearching }" />

        <div class="absolute right-2 top-1/2 -translate-y-1/2 flex items-center gap-1">
          <button v-if="searchQuery.length > 0" @click="clearSearch"
            class="btn btn-ghost btn-circle btn-xs opacity-50 hover:opacity-100">
            <span class="material-symbols-outlined text-sm">close</span>
          </button>
          <span v-if="isSearching" class="loading loading-spinner loading-xs text-primary mr-2"></span>
        </div>
      </div>

      <div class="flex-1 overflow-y-auto rounded-xl border border-base-300 bg-base-200/30">
        <ul v-if="results.length > 0" class="menu w-full p-0">
          <li v-for="product in results" :key="product.id" class="border-b border-base-200 last:border-0">
            <button @click="handleProductClick(product.id)"
              class="flex items-center gap-4 py-4 px-4 active:bg-base-300 transition-colors h-20 cursor-pointer">
              <div class="w-14 h-14 aspect-square shrink-0 rounded-lg overflow-hidden bg-base-100 shadow-sm">
                <Image :src="product.images?.[0] ? getImageUrl(product.images[0]) : null" :alt="product.title" />
              </div>

              <div class="flex flex-col flex-1 min-w-0">
                <span class="font-bold text-sm sm:text-base truncate">{{ product.title }}</span>
                <span class="text-primary font-medium text-sm">C${{ product.price }}</span>
              </div>

              <span class="material-symbols-outlined opacity-20">chevron_right</span>
            </button>
          </li>
        </ul>

        <div v-else-if="searchQuery.length > 0 && !isSearching" class="py-12 text-center">
          <span class="material-symbols-outlined text-4xl opacity-20 block mb-2">search_off</span>
          <p class="text-sm opacity-50">No hay resultados para "{{ searchQuery }}"</p>
        </div>

        <div v-else class="py-12 text-center opacity-40 italic flex flex-col items-center">
          <span class="material-symbols-outlined text-4xl mb-2">find_in_page</span>
          <p class="text-sm">Busca por nombre de producto</p>
        </div>
      </div>
    </div>
  </BaseModal>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { getPaginatedProducts, getImageUrl } from '../api';
import BaseModal from './BaseModal.vue';
import Image from './Image.vue';
import type { Product } from '../api/schemas';
import { useRouter } from 'vue-router';

const showSearch = ref(false);
const searchQuery = ref('');
const results = ref<Product[]>([]);
const isSearching = ref(false);
const router = useRouter();

let debounceTimer: ReturnType<typeof setTimeout>;
let abortController: AbortController | null = null;

const performSearch = async () => {
  if (searchQuery.value.length === 0) return;

  if (abortController) abortController.abort();
  abortController = new AbortController();

  isSearching.value = true;
  try {
    const data = await getPaginatedProducts(0, searchQuery.value);
    results.value = data.items;
  } catch (err: any) {
    if (err.name === 'AbortError') return;
    console.error("Search failed:", err);
  } finally {
    isSearching.value = false;
  }
};

const clearSearch = () => {
  searchQuery.value = '';
  results.value = [];
  if (abortController) abortController.abort();
};

const handleProductClick = (product_id: string) => {
  showSearch.value = false
  clearSearch()
  router.push({ name: 'product', params: { product_id: product_id } })
}

watch(searchQuery, (newVal) => {
  clearTimeout(debounceTimer);

  if (newVal.length === 0) {
    results.value = [];
    isSearching.value = false;
    return;
  }

  isSearching.value = true;
  debounceTimer = setTimeout(performSearch, 1000);
});

watch(showSearch, (isOpen) => {
  if (!isOpen) {
    clearSearch();
    clearTimeout(debounceTimer);
  }
});


</script>