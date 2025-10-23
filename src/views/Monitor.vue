<script setup lang="ts">
  import { onMounted, ref } from "vue";
  import { invoke } from "@tauri-apps/api/core";
  
  interface ServerStatus {
    status: string;
    address: string;
    port: number;
  }
  
const status = ref<ServerStatus | null>(null);

const serverInfo = ref({ status: "Unknown", address: "", port: 0 });

async function refreshStatus() {
  try {
    serverInfo.value = await invoke("get_status");
  } catch (e) {
    serverInfo.value = { status: "Error", address: "-", port: 0 };
    console.error(e);
  }
}

async function startServer() {
  try {
    await invoke("start_server");
    await refreshStatus();
  } catch (e) {
    console.error("Start server failed:", e);
  }
}

async function stopServer() {
  try {
    await invoke("stop_server");
    await refreshStatus();
  } catch (e) {
    console.error("Stop server failed:", e);
  }
}

onMounted(() => {
  refreshStatus();
});
  
</script>
  

<template>
   <div class="monitor">
      <h1>Local Server Monitor</h1>
      <p>
        Status:
        <strong
          :style="{ color: serverInfo.status === 'Running' ? 'green' : 'red' }"
        >{{ serverInfo.status }}</strong>
      </p>
      <p>Address: <code>{{ serverInfo.address }}</code></p>
      <p>Port: <code>{{ serverInfo.port }}</code></p>
      <div class="buttons">
        <button @click="startServer" :disabled="serverInfo.status === 'Running'">
          Start Server
        </button>
        <button @click="stopServer" :disabled="serverInfo.status === 'Stopped'">
          Stop Server
        </button>
      </div>
    </div>
  </template>
  