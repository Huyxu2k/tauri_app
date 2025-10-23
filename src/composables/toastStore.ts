import {ref, inject,type InjectionKey} from "vue";

export type ToastType = 'Success' | 'Error' ;

export interface ToastPayload{
    message: string;
    state: ToastType
}

export interface ToastItem {
    id: number;
    message: string;
    type: ToastType;
    duration: number;
}

export interface ToastMethods {
    show: (message: string, type?: ToastType, duration?: number) => void;
    success: (message: string, duration?: number) => void;
    error: (message: string, duration?:number) => void;
}

export interface ToastContext extends ToastMethods {
    toasts: ReturnType<typeof ref<ToastItem[]>>;
}

export const TOAST_KEY: InjectionKey<ToastContext> = Symbol("toast");

export function createToast(): ToastContext {
    const toasts = ref<ToastItem[]>([]);
    let nextId = 0;
  
    const remove = (id: number) => {
      toasts.value = toasts.value.filter((t) => t.id !== id);
    };
  
    const show: ToastMethods["show"] = (
      message,
      type = "Success",
      duration = 3000
    ) => {
      const id = nextId++;
      const toast: ToastItem = { id, message, type, duration };
      toasts.value.unshift(toast);
  
      setTimeout(() => remove(id), duration);
    };
  
    const success = (msg: string, duration?: number) =>
      show(msg, "Success", duration);
    const error = (msg: string, duration?: number) => show(msg, "Error", duration);
  
    return {
      toasts,
      show,
      success,
      error,
    };
  }

export function useToast() {
    const toast = inject(TOAST_KEY);
    if (!toast) throw new Error("useToast must be used after ToastPlugin is installed");
    return toast;
}