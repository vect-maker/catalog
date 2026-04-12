<template>
    <div class="max-w-7xl mx-auto p-4 sm:p-6 lg:p-8 mt-16">
        <div class="text-sm breadcrumbs mb-6">
            <ul>
                <li>
                    <RouterLink :to="{ name: 'catalog' }">Catálogo</RouterLink>
                </li>
                <li class="opacity-60">{{ product?.title || (loading ? 'Cargando...' : 'Error') }}</li>
            </ul>
        </div>

        <div v-if="loading" class="flex flex-col items-center justify-center py-20">
            <span class="loading loading-spinner loading-lg text-primary"></span>
            <p class="mt-4 opacity-50 font-medium">Obteniendo detalles del producto...</p>
        </div>

        <div v-else-if="product"
            class="grid grid-cols-1 lg:grid-cols-2 gap-8 lg:gap-12 animate-in fade-in duration-500">

            <section class="space-y-4">
                <div class="aspect-square rounded-3xl overflow-hidden bg-base-200 cursor-zoom-in group relative shadow-inner"
                    @click="openGallery(0)">
                    <img :src="product.images?.[0] ? api.getImageUrl(product.images[0]) : '/missingImage.jpg'"
                        class="w-full h-full object-cover transition-transform duration-700 group-hover:scale-110"
                        :alt="product.title" />
                    <div
                        class="absolute bottom-4 right-4 badge badge-neutral bg-black/40 backdrop-blur-md border-none p-4 text-xs">
                        <span class="material-symbols-outlined mr-2 text-sm">zoom_in</span>
                        VER GALERÍA
                    </div>
                </div>

                <div v-if="product.images && product.images.length > 1" class="grid grid-cols-4 gap-4">
                    <div v-for="(img, idx) in product.images" :key="img" @click="openGallery(idx)"
                        class="aspect-square rounded-xl overflow-hidden bg-base-200 cursor-pointer hover:ring-2 hover:ring-primary transition-all shadow-sm">
                        <img :src="api.getImageUrl(img)" class="w-full h-full object-cover" />
                    </div>
                </div>
            </section>

            <section class="flex flex-col justify-between">
                <div class="space-y-6">
                    <header>
                        <h1 class="text-3xl sm:text-4xl font-bold font-['Noto_Serif'] leading-tight">
                            {{ product.title }}
                        </h1>
                        <p class="text-3xl font-bold text-primary mt-4 tracking-tight">
                            C${{ product.price.toLocaleString() }}
                        </p>
                    </header>

                    <div class="divider"></div>

                    <article class="prose max-w-none">
                        <h2 class="text-xs font-bold uppercase tracking-widest opacity-50 mb-3">Descripción del Producto
                        </h2>
                        <p class="text-base-content/80 leading-relaxed text-lg whitespace-pre-line">
                            {{ product.description || 'Sin descripción disponible para este artículo.' }}
                        </p>
                    </article>

                    <div v-if="authStore.isAuthenticated"
                        class="bg-base-200/50 p-4 rounded-2xl flex gap-3 border border-base-300">
                        <button class="btn btn-warning flex-1 shadow-sm" @click="editProduct">
                            <span class="material-symbols-outlined text-sm">edit</span> Editar
                        </button>
                        <button class="btn btn-error flex-1 shadow-sm" @click="confirmDelete">
                            <span class="material-symbols-outlined text-sm">delete</span> Eliminar
                        </button>
                    </div>
                </div>

                <div class="mt-10 lg:mt-0 pt-6">
                    <a :href="whatsappLink" target="_blank" rel="noopener noreferrer"
                        class="btn btn-success btn-lg w-full text-white shadow-xl shadow-success/20 gap-3 hover:scale-[1.02] active:scale-95 transition-all">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="w-6 h-6 fill-current">
                            <path
                                d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51-.173-.008-.371-.01-.57-.01-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 01-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 01-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 012.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0012.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 005.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 00-3.48-8.413z" />
                        </svg>
                        Preguntar Disponibilidad
                    </a>
                </div>
            </section>
        </div>

        <div v-else class="text-center py-24 animate-in fade-in duration-700">
            <span class="material-symbols-outlined text-8xl opacity-10">inventory_2</span>
            <h3 class="text-2xl font-bold mt-6">Lo sentimos</h3>
            <p class="opacity-50 mt-2">No pudimos encontrar el producto solicitado o no está disponible.</p>
            <RouterLink :to="{ name: 'catalog' }" class="btn btn-outline btn-primary mt-8">
                <span class="material-symbols-outlined">arrow_back</span>
                Volver al catálogo
            </RouterLink>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue';
import { useAuthStore } from '../stores/useAuthStore';
import { useImageStore } from '../stores/useImageStore';
import { useProductStore } from '../stores/useProductStore';
import type { Product } from '../api/schemas';
import * as api from '../api';
import { useRouter } from 'vue-router';

const props = defineProps<{
    product_id: string;
}>();

const authStore = useAuthStore();
const imageStore = useImageStore();
const productStore = useProductStore();
const router = useRouter()
const storePhone = import.meta.env.VITE_STORE_PHONE_NUMBER;


const product = ref<Product | null>(null);
const loading = ref(true);

const fetchProductDetails = async () => {
    loading.value = true;
    try {
        const result = await api.getProduct(props.product_id);

        if (!result) {
            product.value = null;
            return;
        }

        product.value = result;
    } catch (error) {
        console.error("Unexpected error during product retrieval:", error);
        product.value = null;
    } finally {
        loading.value = false;
    }
};

const whatsappLink = computed(() => {
    if (!product.value || !storePhone) return '#';

    const productUrl = `${window.location.origin}/products/${product.value.id}`;
    
    const message = `¡Hola! Tengo una pregunta sobre el producto: ${product.value.title}.\n\n` +
                    `Puedes verlo aquí: ${productUrl}\n\n` +
                    `¿Está disponible?`;

    return `https://wa.me/${storePhone}?text=${encodeURIComponent(message)}`;
});
const openGallery = (index: number) => {
    if (!product.value?.images || product.value.images.length === 0) {
        imageStore.showImages(['/missingImage.jpg'], 0);
        return;
    }
    const urls = product.value.images.map(api.getImageUrl);
    imageStore.showImages(urls, index);
};

const editProduct = () => {
};

const confirmDelete = async () => {
    if (confirm(`¿Estás seguro de que deseas eliminar permanentemente "${product.value?.title}"?`)) {
        await productStore.deleteProduct(props.product_id);
        router.push({ "name": 'catalog' })
    }
};

onMounted(fetchProductDetails);

watch(
  () => props.product_id,
  (newId, oldId) => {
    if (newId !== oldId) {
      fetchProductDetails();
    }
  }
);
</script>