<template>
  <header>
    <div class="link-container">
      <div class="link past-link" @click="printClipboard">
        <p class="link-icon">+</p>
        <p class="link-text">Past Link</p>
        <img v-if="displayYoutubeIcon" class="youtube" src="../assets/youtube.png" alt="">
      </div>
      <div class="link smart-mode" @click="openSmartMode()">
        <p class="link-icon">💡</p>
        <p class="link-text">Smart Mode</p>
      </div>
      <div class="link preferences" @click="openPreferences()">
        <p class="link-icon">⚙️</p>
        <p class="link-text">Preferences</p>
      </div>
    </div>
  </header>
</template>
  
<script setup>
import { ref } from "vue";
import { WebviewWindow } from "@tauri-apps/api/window";
import { readText } from '@tauri-apps/api/clipboard';

const displayYoutubeIcon = ref(false);

function openSmartMode() {
  const webview = new WebviewWindow("SmartMode", {
    url: '/smart-mode',
    fullscreen: false,
    title: "Smart Mode",
    resizable: false,
    width: 420,
    height: 380,
    center: true,
    alwaysOnTop: false,
    focus: true,
  });
}

function openPreferences() {
  const webview = new WebviewWindow("Preferences", {
    url: '/preferences',
    fullscreen: false,
    title: "Preferences",
    resizable: false,
    width: 620,
    height: 650,
    center: true,
    alwaysOnTop: false,
    focus: true,
  });
}

setInterval(() => {
  readText()
    .then(clipboardText => {
      displayYoutubeIcon.value = clipboardText?.includes('youtube.com');
    });
}, 1000);

</script>
  
<style scoped>
p {
  margin-bottom: 0;
}

header {
  height: 57px;
  top: 0;
  width: 100%;
  background-color: #2d2d2d;
  color: rgba(238, 238, 238, 0.646);
}

.link-container {
  display: flex;
}

.link {
  margin-top: 3px;
  padding-left: 10px;
  padding-right: 10px;
  cursor: pointer;
}

.link .link-icon {
  color: white;
  border-radius: 50%;
  display: block;
  width: 30px;
  height: 30px;
  text-align: center;
  margin: auto;
  font-size: 20px;
  font-weight: bold;
}

.link .link-text {
  margin-top: 2px;
  text-align: center;
  font-size: 12px;
}

.past-link .link-icon {
  background-color: rgb(7, 155, 7);
}

.smart-mode .link-icon {
  background-color: transparent;
}

.preferences {
  margin-left: auto;
}

img.youtube {
  width: 20px;
  height: 20px;
  position: absolute;
  top: 15px;
  left: 40px;
}
</style>
  