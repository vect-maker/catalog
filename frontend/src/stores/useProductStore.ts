import { defineStore } from "pinia";
import { ref } from "vue";
import { apiFetch, ProductListSchema, type ProductList } from "../api";
import * as api from "../api"

export const useProductStore = defineStore('product', () => {
    const products = ref<ProductList>([]);

    const loadProducts = async () => {
        products.value = await apiFetch("/products", ProductListSchema);

    }

    const deleteProduct = async (product_id: number)=>{
        await api.deleteProduct(product_id);
        await loadProducts();
    }

    return {
        products,
        loadProducts,
        deleteProduct
    }
})