
<template>

  <Navbar class="fixed top-0 w-full z-50 "></Navbar>
  
  <RouterView></RouterView>
  <BottomNav></BottomNav>

</template>

<script setup lang="ts">
import BottomNav from './components/Bottom-nav.vue';
import Navbar from './components/Navbar.vue';
</script>
