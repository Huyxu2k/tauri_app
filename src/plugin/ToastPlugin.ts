import type { App } from "vue";
import { createToast, TOAST_KEY } from "../composables/toastStore";

export const ToastPlugin = {
  install(app: App) {
    const toast = createToast();
    // Tạo provide để khi sử dụng Plugin thì toast đã đc install 
    app.provide(TOAST_KEY, toast);
  },
};
