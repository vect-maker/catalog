<template>
    <div class="p-4 flex items-center justify-center">
        <div class=" w-xl max:w-md ">
            <p class="text-3xl font-bold mb-4">Inicar session</p>
            <form class="fieldset bg-base-200 border-base-300 rounded-box  border p-4" @submit.prevent
                @submit="login">
               
                <label class="fieldset">
                    <label class="label">Usuario</label>
                    <input v-model="name" type="text" class="input w-full" required
                        placeholder="Usuario" minlength="3" maxlength="40"
                         />
                </label>

                <label class="fieldset">
                    <span class="label">Contrasenia</span>
                    <input v-model="password" type="password" minlength="3" class="input w-full" placeholder="contrasenia"
                        required />
                </label>
                <button class="btn btn-neutral mt-4" type="submit">Iniciar session</button>
            </form>
        </div>
    </div>
</template>
<script setup lang="ts">
import { ref } from 'vue';
import { useAuthStore } from '../stores/useAuthStore';
import { useRoute, useRouter } from 'vue-router';

const authStore = useAuthStore();
const route = useRoute();
const router = useRouter();


const name = ref("");
const password = ref("");

const login = async () =>{
    try {
        await authStore.authenticate(name.value, password.value);
        router.push((route.query.redirect as string) || { name: 'catalog' });
    } catch (_) {
        alert("Credenciales invalidas")
    }

}

</script>