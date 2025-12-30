<script setup>
import { onMounted } from 'vue'
import { useAuthStore } from './stores/auth'
import ToastContainer from './components/ToastContainer.vue'
import { RouterView } from 'vue-router'

const authStore = useAuthStore()

onMounted(() => {
    authStore.refreshToken()
})
</script>

<template>
  <ToastContainer />
  <RouterView />
</template>

<style>
/* 
  Global layout logic specific to the Vue App wrapper.
  Most styles are in style.css 
*/
#app {
  width: 100%;
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}
</style>
