<template>
<div class="p-4">
    <p class="text-3xl font-bold mb-4">Catalogo</p>
    <div class="flex items-center justify-center">
      <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 xl:grid-cols-4 gap-4">
        <Product v-for="product in products" :title="product.title" :description="product.description"
          :image_url="product.image_id ? getImageUrl(product.image_id) : null" :price="product.price"></Product>
      </div>



    </div>
  </div>
</template>

<script setup lang="ts">
import { apiFetch, getImageUrl, ProductListSchema, type ProductList } from '../api';
import { onMounted, ref } from 'vue';
import Product from '../components/Product.vue';

const products = ref<ProductList>([]);

onMounted(async ()=>{
  products.value = await apiFetch("/products", ProductListSchema);

})
</script>