<script setup>
import { ref, watch } from 'vue'
import { RouterLink, useRoute } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import { useQuotes } from '../composables/useQuotes'
import { usePasswordToggle } from '../composables/usePasswordToggle'
import { useSSO } from '../composables/useSSO'
import { ERROR_MESSAGES } from '../utils/errorMessages'
import NetworkBackground from '../components/NetworkBackground.vue'

const authStore = useAuthStore()
const route = useRoute()
const username = ref('')
const email = ref('')
const password = ref('')
const confirmPassword = ref('')
const { showPassword, togglePassword } = usePasswordToggle()
const { showPassword: showConfirmPassword, togglePassword: toggleConfirmPassword } = usePasswordToggle()

// Use shared quotes composable
const { currentQuote } = useQuotes()

// Use shared SSO composable
useSSO()



const handleRegister = async () => {
    if (password.value !== confirmPassword.value) {
        authStore.error = ERROR_MESSAGES.PASSWORD_MISMATCH
        return
    }
    authStore.error = null
    const role = route.query.role || 'user'
    await authStore.register(username.value, email.value, password.value, role)
}
</script>

<template>
  <div class="split-screen">
    <!-- Left Side: Brand/Visuals -->
    <div class="panel-visual">
      <!-- Animation Component -->
      <NetworkBackground />
       
      <div class="visual-content">
        <div class="brand-container">
          <img
            src="/logo.svg"
            alt="IoTNet Logo"
            class="brand-logo-large"
          >
        </div>
        <div class="quote-container">
          <transition
            name="fade"
            mode="out-in"
          >
            <p
              :key="currentQuote.text"
              class="brand-quote"
            >
              "{{ currentQuote.text }}"
            </p>
          </transition>
          <span class="brand-author">— {{ currentQuote.author }}</span>
        </div>
      </div>
      <div class="overlay-gradient" />
    </div>

    <!-- Right Side: Register Form -->
    <div class="panel-form">
      <div class="form-container">
        <div class="form-header">
          <img
            src="/logo.svg"
            alt="IoTNet Logo"
            class="brand-logo-mobile"
          >
          <h1>Create Account</h1>
          <p>Get started with your free account today.</p>
        </div>

        <form
          class="auth-form"
          @submit.prevent="handleRegister"
        >
          <div class="input-group">
            <label for="username">Username</label>
            <div class="input-wrapper">
              <input 
                id="username" 
                v-model="username" 
                type="text" 
                placeholder="johndoe" 
                required
                minlength="3"
              >
            </div>
          </div>

          <div class="input-group">
            <label for="email">Email Address</label>
            <div class="input-wrapper">
              <input 
                id="email" 
                v-model="email" 
                type="email" 
                placeholder="john@example.com" 
                required
              >
            </div>
          </div>

          <div class="input-group">
            <label for="password">Password</label>
            <div class="input-wrapper">
              <input 
                id="password" 
                v-model="password" 
                :type="showPassword ? 'text' : 'password'" 
                placeholder="••••••••" 
                required
                minlength="6"
              >
              <button
                type="button"
                class="toggle-password"
                @click="togglePassword"
              >
                <!-- Eye Icon (Show) -->
                <svg
                  v-if="!showPassword"
                  xmlns="http://www.w3.org/2000/svg"
                  width="20"
                  height="20"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                ><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" /><circle
                  cx="12"
                  cy="12"
                  r="3"
                /></svg>
                <!-- Eye Off Icon (Hide) -->
                <svg
                  v-else
                  xmlns="http://www.w3.org/2000/svg"
                  width="20"
                  height="20"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                ><path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24" /><line
                  x1="1"
                  y1="1"
                  x2="23"
                  y2="23"
                /></svg>
              </button>
            </div>
          </div>

          <div class="input-group">
            <label for="confirmPassword">Confirm Password</label>
            <div class="input-wrapper">
              <input 
                id="confirmPassword" 
                v-model="confirmPassword" 
                :type="showConfirmPassword ? 'text' : 'password'" 
                placeholder="••••••••" 
                required
                minlength="6"
              >
              <button
                type="button"
                class="toggle-password"
                @click="toggleConfirmPassword"
              >
                <svg
                  v-if="!showConfirmPassword"
                  xmlns="http://www.w3.org/2000/svg"
                  width="20"
                  height="20"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                ><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" /><circle
                  cx="12"
                  cy="12"
                  r="3"
                /></svg>
                <svg
                  v-else
                  xmlns="http://www.w3.org/2000/svg"
                  width="20"
                  height="20"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                ><path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24" /><line
                  x1="1"
                  y1="1"
                  x2="23"
                  y2="23"
                /></svg>
              </button>
            </div>
          </div>

          <div
            v-if="authStore.error"
            class="inline-error"
          >
            {{ authStore.error }}
          </div>

          <button
            type="submit"
            :disabled="authStore.loading"
            class="btn-primary"
          >
            <span v-if="authStore.loading">Creating account...</span>
            <span v-else>Create Account</span>
          </button>
        </form>

        <div class="form-footer">
          <p>
            Already have an account? <RouterLink :to="{ path: '/login', query: route.query }">
              Sign in
            </RouterLink>
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.split-screen {
  display: flex;
  min-height: 100vh;
  width: 100%;
}

/* Visual Panel (Left) */
.panel-visual {
  display: none;
  flex: 1;
  background: linear-gradient(135deg, #0f172a 0%, #1e1b4b 100%); /* Deep Navy to Indigo */
  position: relative;
  overflow: hidden;
  flex-direction: column;
  justify-content: space-between;
  padding: 4rem;
}

@media (min-width: 1024px) {
  .panel-visual {
    display: flex;
  }
}

.overlay-gradient {
  position: absolute;
  top: 0; left: 0; right: 0; bottom: 0;
  background: linear-gradient(to bottom, rgba(15, 23, 42, 0.85), rgba(15, 23, 42, 0.5));
  z-index: 1;
}

.visual-content {
  position: relative;
  z-index: 10;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
}

.brand-container {
    margin-top: 1rem;
}

.brand-logo-large {
    height: 48px;
    width: auto;
}

.brand-logo-mobile {
  display: block;
  height: 56px;
  width: auto;
  margin: 0 auto 1.5rem auto;
}

@media (min-width: 1024px) {
  .brand-logo-mobile {
    display: none;
  }
}

.quote-container {
    max-width: 480px;
}

.brand-quote {
  font-size: 1.75rem;
  font-weight: 600;
  line-height: 1.3;
  color: white;
  margin-bottom: 1.5rem;
}

.brand-author {
    font-size: 1rem;
    color: #94a3b8;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.05em;
}

/* Form Panel (Right) */
.panel-form {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: white;
  padding: 2rem;
}

/* Mobile: Full width, reduced padding */
@media (max-width: 640px) {
  .panel-form {
    padding: 1.5rem 1rem;
  }
}

.form-container {
  width: 100%;
  max-width: 420px;
}

.form-header {
  text-align: center;
  margin-bottom: 3rem;
}

/* Mobile: Reduce spacing */
@media (max-width: 640px) {
  .form-header {
    margin-bottom: 2rem;
  }
}

.form-header h1 {
  font-size: 2rem;
  font-weight: 700;
  color: var(--color-text-main);
  margin-bottom: 0.75rem;
  letter-spacing: -0.01em;
}

/* Mobile: Smaller heading */
@media (max-width: 640px) {
  .form-header h1 {
    font-size: 1.75rem;
  }
}

.form-header p {
  color: var(--color-text-muted);
  font-size: 1rem;
}

/* Mobile: Smaller text */
@media (max-width: 640px) {
  .form-header p {
    font-size: 0.9rem;
  }
}

.auth-form {
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
}

/* Mobile: Slightly reduce gap */
@media (max-width: 640px) {
  .auth-form {
    gap: 1rem;
  }
}

.input-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.input-group label {
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--color-text-main);
}

.input-wrapper {
    position: relative;
    display: flex;
    align-items: center;
}

.input-wrapper input, .input-wrapper select {
    width: 100%;
    padding-right: 2.5rem;
    min-height: 44px; /* Touch-friendly */
    font-size: 16px; /* Prevent iOS zoom */
}

/* Password Toggle Config */
.toggle-password {
    position: absolute;
    right: 0.75rem;
    background: none;
    border: none;
    padding: 0.5rem; /* Larger touch target */
    min-width: 44px;
    min-height: 44px;
    color: #94a3b8;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: color 0.2s;
}

.toggle-password:hover {
    color: var(--color-primary);
}

/* Inline Error - Refined for Enterprise Feel */
.inline-error {
  padding: 0.875rem 1rem;
  background: #fef2f2;
  border: 1px solid #fecaca;
  border-left: 3px solid #ef4444;
  border-radius: 6px;
  color: #dc2626;
  font-size: 0.875rem;
  line-height: 1.5;
  text-align: left;
  word-break: break-word;
  margin: 0;
  width: 100%;
  box-sizing: border-box;
  animation: slideDown 0.2s ease-out;
}

@keyframes slideDown {
  from {
    opacity: 0;
    transform: translateY(-8px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.btn-primary {
  min-height: 48px; /* Touch-friendly */
  font-size: 1rem;
}

/* Mobile: Larger button */
@media (max-width: 640px) {
  .btn-primary {
    min-height: 52px;
    font-size: 1.05rem;
  }
}

.form-footer {
  margin-top: 2.5rem;
  text-align: center;
  font-size: 0.95rem;
  color: var(--color-text-muted);
}

/* Mobile: Reduce spacing */
@media (max-width: 640px) {
  .form-footer {
    margin-top: 2rem;
    font-size: 0.9rem;
  }
}

/* Fade Transition for Quotes */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.5s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
