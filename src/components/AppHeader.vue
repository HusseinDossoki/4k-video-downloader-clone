<template>
  <header>
    <div class="link-container">
      <div class="link past-link" @click="download">
        <p class="link-icon"><i class="fa-solid fa-circle-plus"></i></p>
        <p class="link-text">Past Link</p>
        <i v-if="validYoutubeUrl" class="fa-brands fa-youtube"></i>
      </div>
      <div class="link smart-mode" @click="openSmartMode()">
        <p class="link-icon"><i class="fa-regular fa-lightbulb text-secondary"
            :class="{ 'text-warning': smartModeStore.enabled }"></i></p>
        <p class="link-text">Smart Mode</p>
      </div>
      <div class="link preferences" @click="openPreferences()">
        <p class="link-icon"><i class="fa-solid fa-screwdriver-wrench"></i></p>
        <p class="link-text">Preferences</p>
      </div>
    </div>
  </header>
</template>

<script setup>
import { ref } from "vue";
import { useSmartModeStore } from "../stores/SmartModeStore";
import { WebviewWindow } from "@tauri-apps/api/window";
import { readText } from '@tauri-apps/api/clipboard';

const validYoutubeUrl = ref(false);
const copiedUrl = ref(null);
const smartModeStore = useSmartModeStore();

function openSmartMode() {
  const webview = new WebviewWindow("SmartMode", {
    url: '/smart-mode',
    title: "Smart Mode",
    resizable: false,
    width: 420,
    height: 380,
    center: true,
    focus: true,
  });
}

function openPreferences() {
  const webview = new WebviewWindow("Preferences", {
    url: '/preferences',
    title: "Preferences",
    resizable: false,
    width: 620,
    height: 650,
    center: true,
    focus: true,
  });
}

function download() {
  if (!validYoutubeUrl.value) return;
  console.log(copiedUrl.value);
}

setInterval(() => {
  readText()
    .then(res => {
      validYoutubeUrl.value = res?.includes('youtube.com/watch');
      copiedUrl.value = res;
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
  font-size: 25px;
  font-weight: bold;
}

.link .link-text {
  margin-top: 2px;
  text-align: center;
  font-size: 12px;
}

.past-link .link-icon i {
  color: rgb(7, 155, 7);
  background: white;
  border-radius: 50%;
  zoom: 1.3;
}

.smart-mode .link-icon {
  background-color: transparent;
}

.preferences {
  margin-left: auto;
}

i.fa-youtube {
  color: color(display-p3 0.9176 0.2 0.1373);
  position: absolute;
  top: 23px;
  left: 40px;
}
</style>
