import { defineStore } from "pinia";
import { ref } from "vue";
import * as api from "../api"
import { type PaginatedProducts } from "../api/schemas";
import { useNotificationsStore } from "./useNotificationsStore";

export const useProductStore = defineStore('product', () => {
    const notificationsStore = useNotificationsStore()

    const paginatedProducts = ref<PaginatedProducts>();
    const isLoading = ref(false);

    const loadProducts = async (page: number = 0) => {
        isLoading.value = true;
        try {
            paginatedProducts.value = await api.getPaginatedProducts(page);
        } finally {
            isLoading.value = false;
        }
    }



    const nextProducts = async () => {
        if (!paginatedProducts.value) return

        await loadProducts(paginatedProducts.value.page + 1)
    }

    const previousProducts = async () => {
        if (!paginatedProducts.value) return

        await loadProducts(paginatedProducts.value.page - 1)
    }


    const deleteProduct = async (product_id: string) => {
        try {
            await api.deleteProduct(product_id);
            await loadProducts();

            notificationsStore.push(
                "Producto Eliminado",
                `El producto con ID ${product_id} ha sido removido exitosamente.`,
                "success"
            );
        } catch (error) {
            notificationsStore.push(
                "Error de Eliminación",
                `No se pudo eliminar el producto ${product_id}.`,
                "error"
            );
        }
    }
    return {
        paginatedProducts,
        loadProducts,
        deleteProduct,
        isLoading,
        nextProducts,
        previousProducts
    }
})