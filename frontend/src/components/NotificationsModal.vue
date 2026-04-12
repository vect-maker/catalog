<template>
  <button class="btn btn-ghost btn-circle" @click="showPanel = true" aria-label="Notificaciones">
    <div class="indicator">
      <span class="material-symbols-outlined">notifications</span>
      <span v-if="notificationsStore.unreadCount > 0" class="badge badge-xs badge-primary indicator-item"></span>
    </div>
  </button>

  <Teleport to="body">
    <div 
      class="fixed inset-0 z-100 flex justify-end transition-opacity duration-300"
      :class="showPanel ? 'opacity-100' : 'opacity-0 pointer-events-none'"
    >
      <div class="absolute inset-0 bg-black/40" @click="showPanel = false"></div>

      <aside 
        class="relative w-full max-w-sm bg-base-100 h-full shadow-2xl flex flex-col transition-transform duration-300 ease-in-out"
        :class="showPanel ? 'translate-x-0' : 'translate-x-full'"
      >
        <div class="p-4 border-b border-base-200 flex items-center justify-between bg-base-100">
          <div class="flex items-center gap-2">
            <h2 class="text-lg font-bold">Notificaciones</h2>
            <span v-if="notificationsStore.unreadCount > 0" class="badge badge-sm badge-primary">
                {{ notificationsStore.unreadCount }}
            </span>
          </div>
          <button class="btn btn-sm btn-circle btn-ghost" @click="showPanel = false">
            <span class="material-symbols-outlined">close</span>
          </button>
        </div>

        <div v-if="notificationsStore.notifications.length > 0" class="px-4 py-2 border-b border-base-200 bg-base-200/20 flex justify-end">
          <button @click="notificationsStore.markAllAsRead" class="btn btn-ghost btn-xs text-primary">
            Marcar todo como leído
          </button>
        </div>

        <div class="flex-1 overflow-y-auto">
          <ul v-if="notificationsStore.notifications.length > 0" class="divide-y divide-base-200">
            <li 
              v-for="note in notificationsStore.notifications" 
              :key="note.id"
              class="p-4 hover:bg-base-200/50 transition-colors relative cursor-pointer"
              @click="notificationsStore.markAsRead(note.id)"
            >
              <div v-if="!note.isRead" class="absolute left-1 top-1/2 -translate-y-1/2 w-1 h-8 bg-primary rounded-full"></div>
              
              <div class="flex gap-3">
                <div :class="['p-2 badge h-fit border-none', typeStyles[note.type]]">
                   <span class="material-symbols-outlined text-sm">{{ typeIcons[note.type] }}</span>
                </div>
                <div class="flex-1">
                  <p class="text-sm font-semibold" :class="{ 'opacity-60': note.isRead }">{{ note.title }}</p>
                  <p class="text-xs opacity-70 mt-1">{{ note.message }}</p>
                  <p class="text-[10px] opacity-40 mt-2 uppercase font-bold">{{ formatTime(note.timestamp) }}</p>
                </div>
              </div>
            </li>
          </ul>

          <div v-else class="h-full flex flex-col items-center justify-center opacity-30 p-8 text-center">
            <span class="material-symbols-outlined text-6xl mb-2">notifications_off</span>
            <p>No tienes notificaciones pendientes</p>
          </div>
        </div>
      </aside>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useNotificationsStore } from '../stores/useNotificationsStore';

const notificationsStore = useNotificationsStore();
const showPanel = ref(false);

const typeStyles = {
  success: 'bg-success/20 text-success',
  warning: 'bg-warning/20 text-warning',
  info: 'bg-info/20 text-info',
  error: 'bg-error/20 text-error'
};

const typeIcons = {
  success: 'check_circle',
  warning: 'warning',
  info: 'info',
  error: 'error'
};

const formatTime = (date: Date) => {
  const diffInMinutes = Math.round((date.getTime() - Date.now()) / (1000 * 60));
  
  return new Intl.RelativeTimeFormat('es', { numeric: 'auto' }).format(
    diffInMinutes, 
    'minute'
  );
};
</script>