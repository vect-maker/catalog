<template>
    <label ref="dropZoneRef"
        class="flex min-h-64 w-full cursor-pointer flex-col items-center justify-center rounded-lg border-2 border-dashed border-base-300 bg-base-200 p-4 overflow-hidden relative"
        :class="{ 'border-primary bg-base-300 transition-colors': isOverDropZone }">
        
        <div v-if="!previews.length" class="flex flex-col items-center justify-center pb-6 pt-5 p-2 pointer-events-none">
            <p class="mb-2 text-sm text-base-content/70">
                <span class="font-semibold text-center">Haz click para cargar imágenes</span> 
                o arrastra y suelta
            </p>
            <p class="text-xs text-base-content/60">PNG, JPG, WEBP</p>
        </div>

        <div v-else class="grid grid-cols-3 sm:grid-cols-4 gap-4 w-full h-full overflow-y-auto pointer-events-none">
            <div v-for="url in previews" :key="url" class="relative aspect-square">
                <img :src="url" alt="preview" class="w-full h-full object-cover rounded shadow-md" />
            </div>
        </div>

        <input id="dropzone-file" type="file" multiple accept="image/png,image/jpeg,image/webp" class="hidden"
            @change="onFileChange" />
    </label>
</template>

<script setup lang="ts">
import { useDropZone } from '@vueuse/core'
import { onBeforeUnmount, ref } from 'vue'

const emit = defineEmits<{
      (e: 'filesChanged', payload: File[]): void
}>()

const dropZoneRef = ref<HTMLDivElement>()
const previews = ref<string[]>([]) 

const processFiles = (rawFiles: File[] | FileList | null) => {
    if (!rawFiles) return

    const validFiles = Array.from(rawFiles).filter(file => file.type.startsWith('image/'))
    if (!validFiles.length) return

    cleanupUrls()

    previews.value = validFiles.map(file => URL.createObjectURL(file))
    
    emit('filesChanged', validFiles)
}

const onFileChange = (event: Event) => {
    const target = event.target as HTMLInputElement
    processFiles(target.files)
    target.value = '' 
}

const { isOverDropZone } = useDropZone(dropZoneRef, {
    onDrop: processFiles,
    dataTypes: ['image/png', 'image/jpeg', 'image/webp'], 
    multiple: true, 
    preventDefaultForUnhandled: false,
})



const cleanupUrls = () => {
    previews.value.forEach(url => URL.revokeObjectURL(url))
    previews.value = []
   
}

defineExpose({
    clearFiles: cleanupUrls
})

onBeforeUnmount(cleanupUrls)

</script>