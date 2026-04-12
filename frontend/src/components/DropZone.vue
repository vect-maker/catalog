<template>
    <label 
        ref="dropZoneRef"
        class="w-full aspect-square flex justify-center items-center bg-gray-50 rounded-lg border-2 border-dashed border-gray-300 cursor-pointer hover:bg-gray-100 transition-colors"  
        :class="{ 'border-primary bg-base-300': isOverDropZone }"
    >
        <span class="material-symbols-outlined text-gray-400 text-3xl">add</span>
        <input 
            type="file" 
            accept="image/png,image/jpeg,image/webp" 
            class="hidden" 
            @change="onFileChange" 
            :disabled="isCropping"
        />
    </label>

    <Teleport to="body">
        <div v-if="isCropping" class="fixed inset-0 z-100 flex items-center justify-center bg-black/80 p-4 backdrop-blur-sm">
            <div class="bg-base-100 w-full max-w-2xl rounded-xl overflow-hidden shadow-2xl flex flex-col">
                
                <div class="p-4 border-b border-base-300 flex justify-between items-center">
                    <h3 class="font-bold text-lg">Recortar Imagen</h3>
                    <button type="button" @click="cancelCrop" class="btn btn-sm btn-circle btn-ghost">
                        <span class="material-symbols-outlined text-sm">close</span>
                    </button>
                </div>
                
                <div class="w-full h-[60vh] bg-neutral">
                    <Cropper
                        ref="cropperRef"
                        class="w-full h-full"
                        :src="tempImageUrl"
                        :stencil-props="{ aspectRatio: 1 }" 
                    />
                </div>

                <div class="p-4 flex justify-end gap-3 bg-base-200">
                    <button type="button" @click="cancelCrop" class="btn btn-ghost">Cancelar</button>
                    <button type="button" @click="confirmCrop" class="btn btn-primary" :disabled="isProcessing">
                        <span v-if="isProcessing" class="loading loading-spinner loading-sm"></span>
                        Agregar Imagen
                    </button>
                </div>
            </div>
        </div>
    </Teleport>
</template>

<script setup lang="ts">
import { useDropZone } from '@vueuse/core'
import { ref } from 'vue'
import { Cropper } from 'vue-advanced-cropper'
import 'vue-advanced-cropper/dist/style.css'; 

const emit = defineEmits<{
    (e: 'fileChanged', payload: File): void
}>()

const dropZoneRef = ref<HTMLLabelElement>()

// Cropper State
const isCropping = ref(false)
const isProcessing = ref(false)
const tempImageUrl = ref<string>('')
const cropperRef = ref<InstanceType<typeof Cropper> | null>(null)

const processFiles = (rawFiles: File[] | FileList | null) => {
    if (!rawFiles || rawFiles.length === 0) return

    const validFile = Array.from(rawFiles).find(file => file.type.startsWith('image/'))
    if (!validFile) return

    tempImageUrl.value = URL.createObjectURL(validFile)
    isCropping.value = true
}

const onFileChange = (event: Event) => {
    const target = event.target as HTMLInputElement
    processFiles(target.files)
    target.value = '' 
}

const { isOverDropZone } = useDropZone(dropZoneRef, {
    onDrop: processFiles,
    dataTypes: ['image/png', 'image/jpeg', 'image/webp'],
    multiple: false, 
    preventDefaultForUnhandled: false,
})

const confirmCrop = () => {
    if (!cropperRef.value) return
    isProcessing.value = true

    const { canvas } = cropperRef.value.getResult()
    
    if (canvas) {
        canvas.toBlob((blob) => {
            if (blob) {
                const croppedFile = new File([blob], `product-${Date.now()}.webp`, { type: 'image/webp' })
                
                emit('fileChanged', croppedFile)
                cleanup()
            }
        }, 'image/webp', 0.8) 
    }
}

const cancelCrop = () => {
    cleanup()
}

const cleanup = () => {
    isCropping.value = false
    isProcessing.value = false
    if (tempImageUrl.value) {
        URL.revokeObjectURL(tempImageUrl.value)
        tempImageUrl.value = ''
    }
}
</script>