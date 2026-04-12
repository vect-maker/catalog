<template>
  <div class="toast toast-end toast-bottom z-110 flex flex-col items-end">
    <TransitionGroup name="toast-list">
      <div v-for="toast in activeToasts" :key="toast.id"
        class="alert shadow-lg transition-all duration-300 w-auto min-w-62.5" :class="typeClasses[toast.type]">
        <span class="material-symbols-outlined">{{ typeIcons[toast.type] }}</span>
        <div class="flex flex-col">
          <span class="font-bold text-sm">{{ toast.title }}</span>
          <span class="text-xs">{{ toast.message }}</span>
        </div>
        <button @click="removeToast(toast.id)" class="btn btn-ghost btn-xs btn-circle">
          <span class="material-symbols-outlined text-sm">close</span>
        </button>
      </div>
    </TransitionGroup>
  </div>
</template>

<style scoped>
.toast-list-enter-from {
  opacity: 0;
  transform: translateY(20px);
}

.toast-list-leave-to {
  opacity: 0;
  transform: scale(0.95);
}
</style>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { useNotificationsStore, type AppNotification } from '../stores/useNotificationsStore';

const notificationsStore = useNotificationsStore();
const activeToasts = ref<AppNotification[]>([]);
const TOAST_DURATION = 4000;

const typeClasses = {
  success: 'alert-success text-success-content',
  warning: 'alert-warning text-warning-content',
  info: 'alert-info text-info-content',
  error: 'alert-error text-error-content'
};

const typeIcons = {
  success: 'check_circle',
  warning: 'warning',
  info: 'info',
  error: 'error'
};
const removeToast = (id: number) => {
  activeToasts.value = activeToasts.value.filter(t => t.id !== id);
};

watch(() => notificationsStore.notifications.length, (newLen, oldLen) => {
  if (newLen > oldLen) {
    const latest = notificationsStore.notifications[0];

    activeToasts.value.push(latest);

    setTimeout(() => {
      removeToast(latest.id);
    }, TOAST_DURATION);
  }
});
</script>