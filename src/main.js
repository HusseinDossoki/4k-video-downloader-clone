import { createApp } from "vue";
import "./assets/bootstrap.min.css";
import "./assets/fontawesome-free-6.2.1-web/css/all.min.css";
import "./style.css";
import App from "./App.vue";
import { createPinia } from "pinia";
import router from "./router";

createApp(App)
.use(router)
.use(createPinia() )
.mount("#app");
