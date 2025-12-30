import { useToastStore } from '../stores/toast'

export function useToast() {
    const toastStore = useToastStore()

    return {
        success: (message, duration) => toastStore.addToast(message, 'success', duration),
        error: (message, duration) => toastStore.addToast(message, 'error', duration),
        warning: (message, duration) => toastStore.addToast(message, 'warning', duration),
        info: (message, duration) => toastStore.addToast(message, 'info', duration)
    }
}
