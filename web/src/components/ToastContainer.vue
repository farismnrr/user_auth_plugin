<script setup>
import { useToastStore } from '../stores/toast'

const toastStore = useToastStore()

const getToastIcon = (type) => {
    switch (type) {
        case 'success':
            return '✓'
        case 'error':
            return '✕'
        case 'warning':
            return '⚠'
        case 'info':
        default:
            return 'ℹ'
    }
}
</script>

<template>
  <div class="toast-container">
    <transition-group name="toast">
      <div
        v-for="toast in toastStore.toasts"
        :key="toast.id"
        :class="['toast', `toast-${toast.type}`]"
      >
        <div class="toast-icon">
          {{ getToastIcon(toast.type) }}
        </div>
        <div class="toast-message">
          {{ toast.message }}
        </div>
        <button
          class="toast-close"
          @click="toastStore.removeToast(toast.id)"
        >
          ✕
        </button>
      </div>
    </transition-group>
  </div>
</template>

<style scoped>
.toast-container {
  position: fixed;
  top: 1.5rem;
  right: 1.5rem;
  z-index: 9999;
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  pointer-events: none;
}

/* Mobile: Adjust position */
@media (max-width: 640px) {
  .toast-container {
    top: 1rem;
    right: 1rem;
    left: 1rem;
  }
}

.toast {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  min-width: 320px;
  max-width: 420px;
  padding: 1rem 1.25rem;
  border-radius: 12px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15), 0 2px 8px rgba(0, 0, 0, 0.1);
  backdrop-filter: blur(10px);
  pointer-events: auto;
  font-size: 0.9rem;
  line-height: 1.5;
  font-weight: 500;
  transition: all 0.3s ease;
}

/* Mobile: Full width */
@media (max-width: 640px) {
  .toast {
    min-width: auto;
    max-width: none;
    width: 100%;
  }
}

.toast:hover {
  transform: translateY(-2px);
  box-shadow: 0 12px 32px rgba(0, 0, 0, 0.2), 0 4px 12px rgba(0, 0, 0, 0.15);
}

.toast-icon {
  flex-shrink: 0;
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.125rem;
  font-weight: 700;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.25);
}

.toast-message {
  flex: 1;
  color: white;
  word-break: break-word;
}

.toast-close {
  flex-shrink: 0;
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(255, 255, 255, 0.15);
  border: none;
  border-radius: 6px;
  color: white;
  font-size: 1rem;
  font-weight: 700;
  cursor: pointer;
  transition: all 0.2s ease;
}

.toast-close:hover {
  background: rgba(255, 255, 255, 0.3);
  transform: scale(1.1);
}

/* Toast Types */
.toast-success {
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.toast-error {
  background: linear-gradient(135deg, #f87171 0%, #ef4444 100%);
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.toast-warning {
  background: linear-gradient(135deg, #fbbf24 0%, #f59e0b 100%);
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.toast-info {
  background: linear-gradient(135deg, #60a5fa 0%, #3b82f6 100%);
  border: 1px solid rgba(255, 255, 255, 0.2);
}

/* Animations */
.toast-enter-active {
  animation: toast-slide-in 0.3s ease-out;
}

.toast-leave-active {
  animation: toast-slide-out 0.3s ease-in;
}

@keyframes toast-slide-in {
  from {
    opacity: 0;
    transform: translateX(100%) scale(0.9);
  }
  to {
    opacity: 1;
    transform: translateX(0) scale(1);
  }
}

@keyframes toast-slide-out {
  from {
    opacity: 1;
    transform: translateX(0) scale(1);
  }
  to {
    opacity: 0;
    transform: translateX(100%) scale(0.9);
  }
}

/* Mobile: Slide from top instead */
@media (max-width: 640px) {
  @keyframes toast-slide-in {
    from {
      opacity: 0;
      transform: translateY(-100%) scale(0.95);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  @keyframes toast-slide-out {
    from {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
    to {
      opacity: 0;
      transform: translateY(-100%) scale(0.95);
    }
  }
}
</style>
