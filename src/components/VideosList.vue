<template>
  <section class="d-flex box" :class="{ odd: index % 2 == 0, active: selected?.id == video.id }"
    v-for="(video, index) in downloadsStore.list" :key="video.id" @click="selected = video"
    @contextmenu="onContextMenu($event, video)">
    <img :src="video.thumbnail" :class="{ 'img-err': !video.thumbnail }">
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
        <div class="bar-item time"><i class="fa-regular fa-clock"></i> {{ $filters.formatTime(video.length_seconds) }}</div>
        <div class="bar-item size"><i class="fa-solid fa-ruler-horizontal"></i> {{
          $filters.formatSize(video.size_in_bytes)
        }}</div>
        <div class="bar-item">
          <i class="fa-solid fa-down-long" :class="{ 'text-warning': video.status == 'paused' }"></i>
          <div class="progress">
            <div class="progress-bar" role="progressbar" :style="{ width: video.progress + '%' }" aria-valuemin="0"
              aria-valuemax="100"></div>
          </div>
        </div>
      </div>

      <div class="d-flex" v-if="video.status == 'downloaded'">
        <div class="bar-item time"><i class="fa-regular fa-clock"></i> {{ $filters.formatTime(video.length_seconds) }}</div>
        <div class="bar-item size"><i class="fa-solid fa-ruler-horizontal"></i> {{
          $filters.formatSize(video.size_in_bytes)
        }}</div>
        <div class="bar-item"><i class="fa-brands fa-youtube"></i> {{ video.format + ' . ' + video.quality_label }}
        </div>
      </div>

    </div>
    <div class="actions">
      <div class="close" @click="deleteDownloadItem(video.id)">❌</div>
      <div class="resume" v-if="video.status == 'paused'">Resume</div>
    </div>
  </section>
</template>

<script setup>
import { ref } from "vue";
import { useDownloadsStore } from "../stores/DownloadsStore";
import ContextMenu from "@imengyu/vue3-context-menu";
import { invoke } from "@tauri-apps/api/tauri";
import { writeText } from '@tauri-apps/api/clipboard';

const downloadsStore = useDownloadsStore();
const selected = ref(null);

async function deleteDownloadItem(id) {
  await downloadsStore.deleteDownloadItem(id);
}
function onContextMenu(e, video) {
  //prevent the browser's default menu
  e.preventDefault();
  //show our menu
  ContextMenu.showContextMenu({
    theme: 'dark',
    customClass: 'custom-context-menu',
    x: e.x,
    y: e.y,
    items: [
      {
        label: "Play",
        onClick: () => {
          alert("You click a menu item");
        }
      },
      {
        divided: true,
        label: "Show in Finder",
        onClick: () => {
          invoke("show_in_folder", { path: `${video.directory}/${video.title}.mp4` });
        }
      },
      {
        divided: true,
        label: "Copy Link Address",
        onClick: () => {
          writeText(video.url);
        }
      },
      {
        label: "Remove",
        onClick: () => {
          deleteDownloadItem(video.id);
        }
      },
      {
        label: "Delete File",
        onClick: () => {
          deleteDownloadItem(video.id);
        }
      },
      {
        label: "Remove All",
        onClick: () => {
          invoke("remove_all_downloads");
        }
      },
    ]
  });
}
</script>

<style scoped lang="scss">
:global(.custom-context-menu) {
  padding: 0 !important;
  padding-bottom: 5px !important;
  padding-top: 5px !important;
}

:global(.custom-context-menu .mx-context-menu-item) {
  cursor: pointer !important;
  padding: 5px !important;
}

section {
  position: relative;
  background-color: transparent;
  color: white;
  height: 58px;
  padding: 5px;
  overflow: hidden;
}

section.active {
  background-color: #2678F0 !important;
}

section.active .bar-item {
  color: white !important;
}

section.odd {
  background-color: #292929;
}

.body {
  margin-left: 10px;
}

.body .video-title {
  font-size: 13px;
  width: max-content;
}

.bar-item {
  margin-right: 10px;
  font-size: 12px;
  color: rgb(148, 147, 147);
  font-weight: 500;

  i {
    font-size: 15px;
  }
  &.time {
    width: 72px;
  }
  &.size {
    width: 90px;
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
  background-color: #3F3F3F;
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
