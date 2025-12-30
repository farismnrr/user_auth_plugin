import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useToastStore = defineStore('toast', () => {
    const toasts = ref([])
    let nextId = 0

    const addToast = (message, type = 'info', duration = 4000) => {
        const id = nextId++
        const toast = {
            id,
            message,
            type, // 'success', 'error', 'warning', 'info'
            duration
        }

        toasts.value.push(toast)

        // Auto-remove after duration
        if (duration > 0) {
            setTimeout(() => {
                removeToast(id)
            }, duration)
        }

        return id
    }

    const removeToast = (id) => {
        const index = toasts.value.findIndex(t => t.id === id)
        if (index > -1) {
            toasts.value.splice(index, 1)
        }
    }

    return {
        toasts,
        addToast,
        removeToast
    }
})
