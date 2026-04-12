<template>
    <div class="grid grid-cols-2 md:grid-cols-4 gap-4 w-full">

        <div v-for="(url, index) in previews" :key="url" class="relative w-full aspect-square group">
            <img :src="url" alt="preview"
                class="w-full h-full object-cover rounded-lg shadow-sm border border-gray-200" />

            <button type="button" class="btn btn-error btn-sm top-2 right-2 absolute" aria-label="Editar producto"   @click.stop.prevent="removeFile(index)">
          <span class="material-symbols-outlined text-sm" aria-hidden="true">delete</span>
        </button>
        </div>

        <DropZone @fileChanged="onFileAdded" />

    </div>
</template>

<script setup lang="ts">
import { ref, watch, onBeforeUnmount } from 'vue';
import DropZone from './DropZone.vue';

const files = defineModel<File[]>({ default: () => [] });
const previews = ref<string[]>([]);

const onFileAdded = (file: File) => {
    files.value.push(file);
    previews.value.push(URL.createObjectURL(file));
};

const removeFile = (index: number) => {
    URL.revokeObjectURL(previews.value[index]);
    previews.value.splice(index, 1);
    files.value.splice(index, 1); 
};

watch(files, (newFiles) => {
    if (newFiles.length === 0 && previews.value.length > 0) {
        previews.value.forEach(url => URL.revokeObjectURL(url));
        previews.value = [];
    }
}, { deep: true });

onBeforeUnmount(() => {
    previews.value.forEach(url => URL.revokeObjectURL(url));
});
</script>