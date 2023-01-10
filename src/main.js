import { createApp } from "vue";
import "./assets/bootstrap.min.css";
import "./assets/fontawesome-free-6.2.1-web/css/all.min.css";
import "./style.css";
import App from "./App.vue";
import { createPinia } from "pinia";
import router from "./router";
import "@imengyu/vue3-context-menu/lib/vue3-context-menu.css";
import ContextMenu from "@imengyu/vue3-context-menu";
import filters from "./helpers/filters";

const app = createApp(App)
  .use(ContextMenu)
  .use(router)
  .use(createPinia());

app.config.globalProperties.$filters = filters;
app.mount("#app");
