<script setup>
import NetworkBackground from '../components/NetworkBackground.vue'

const goBack = () => {
  window.location.href = '/login'
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
          <p class="brand-quote">
            "Security is not about building walls, but about knowing who has the keys."
          </p>
          <span class="brand-author">— IoTNet Security Team</span>
        </div>
      </div>
      <div class="overlay-gradient" />
    </div>

    <!-- Right Side: Forbidden Content -->
    <div class="panel-form">
      <div class="form-container forbidden-container">
        <div class="form-header">
          <img
            src="/logo.svg"
            alt="IoTNet Logo"
            class="brand-logo-mobile"
          >
          
          <!-- 403 Badge -->
          <div class="error-badge">
            <span class="error-code">403</span>
          </div>
          
          <h1>Nice Try, Detective 🔍</h1>
          <p class="subtitle">
            Access Forbidden
          </p>
        </div>

        <div class="forbidden-message">
          <p class="main-message">
            You've found a door that doesn't lead anywhere we recognize. 
            In the world of security, every destination needs to be on the map. 
            <strong>Yours? Off the grid.</strong>
          </p>

          <div class="riddle-box">
            <p class="riddle-text">
              <em>🧩 I protect without moving, validate without judging, and block without touching. What am I?</em>
            </p>
            <details class="riddle-answer">
              <summary>Reveal Answer</summary>
              <p>An Origin Whitelist</p>
            </details>
          </div>

          <p class="footer-message">
            If you believe this is a mistake, contact your system administrator.
          </p>
        </div>

        <div class="action-buttons">
          <button
            class="btn-primary"
            @click="goBack"
          >
            ← Return to Safety
          </button>
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
  background: linear-gradient(135deg, #0f172a 0%, #1e1b4b 100%);
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

@media (max-width: 640px) {
  .panel-form {
    padding: 1.5rem 1rem;
  }
}

.form-container {
  width: 100%;
  max-width: 480px;
}

.forbidden-container {
  text-align: center;
}

.form-header {
  margin-bottom: 2rem;
}

.error-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 1.5rem;
}

.error-code {
  font-size: 4rem;
  font-weight: 800;
  background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  letter-spacing: -0.02em;
}

.form-header h1 {
  font-size: 1.875rem;
  font-weight: 700;
  color: var(--color-text-main, #1e293b);
  margin-bottom: 0.5rem;
  letter-spacing: -0.01em;
}

.form-header .subtitle {
  font-size: 1rem;
  color: #ef4444;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.forbidden-message {
  text-align: left;
}

.main-message {
  font-size: 1.05rem;
  color: var(--color-text-muted, #64748b);
  line-height: 1.7;
  margin-bottom: 1.5rem;
}

.main-message strong {
  color: var(--color-text-main, #1e293b);
}

.riddle-box {
  background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%);
  border: 1px solid #e2e8f0;
  border-radius: 12px;
  padding: 1.25rem 1.5rem;
  margin-bottom: 1.5rem;
  position: relative;
}

.riddle-text {
  font-size: 0.95rem;
  color: var(--color-text-muted, #64748b);
  line-height: 1.6;
  margin-bottom: 0.75rem;
}

.riddle-answer {
  font-size: 0.875rem;
  color: var(--color-primary, #6366f1);
  cursor: pointer;
}

.riddle-answer summary {
  font-weight: 600;
  transition: color 0.2s;
}

.riddle-answer summary:hover {
  color: #4f46e5;
}

.riddle-answer p {
  margin-top: 0.5rem;
  padding: 0.5rem;
  background: white;
  border-radius: 6px;
  font-weight: 600;
  color: var(--color-text-main, #1e293b);
}

.footer-message {
  font-size: 0.875rem;
  color: var(--color-text-muted, #64748b);
  text-align: center;
}

.action-buttons {
  margin-top: 2rem;
}

.btn-primary {
  width: 100%;
  min-height: 48px;
  font-size: 1rem;
  background: linear-gradient(135deg, var(--color-primary, #6366f1) 0%, #4f46e5 100%);
  border: none;
  border-radius: 8px;
  color: white;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-primary:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(99, 102, 241, 0.3);
}

@media (max-width: 640px) {
  .error-code {
    font-size: 3rem;
  }
  
  .form-header h1 {
    font-size: 1.5rem;
  }
  
  .main-message {
    font-size: 0.95rem;
  }
}
</style>
