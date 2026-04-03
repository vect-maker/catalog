<template>
    <label ref="dropZoneRef"
        class="flex h-64 w-full cursor-pointer flex-col items-center justify-center rounded-lg border-2 border-dashed border-base-300 bg-base-200"
        :class="{ 'hover:bg-base-300 transition-colors': isOverDropZone }">
        <div v-if="!previewUrl" class="flex flex-col items-center justify-center pb-6 pt-5 p-2">
            <p class="mb-2 text-sm text-base-content/70">
                <span class="font-semibold text-center">Haz click para cargar imagen</span> 
                o arrastra y suelta
            </p>
            <p class="text-xs text-base-content/60">PNG o JPG</p>
        </div>
        <div v-else class="h-full w-full flex items-center justify-center">
            <img :src="previewUrl" alt="preview" class="max-h-full max-w-full object-contain rounded" />
        </div>

        <input  id="dropzone-file" type="file" accept="image/png,image/jpeg,image/webp" class="hidden"
            @change="onFileChange" />
    </label>
</template>
<script setup lang="ts">
import { useDropZone } from '@vueuse/core'
import { onBeforeUnmount, ref, watch } from 'vue'

const emit = defineEmits<{
      (e: 'fileChange', payload: File | null): void
}>()

const image = ref<File | null>(null)
const dropZoneRef = ref<HTMLDivElement>()
const previewUrl = ref<string | null>(null)

watch(image, (newVal) => {
    if (!newVal) return
    if (previewUrl.value) URL.revokeObjectURL(previewUrl.value)
    previewUrl.value = URL.createObjectURL(newVal)

    emit('fileChange', image.value)
})

const onFileChange = (event: Event) => {
    const target = event.target as HTMLInputElement
    if (target.files?.length) {
        image.value = target.files[0]
    }
}

const { isOverDropZone } = useDropZone(dropZoneRef, {
    onDrop: (files: File[] | null) => {
        if (files?.length) {
            image.value = files[0]
        }
    },
    dataTypes: ["image/png,image/jpeg,image/webp"],
    multiple: false,
    preventDefaultForUnhandled: false,
})

onBeforeUnmount(() => {
  if (previewUrl.value) URL.revokeObjectURL(previewUrl.value)
})

</script>