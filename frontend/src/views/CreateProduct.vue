<template>
    <div class="p-4 flex items-center justify-center">
        <div class=" w-xl max:w-md ">
            <p class="text-3xl font-bold mb-4">Agregar Producto</p>
            <form class="fieldset bg-base-200 border-base-300 rounded-box  border p-4" @submit.prevent
                @submit="createProduct">
                <DropZone @files-changed="handleFilesChange"></DropZone>
                <label class="fieldset">
                    <label class="label">Titulo</label>
                    <input v-model="title" type="text" class="input validator w-full" required
                        placeholder="Titulo del producto (4-40 caracteres) " minlength="4" maxlength="40"
                        title="Must be between be 4 to 40" />
                    <span class="validator-hint hidden">Debe de tener de 4 a 40 caracteres</span>
                </label>

                <label class="fieldset">
                    <span class="label">Costo (CORDOBAS NI)</span>
                    <input v-model="price" type="number" min="0" class="input validator w-full" placeholder="Costo"
                        required />
                    <span class="validator-hint hidden">Costo</span>
                </label>
                <label class="fieldset">
                    <span class="label">Descripcion</span>
                    <textarea v-model="description" minlength="4" maxlength="500" required class="textarea w-full"
                        placeholder="Descripcion"></textarea>
                    <span class="validator-hint hidden">Debe de tener de 4 a 500 caracteres</span>
                </label>


                <button class="btn btn-neutral mt-4" type="submit">Publicar</button>
                <button class="btn btn-ghost mt-1" type="reset">Borrar progreso</button>
            </form>
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

const handleFilesChange = (files: File[]) => {

    productImages.value = files.filter((f) => {
        if ( f.size > cropSizeLimit) {
            console.error("La imagen es muy grande: maximo 1mb")
            return false
        }

        return true
    })

}

const createProduct = async () => {
    if (productImages.value.length < 1) {
        alert("Debe de incluir almenos una imagen")
        return
    }

    try {
        const product = await createProductRequest({
            title: title.value,
            price: price.value,
            description: description.value
        })
        
        productImages.value.forEach(async (imageFile) => {
            const image = await addImageToProduct(product.id, imageFile);
            console.log(image)
        });    

    } catch (error) {
        console.error(error)
    }
}

</script>