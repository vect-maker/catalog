<template>
    <Teleport to="body">
        <dialog 
            ref="modalRef" 
            class="modal" 
            :class="{ 'modal-open': modelValue }"
            @keydown.esc="close"
        >
            <div class="modal-box relative">
                <button 
                    type="button" 
                    @click="close" 
                    class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
                > <span class="material-symbols-outlined"> close</span></button>

                <h3 v-if="$slots.title" class="text-lg font-bold mb-4">
                    <slot name="title" />
                </h3>
                
                <div class="py-4">
                    <slot />
                </div>

                <div v-if="$slots.actions" class="modal-action">
                    <slot name="actions" />
                </div>
            </div>

            <form method="dialog" class="modal-backdrop">
                <button @click="close">close</button>
            </form> 
        </dialog>
    </Teleport>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';

const props = defineProps<{
    modelValue: boolean
}>();

const emit = defineEmits<{
    (e: 'update:modelValue', value: boolean): void
}>();

const modalRef = ref<HTMLDialogElement | null>(null);

watch(() => props.modelValue, (isOpen) => {
    if (!modalRef.value) return;
    
    if (isOpen) {
        modalRef.value.showModal();
    } else {
        modalRef.value.close();
    }
});

const close = () => {
    emit('update:modelValue', false);
};
</script>