<template>
  <header>
    <div class="link-container">
      <div class="link past-link" @click="download">
        <p class="link-icon"><i class="fa-solid fa-circle-plus"></i></p>
        <p class="link-text">Past Link</p>
        <i v-if="validYoutubeUrl" class="fa-brands fa-youtube"></i>
      </div>
      <div class="link smart-mode" @click="openSmartMode">
        <p class="link-icon"><i class="fa-regular fa-lightbulb text-secondary"
            :class="{ 'text-warning': smartModeStore.enabled }"></i></p>
        <p class="link-text">Smart Mode</p>
      </div>
      <div class="link preferences" @click="openPreferences">
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
import { invoke } from "@tauri-apps/api/tauri";
import { listen, emit } from '@tauri-apps/api/event';

const validYoutubeUrl = ref(false);
const copiedUrl = ref(null);
const smartModeStore = useSmartModeStore();

listen_new_video();
async function listen_new_video() {
  const unlisten = await listen('download_video', (event) => {
    console.log(event.payload);
    invoke("download_new_video", event.payload).then(res => {
      console.log('Video added: New Id => ', res);
    }).catch(err => {
      console.log(err);
    });
  });
}

async function openSmartMode() {
  const webview = new WebviewWindow("SmartMode", {
    url: '/smart-mode',
    title: "Smart Mode",
    resizable: false,
    width: 420,
    height: 380,
    center: true,
    focus: true,
  });


  const unlisten = await webview.listen('smart-mode-changed', (event) => {
    smartModeStore.refreshState(event.payload);
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

async function openDownloadClip() {
  const webview = new WebviewWindow("DownloadClip", {
    url: '/download-clip',
    title: "Download Clip",
    resizable: false,
    width: 515,
    height: 500,
    center: true,
    focus: true,
  });
}


async function download() {
  if (!validYoutubeUrl.value) return;
  if (!smartModeStore.enabled) {
    // Open the custom download window
    openDownloadClip();
    return;
  }

  const param = {
    url: copiedUrl.value,
    directory: smartModeStore.directory,
    format: smartModeStore.format,
    quality: smartModeStore.quality,
    qualityLabel: smartModeStore.quality_label,
  };
  emit('download_video', param);
}

setInterval(() => {
  readText()
    .then(res => {
      validYoutubeUrl.value = res?.includes('youtube.com/watch') || res?.includes('youtube.com/shorts');
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
  color: #EA3323;
  position: absolute;
  top: 23px;
  left: 40px;
}
</style>
