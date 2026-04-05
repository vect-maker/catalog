<template>
<div class="p-4">
    
      <p class="text-3xl font-bold mb-4">Catalogo</p>
      <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 xl:grid-cols-4 gap-4">
        <Product v-for="product in productStore.products"  :key="product.id" :product="product"></Product>
      </div>



  </div>
  
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import Product from '../components/Product.vue';
import { useProductStore } from '../stores/useProductStore';


const productStore = useProductStore()


onMounted(async ()=>{
  await productStore.loadProducts()
})
</script>