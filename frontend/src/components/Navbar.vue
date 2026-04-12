<template>
    <div class="navbar    backdrop-blur-md bg-base-100 shadow-sm">
        <div class="navbar-start">
            <div class="dropdown">
                <div tabindex="0" role="button" class="btn btn-ghost btn-circle">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M4 6h16M4 12h16M4 18h7" />
                    </svg>
                </div>
                <ul tabindex="-1"
                    class="menu menu-sm dropdown-content bg-base-100 rounded-box z-1 mt-3 w-52 p-2 shadow">
                    <li>
                        <RouterLink :to="{ name: 'catalog' }">Catalogo</RouterLink>
                    </li>
                    <li v-if="authStore.isAuthenticated">
                        <RouterLink :to="{ name: 'productCreate' }">Agregar producto</RouterLink>
                    </li>
                    <li v-if="!authStore.isAuthenticated">
                        <RouterLink :to="{ name: 'login' }">Iniciar session</RouterLink>
                    </li>
                    <li v-else><button @click="logout">Cerrar session</button></li>
                    <li><a>Sobre nosotros</a></li>
                </ul>
            </div>
        </div>
        <div class="navbar-center">
            <RouterLink :to="{ name: 'catalog' }" class="btn btn-ghost font-['Noto_Serif'] font-bold text-xl">
                {{ store_name }}</RouterLink>
        </div>
        <div class="navbar-end">
            <Search></Search>
            <button class="btn btn-ghost btn-circle">
                <div class="indicator">
                    <span class="material-symbols-outlined">
                        notifications
                    </span>
                    <span class="badge badge-xs badge-primary indicator-item"></span>
                </div>
            </button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
import { useAuthStore } from '../stores/useAuthStore';
import Search from './Search.vue';


const store_name = import.meta.env.VITE_STORE_NAME;

const authStore = useAuthStore()
const router = useRouter()

const logout = () => {
    authStore.logout();
    router.push({ "name": "catalog" })
}

</script>