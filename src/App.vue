<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";
import ToastMessage from "./component/ToastMessage.vue";

</script>
<template>
  <div>
    <nav>
        <RouterLink to="/monitor">Monitor</RouterLink> |
        <RouterLink to="/device">Device</RouterLink> |
        <RouterLink to="/setting">Setting</RouterLink>
        <RouterLink to="/dashboard">Dashboard</RouterLink>
    </nav>
    
  <RouterView v-slot="{ Component }">
    <keep-alive>
      <component :is="Component" />
    </keep-alive>
  </RouterView>
  <!-- Toast Message -->
  <ToastMessage />
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }
}


.monitor {
  max-width: 400px;
  margin: 2rem auto;
  padding: 1.5rem;
  border-radius: 12px;
  background: #f5f5f5;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  text-align: center;
}
.buttons {
  display: flex;
  justify-content: center;
  gap: 1rem;
  margin-top: 1rem;
}
button {
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-weight: bold;
}
button:disabled {
  background: #ccc;
  cursor: not-allowed;
}
button:not(:disabled):first-child {
  background: #4caf50;
  color: white;
}
button:not(:disabled):last-child {
  background: #f44336;
  color: white;
}

</style>