import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import { ToastPlugin } from "./plugin/ToastPlugin";
import ToastMessage from "./component/ToastMessage.vue";

const app = createApp(App);
app.use(router);
app.use(ToastPlugin)

// Mount ToastMessage toàn cục
app.component("ToastMessage", ToastMessage);

app.mount("#app");