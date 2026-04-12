<template>
    <div class="navbar backdrop-blur-md bg-base-100 shadow-sm fixed top-0 w-full z-50">
        <div class="navbar-start">
            <div class="dropdown">
                <div tabindex="0" role="button" class="btn btn-ghost btn-circle" aria-label="Menú principal">
             
                     <span class="material-symbols-outlined text-sm">menu</span>
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

        <div class="navbar-center max-w-[50%] sm:max-w-none">
            <RouterLink :to="{ name: 'catalog' }"
                class="btn btn-ghost font-['Noto_Serif'] font-bold px-2 text-base sm:text-xl truncate">
                {{ store_name }}
            </RouterLink>
        </div>

        <div class="navbar-end gap-0 sm:gap-2">
            <SearchModal />
            <NotificationsModal />
        </div>
    </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
import { useAuthStore } from '../stores/useAuthStore';
import SearchModal from './SearchModal.vue';
import NotificationsModal from './NotificationsModal.vue';

const store_name = import.meta.env.VITE_STORE_NAME;
const authStore = useAuthStore()
const router = useRouter()

const logout = () => {
    authStore.logout();
    router.push({ name: "catalog" })
}
</script>