import { createApp } from "vue";
import "./assets/bootstrap.min.css"
import "./style.css";
import App from "./App.vue";
import { createPinia } from "pinia";

createApp(App)
.use(createPinia() )
.mount("#app");
