<script setup lang="ts">
import {useToast, type ToastType } from "../composables/toastStore";

const { toasts } = useToast();

// Gán màu cho trạng thái của toast message
const backgroundColor = (type: ToastType) => {
  switch (type) {
    case "Success":
      return "#4CAF50";
    case "Error":
      return "#F44336";
    default:
      return "#2196F3";
  }
};

</script>

<template>
    <teleport to="body">
      <div class="toast-wrapper">
        <transition-group name="toast-list" tag="div">
          <div
            v-for="toast in toasts"
            :key="toast.id"
            class="toast-item"
            :style="{ backgroundColor: backgroundColor(toast.type) }"
          >
            {{ toast.message }}
          </div>
        </transition-group>
      </div>
    </teleport>
  </template>
  

  <style scoped>
  .toast-wrapper {
    position: fixed;
    top: 20px;
    right: 20px;
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 10px;
    z-index: 9999;
  }
  
  .toast-item {
    color: white;
    padding: 12px 16px;
    margin-bottom: 5px;
    border-radius: 8px;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
    min-width: 220px;
    font-weight: 600;
    opacity: 0.95;
  }
  
  /* Transition effects */
  .toast-list-enter-active,
  .toast-list-leave-active {
    transition: all 0.3s ease;
  }
  
  .toast-list-enter-from {
    opacity: 0;
    transform: translateY(-20px);
  }
  
  .toast-list-leave-to {
    opacity: 0;
    transform: translateY(20px);
  }
  </style>