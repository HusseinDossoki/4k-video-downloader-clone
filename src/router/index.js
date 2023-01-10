import { createRouter, createWebHistory } from 'vue-router';
import Home from "../pages/Home.vue";
import SmartMode from "../pages/SmartMode.vue";
import Preferences from "../pages/Preferences.vue";
import DownloadClip from "../pages/DownloadClip.vue";

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
    {
      path: "/download-clip",
      component: DownloadClip
    },
  ],
})

export default router
