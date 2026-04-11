import { defineStore } from "pinia";
import { ref } from "vue";


export const useImageStore = defineStore('image', () => {
    const showLightbox = ref(false);

    const images = ref<string[]>([]);
    const imageIndex = ref(0);

    const showImages = (new_images: string[], new_imageIndex: number)=>{
        images.value = new_images;
        imageIndex.value = new_imageIndex;
        showLightbox.value = true;
    }


    return {
        showLightbox,
        images,
        imageIndex,
        showImages
    }
})