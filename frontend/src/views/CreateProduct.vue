<template>
    <div class="p-4 flex items-center justify-center">
        <div class="w-xl max:w-md">
            <p class="text-3xl font-bold mb-4">Agregar Producto</p>
            
            <div class="relative rounded-box overflow-hidden">
                
                <div v-if="isUploading" class="absolute inset-0 z-50 flex flex-col items-center justify-center bg-base-100/40">
                    <span class="loading loading-spinner text-primary loading-lg mb-4"></span>
                    <p class="text-xl font-bold text-center">Subiendo producto...</p>
                </div>

                <form 
                    class="fieldset bg-base-200 border-base-300 rounded-box border p-4 transition-all duration-300"
                    :class="{ 'blur-sm pointer-events-none select-none': isUploading }" 
                    @submit.prevent="createProduct"
                >
                    <DropZone ref="dropZoneRef" @files-changed="handleFilesChange" :disabled="isUploading"></DropZone>
                    
                    <label class="fieldset">
                        <label class="label">Titulo</label>
                        <input v-model="title" :disabled="isUploading" type="text" class="input validator w-full" required
                            placeholder="Titulo del producto (4-40 caracteres) " minlength="4" maxlength="40"
                            title="Must be between be 4 to 40" />
                        <span class="validator-hint hidden">Debe de tener de 4 a 40 caracteres</span>
                    </label>

                    <label class="fieldset">
                        <span class="label">Costo (CORDOBAS NI)</span>
                        <input v-model="price" :disabled="isUploading" type="number" min="0" class="input validator w-full" placeholder="Costo"
                            required />
                        <span class="validator-hint hidden">Costo</span>
                    </label>

                    <label class="fieldset">
                        <span class="label">Descripcion</span>
                        <textarea v-model="description" :disabled="isUploading" minlength="4" maxlength="500"  class="textarea w-full"
                            placeholder="Descripcion"></textarea>
                        <span class="validator-hint hidden">Debe de tener de 4 a 500 caracteres</span>
                    </label>

                    <button class="btn btn-neutral mt-4" :disabled="isUploading" type="submit">Publicar</button>
                    <button class="btn btn-ghost mt-1" :disabled="isUploading" type="button" @click="clearForm">Borrar progreso</button>
                </form>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import DropZone from '../components/DropZone.vue';
import { createProduct as createProductRequest, addImageToProduct } from '../api';

const cropSizeLimit = 1024 * 1024
const title = ref("");
const price = ref(0);
const description = ref("");
const productImages = ref<File[]>([]);
const isUploading = ref(false);

const dropZoneRef = ref<InstanceType<typeof DropZone> | null>(null)

const handleFilesChange = (files: File[]) => {

    productImages.value = files.filter((f) => {
        if (f.size > cropSizeLimit) {
            console.error("La imagen es muy grande: maximo 1mb")
            return false
        }

        return true
    })

}

const clearForm = () => {
    title.value = "";
    description.value = "";
    price.value = 0;
    productImages.value = [];
    dropZoneRef.value?.clearFiles();

}

const createProduct = async () => {
    if (productImages.value.length < 1) {
        alert("Debe de incluir almenos una imagen")
        return
    }

    try {
        isUploading.value = true;
        const product = await createProductRequest({
            title: title.value,
            price: price.value,
            description: description.value.length < 1 ? null : description.value
        })
        const uploadPromises = productImages.value.map(imageFile =>
            addImageToProduct(product.id, imageFile)
        );

        const uploadedImages = await Promise.all(uploadPromises);

        console.log(uploadedImages)

    } catch (error) {
        console.error(error);
        clearForm()
        isUploading.value = false;
        return
    }

    clearForm()
    isUploading.value = false;


}

</script>