<template>


  <div class="card bg-base-100 shadow-sm group">
  <figure class="px-4 pt-4">
    <img
      :src="image_url" :alt="title"
      class="rounded-xl w-full h-full object-cover transition-transform duration-500 group-hover:scale-110" />
  </figure>
  <div class="card-body">
    <h2 class="card-title text-left">{{ title }}</h2>
    <p>{{ description }}</p>
    <p class="text-xl font-semibold text-right">C${{ price }}</p>

  </div>
</div>
</template>
<script setup lang="ts">

const props = defineProps<{
    title: string,
    description: string,
    image_url: string,
    price: number
}>()

</script>