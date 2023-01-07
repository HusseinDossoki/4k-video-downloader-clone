<template>
  <section class="d-flex" :class="{ odd: index % 2 == 0, active: selected?.id == video.id }"
    v-for="(video, index) in downloadsStore.list" :key="video.id" @click="selected = video">
    <img :src="video.thumbnail" :class="{'img-err': !video.thumbnail}">
    <div class="body">
      <h6 class="video-title">{{ video.title || 'Retrieving information' }}</h6>

      <div class="d-flex" v-if="video.status == 'failed'">
        <small style="font-size: 11px" class="text-danger">Failed</small>
      </div>

      <div class="d-flex" v-if="video.status == 'new'">
        <small style="font-size: 11px">Parsing video...</small>
      </div>

      <div class="d-flex" v-if="video.status == 'postponed'">
        <div class="bar-item"><i class="fa-solid fa-circle-info"></i> Downloading is postponed</div>
      </div>

      <div class="d-flex" v-if="video.status == 'inprogress' || video.status == 'paused'">
        <div class="bar-item"><i class="fa-regular fa-clock"></i> {{ formatTime(video.length_seconds) }}</div>
        <div class="bar-item"><i class="fa-solid fa-ruler-horizontal"></i> {{ video.size }}</div>
        <div class="bar-item">
          <i class="fa-solid fa-down-long" :class="{'text-warning': video.status == 'paused'}"></i>
          <div class="progress">
            <div class="progress-bar" role="progressbar" :style="{ width: video.progress + '%' }" aria-valuemin="0"
              aria-valuemax="100"></div>
          </div>
        </div>
      </div>

      <div class="d-flex" v-if="video.status == 'downloaded'">
        <div class="bar-item"><i class="fa-regular fa-clock"></i> {{ formatTime(video.length_seconds) }}</div>
        <div class="bar-item"><i class="fa-solid fa-ruler-horizontal"></i> {{ video.size }}</div>
        <div class="bar-item"><i class="fa-brands fa-youtube"></i> {{ video.quality }}</div>
      </div>

    </div>
    <div class="actions">
      <div class="close">❌</div>
      <div class="resume">Resume</div>
    </div>
  </section>
</template>

<script setup>
import { ref } from "vue";
import { useDownloadsStore } from "../stores/DownloadsStore";
const downloadsStore = useDownloadsStore();
const selected = ref(null);

function formatTime(seconds) {
  // If number of seconds are less than 3600, you can remove hours part and format the string in minutes and seconds.
  if(seconds < 3600) return new Date(seconds * 1000).toISOString().substr(14, 5);
  return new Date(seconds * 1000).toISOString().substr(11, 8);
}
</script>

<style scoped lang="scss">
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
  i {
    font-size: 15px;
  }
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
i.fa-down-long {
  margin-right: 5px;
font-size: 15px;
}

.img-err {
  background-color: #ddd;
}
</style>
