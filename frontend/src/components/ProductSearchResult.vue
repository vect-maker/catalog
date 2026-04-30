<template>
    <button class="flex items-center gap-4 py-4 px-4 active:bg-base-300 transition-colors h-auto min-h-[5rem] cursor-pointer text-left">
        <div class="w-14 h-14 aspect-square shrink-0 rounded-lg overflow-hidden bg-base-100 shadow-sm">
            <Image :src="product.images?.[0] ? getImageUrl(product.images[0]) : null" :alt="product.title" />
        </div>

        <div class="flex flex-col flex-1 min-w-0 justify-center">
            <span class="text-sm sm:text-base truncate">
                <span v-for="(token, index) in getHighlightedTokens(product.title, searchQuery)" :key="index"
                    :class="token.isMatch ? 'bg-yellow-200 text-gray-900 font-bold px-0.5 rounded-sm' : ''">
                    {{ token.text }}
                </span>
            </span>
            
            <span v-if="descriptionSnippet" class="text-xs text-gray-500 truncate mt-0.5">
                <span v-for="(token, index) in getHighlightedTokens(descriptionSnippet, searchQuery)" :key="'desc-'+index"
                    :class="token.isMatch ? 'bg-yellow-200 text-gray-900 font-bold px-0.5 rounded-sm' : ''">
                    {{ token.text }}
                </span>
            </span>

            <span class="text-primary font-medium text-sm mt-0.5">C${{ product.price }}</span>
        </div>

        <span class="material-symbols-outlined opacity-20">chevron_right</span>
    </button>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { Product } from '../api/schemas';
import Image from './Image.vue';
import { getHighlightedTokens, getMatchSnippet } from '../utils/highlight';
import { getImageUrl } from '../api';

const props = defineProps<{
    product: Product,
    searchQuery: string
}>();

const descriptionSnippet = computed(() => {
    return getMatchSnippet(props.product.description, props.searchQuery);
});
</script>