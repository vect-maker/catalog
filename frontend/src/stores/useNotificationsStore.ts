import { defineStore } from "pinia";
import { ref, computed, watch } from "vue";

export type NotificationType = 'info' | 'success' | 'warning' | 'error';

export interface AppNotification {
    id: number;
    title: string;
    message: string;
    type: NotificationType;
    timestamp: Date;
    isRead: boolean;
}

const STORAGE_KEY = 'app_notifications_v1';
const MAX_HISTORY = 50;

const loadSavedNotifications = (): AppNotification[] => {
    try {
        const saved = localStorage.getItem(STORAGE_KEY);
        if (!saved) return [];
        const parsed = JSON.parse(saved);
        return parsed.map((n: any) => ({ ...n, timestamp: new Date(n.timestamp) }));
    } catch {
        return [];
    }
};

export const useNotificationsStore = defineStore('notifications', () => {
    const notifications = ref<AppNotification[]>(loadSavedNotifications());

    let notificationIdCounter = notifications.value.length > 0 
        ? Math.max(...notifications.value.map(n => n.id)) 
        : 0;

    watch(notifications, (newVal) => {
        const truncated = newVal.slice(0, MAX_HISTORY);
        localStorage.setItem(STORAGE_KEY, JSON.stringify(truncated));
    }, { deep: true });

    const unreadCount = computed(() => 
        notifications.value.filter(n => !n.isRead).length
    );

    const push = (title: string, message: string, type: NotificationType) => {
        const id = ++notificationIdCounter;

        notifications.value.unshift({
            id,
            title,
            message,
            type,
            timestamp: new Date(),
            isRead: false
        });
    };

    const markAsRead = (id: number) => {
        const note = notifications.value.find(n => n.id === id);
        if (note) note.isRead = true;
    };

    const markAllAsRead = () => {
        notifications.value.forEach(n => n.isRead = true);
    };

    const clearAll = () => {
        notifications.value = [];
        localStorage.removeItem(STORAGE_KEY);
    };

    return {
        notifications,
        unreadCount,
        push,
        markAsRead,
        markAllAsRead,
        clearAll
    };
});