import { defineStore } from "pinia"
import { computed, ref } from "vue"
import * as api from "../api"

export const useAuthStore = defineStore('auth', () => {
    const token = ref<string | null>(localStorage.getItem('auth_token'));

    const isAuthenticated = computed(() => token.value != null)

    const setToken = (newToken: string) => {
        token.value = newToken;
        localStorage.setItem('auth_token', newToken);
    };

    const logout = () => {
        token.value = null;
        localStorage.removeItem('auth_token');
    };


    const authenticate = async (name: string, password: string) => {
        const token = await api.authenticate({
            name, password
        })
        setToken(token.token);
    }



    return {
        authenticate, isAuthenticated, logout, token
    }

})