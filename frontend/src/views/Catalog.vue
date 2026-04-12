<template>
  <div class="p-4 flex flex-col items-center gap-6">

    <p class="text-3xl font-bold mb-4">Catalogo</p>
    <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 2xl:grid-cols-6 gap-6">
      <Product v-for="product in productStore.paginatedProducts?.items" :key="product.id" :product="product"></Product>
    </div>
    <div class="join" v-if="productStore.paginatedProducts">
      <button class="join-item btn" :disabled="productStore.paginatedProducts.page < 1"
        @click="productStore.previousProducts()">«</button>
      <button class="join-item btn">Pagina {{ productStore.paginatedProducts.page + 1 }}</button>
      <button class="join-item btn" :disabled="productStore.paginatedProducts.is_last"
        @click="productStore.nextProducts()">»</button>
    </div>
  </div>

</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import Product from '../components/Product.vue';
import { useProductStore } from '../stores/useProductStore';


const productStore = useProductStore()


onMounted(async () => {
  await productStore.loadProducts()
})
</script>