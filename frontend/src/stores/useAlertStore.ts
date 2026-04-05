import { defineStore } from "pinia";
import { ref } from "vue";

interface Alert {
    message: string,
    id: number
}

let toastIdCounter = 0;

export const useAlertStore = defineStore('alert', () => {
    const alerts = ref<Alert[]>([])

    const removeAlert = (id: number) =>{
        alerts.value = alerts.value.filter(t => t.id !== id)
    }


    const pushAlert = (message: string, durationMs = 3000) => {
        const id = ++toastIdCounter;

        alerts.value.push(
            {
                id,
                message
            }
        )

        setTimeout(() => {
            removeAlert(id)
        }, durationMs)
    }

    return {
        pushAlert,
        alerts
    }

})