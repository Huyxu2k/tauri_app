<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import {ToastPayload, useToast } from "../composables/toastStore";


// interface Device {
//   id: number;
//   serial: string;
//   name: string;
//   model: string;
//   ip_address: string;
//   note: string;
//   count_on: string;
//   status: string;
// }

// const devices = ref<Device[]>([]);

// async function fetchDevices() {
//   devices.value = await invoke<Device[]>("list_devices");
// }
const toast = useToast();
//test
async function handleTest() {
  try {
    const result = (await invoke("test_success")) as ToastPayload;

    if (result && result.state) {
      if (result.state === "Success") {
        toast.success(result.message);
      } else if (result.state === "Error") {
        toast.error(result.message);
      }
    }
  } catch (error) {
    console.error("Lỗi khi gọi command:", error);
    toast.error(`Lỗi hệ thống: ${error}`);
  }
}

// load khi vào screen
//fetchDevices();
</script>

<template>
  <div class="p-4">
    <h1>Connected Devices</h1>

    <button 
      @click="handleTest"
      class="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded transition duration-150"
    >
      Refresh
    </button>

    <!-- <ul>
      <li v-for="device in devices" :key="device.id">
        {{ device.id }} - {{ device.serial }} - {{ device.name }} -
        {{ device.model }}- {{ device.ip_address }}- {{ device.status }}
      </li>
    </ul>

    <p v-if="devices.length === 0">No devices found.</p> -->
  </div>
</template>
