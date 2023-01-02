import { createRouter, createWebHistory } from 'vue-router';
import Home from "../pages/Home.vue";
import SmartMode from "../pages/SmartMode.vue";
import Preferences from "../pages/Preferences.vue";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      component: Home
    },
    {
      path: "/smart-mode",
      component: SmartMode
    },    
    {
      path: "/preferences",
      component: Preferences
    },
  ],
})

export default router
