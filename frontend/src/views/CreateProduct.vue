<template>
    <div class="p-4 flex items-center justify-center">
        <div class="w-xl max:w-md">
            <p class="text-3xl font-bold mb-4">Agregar Producto</p>

            <div class="relative rounded-box overflow-hidden">

                <div v-if="isUploading"
                    class="absolute inset-0 z-50 flex flex-col items-center justify-center bg-base-100/40">
                    <span class="loading loading-spinner text-primary loading-lg mb-4"></span>
                    <p class="text-xl font-bold text-center">Subiendo producto...</p>
                </div>

                <form class="fieldset bg-base-200 border-base-300 rounded-box border p-6 transition-all duration-300"
                    :class="{ 'blur-sm pointer-events-none select-none': isUploading }" @submit.prevent="createProduct">
                    
                    <div class="fieldset">
                        <label class="label">Imagenes</label>
                        <ProductImagesEditor v-model="productImages"></ProductImagesEditor>
                    </div>

                    <label class="fieldset">
                        <label class="label">Titulo</label>
                        <input v-model="title" :disabled="isUploading" type="text" class="input validator w-full"
                            required placeholder="Titulo del producto (4-40 caracteres)" minlength="4" maxlength="40"
                            title="Must be between 4 to 40" />
                        <span class="validator-hint hidden">Debe de tener de 4 a 40 caracteres</span>
                    </label>

                    <label class="fieldset">
                        <span class="label">Costo (CORDOBAS NI)</span>
                        <input v-model="price" :disabled="isUploading" type="number" min="1"
                            class="input validator w-full" placeholder="Costo" required />
                        <span class="validator-hint hidden">Costo</span>
                    </label>

                    <label class="fieldset">
                        <span class="label">Descripcion</span>
                        <textarea v-model="description" :disabled="isUploading" minlength="4" maxlength="500"
                            class="textarea w-full" placeholder="Descripcion"></textarea>
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
import { createProduct as createProductRequest, addImageToProduct } from '../api';
import { useAlertStore } from '../stores/useAlertStore';
import ProductImagesEditor from '../components/ProductImagesEditor.vue';

const alertStore = useAlertStore()

const cropSizeLimit = 1024 * 1024
const title = ref("");
const price = ref(0);
const description = ref("");
const productImages = ref<File[]>([]);
const isUploading = ref(false);


const clearForm = () => {
    title.value = "";
    description.value = "";
    price.value = 0;
    productImages.value = []; 
}

const createProduct = async () => {
    if (productImages.value.length < 1) {
        alertStore.pushAlert("Debe de incluir al menos una imagen");
        return;
    }

    const hasOversized = productImages.value.some(file => file.size > cropSizeLimit);
    if (hasOversized) {
        alertStore.pushAlert("Una o más imágenes superan el límite de 1MB.");
        return;
    }

    try {
        isUploading.value = true;
        
        const product = await createProductRequest({
            title: title.value,
            price: price.value,
            description: description.value.length < 1 ? null : description.value
        });

        const uploadPromises = productImages.value.map(imageFile =>
            addImageToProduct(product.id, imageFile)
        );

        await Promise.all(uploadPromises);

        alertStore.pushAlert("Se publico el producto correctamente");
    } catch (error) {
        console.error(error);
        alertStore.pushAlert("Error al publicar el producto");
    } finally {
        clearForm();
        isUploading.value = false;
    }
}
</script>