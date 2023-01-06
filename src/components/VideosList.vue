<template>
  <section class="d-flex" :class="{ odd: index % 2 == 0, active: selected?.id == video.id }"
    v-for="(video, index) in downloadsStore.list" :key="video.id" @click="selected = index">
    <img src="../assets/empty-list.png" alt="video image">
    <div class="body">
      <h6 class="video-title">{{ video.title }}</h6>
      <div class="d-flex">
        <div class="bar-item">🕑 {{ formatTime(video.length_seconds) }}</div>
        <div class="bar-item">ᴞ {{ video.size }}</div>
        <div class="bar-item">⬇
          <div class="progress">
            <div class="progress-bar" role="progressbar" :style="{ width: video.progress + '%' }" aria-valuemin="0"
              aria-valuemax="100"></div>
          </div>
        </div>
      </div>
    </div>
    <div class="actions">
      <div class="close">❌</div>
      <di class="resume">Resume</di>
    </div>
  </section>
</template>

<script setup>
import { ref } from "vue";
import { useDownloadsStore } from "../stores/DownloadsStore";
const downloadsStore = useDownloadsStore();
const selected = ref(null);

function formatTime(seconds) {
  let date = new Date(null);
  date.setSeconds(seconds);
  let hhmmssFormat = date.toISOString().substr(11, 8);
  return hhmmssFormat;
}
</script>

<style scoped>
section {
  position: relative;
  background-color: transparent;
  color: white;
  height: 58px;
  padding: 5px;
  overflow: hidden;
}

section.active {
  background-color: color(srgb 0.1504 0.4695 0.94) !important;
}

section.active .bar-item {
  color: white !important;
}

section.odd {
  background-color: color(srgb 0.1608 0.1608 0.1608);
}

.body {
  margin-left: 10px;
}

.body .video-title {
  font-size: 13px;
}

.bar-item {
  margin-right: 10px;
  font-size: 12px;
  color: rgb(148, 147, 147);
  font-weight: 500;
}

img {
  width: 90px;
  /* height: 100%; */
  margin: 0;
  padding: 0;
  border-radius: 2px;
}

.progress {
  width: 150px;
  height: 6px;
  border-radius: 15px;
  display: inline-flex;
  background-color: color(srgb 0.247 0.2471 0.2471);
}

.actions {
  display: none;
  text-align: right;
  font-size: 10px;
  margin-left: auto;
  margin-right: 8px;
}

.actions .resume {
  display: block;
  margin-top: 10px;
  font-size: 13px;
  color: #ddd;
  cursor: pointer;
}

.actions .close {
  cursor: pointer;
}

section:hover .actions {
  display: block;
}
</style>
