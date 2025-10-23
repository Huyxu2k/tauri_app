import { createRouter, createWebHashHistory } from "vue-router";
import Monitor from "../views/Monitor.vue";
import Device from "../views/Device.vue";
import Setting from "../views/Setting.vue";
import Dashboard from "../views/Dashboard.vue";

const routes = [
  { path: "/", redirect: "/monitor" },
  { path: "/monitor", name: "Monitor", component: Monitor },
  { path: "/device", name: "Device", component: Device },
  { path: "/setting", name: "Setting", component: Setting },
  { path: "/dashboard", name: "Dashboard", component: Dashboard },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;
