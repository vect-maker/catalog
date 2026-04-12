import { defineStore } from "pinia";
import { ref } from "vue";
import * as api from '../api';

export const useBackendStore = defineStore('backend', () => {
    const isBackendReady = ref(false);
    const isInitialized = ref(false); 
    const isPolling = ref(false);
    let pollingTimer: ReturnType<typeof setTimeout> | null = null;


    const checkIfBackendReady = async () => {
        try {
            isBackendReady.value = await api.checkIfBackendReady();
        } catch (error) {
            isBackendReady.value = false;
        } finally {
            isInitialized.value = true;
        }
    };


    const startHeartbeat = (intervalMs = 10000) => {
        if (isPolling.value) return;
        isPolling.value = true;

        const run = async () => {
            if (!isPolling.value) return;

            await checkIfBackendReady();
            pollingTimer = setTimeout(run, intervalMs);
        };

        run();
    };

    const stopHeartbeat = () => {
        isPolling.value = false;
        if (pollingTimer) {
            clearTimeout(pollingTimer);
            pollingTimer = null;
        }
    };

    return {
        isBackendReady,
        isInitialized,
        isPolling,
        startHeartbeat,
        stopHeartbeat,
        checkIfBackendReady
    };
});