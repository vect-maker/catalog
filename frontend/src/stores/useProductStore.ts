import { defineStore } from "pinia";
import { ref } from "vue";
import * as api from "../api"
import {  type PaginatedProducts } from "../api/schemas";

export const useProductStore = defineStore('product', () => {
    const paginatedProducts = ref<PaginatedProducts>();

    const loadProducts = async (page: number = 0) => {
        paginatedProducts.value = await api.getPaginatedProducts(page);
    }

    const nextProducts = async ()=>{
        if (!paginatedProducts.value) return

        await loadProducts(paginatedProducts.value.page + 1)
    }

    const previousProducts = async ()=>{
        if (!paginatedProducts.value) return

        await loadProducts(paginatedProducts.value.page - 1)
    }


    const deleteProduct = async (product_id: string)=>{
        await api.deleteProduct(product_id);
        await loadProducts();
    }

    return {
        paginatedProducts,
        loadProducts,
        deleteProduct,
        nextProducts,
        previousProducts
    }
})