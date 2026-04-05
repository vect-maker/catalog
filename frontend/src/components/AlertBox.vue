<template>

  <div class="toast toast-end">
    <div v-for="alert in alertStore.alerts" class="alert alert-info">
      <span>{{ alert.message }}</span>
    </div>

  </div>
</template>
<script setup lang="ts">
import { useAlertStore } from '../stores/useAlertStore';



const alertStore = useAlertStore()
</script>