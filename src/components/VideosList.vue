<template>
  <section class="d-flex box" :class="{ odd: index % 2 == 0, active: selected?.id == video.id }"
    v-for="(video, index) in downloadsStore.downloads" :key="video.id" @click="selected = video"
    @contextmenu="onContextMenu($event, video)">
    <img :src="video.thumbnail" :class="{ 'img-err': !video.thumbnail }">

    <div class="body" v-if="video.status == 'queued'">
      <h6 class="video-title">Queued</h6>

      <div class="d-flex">
        <small style="font-size: 11px">Waiting...</small>
      </div>

    </div>
    <div class="body" v-if="video.status == 'parsing'">
      <h6 class="video-title">Retrieving information</h6>

      <div class="d-flex">
        <small style="font-size: 11px">Parsing video...</small>
      </div>
    </div>
    <div class="body" v-if="video.status == 'parsed'">
      <h6 class="video-title">{{ video.title }}</h6>

      <div class="d-flex">
        <div class="bar-item time"><i class="fa-regular fa-clock"></i> {{ $filters.formatTime(video.length_seconds) }}
        </div>
        <div class="bar-item size"><i class="fa-solid fa-ruler-horizontal"></i> {{
          $filters.formatSize(video.size_in_bytes)
        }}</div>
        <div class="bar-item">
          <i class="fa-solid fa-down-long"></i>
          <div class="progress">
            <div class="progress-bar" role="progressbar" :style="{ width: video.progress + '%' }" aria-valuemin="0"
              aria-valuemax="100"></div>
          </div>
        </div>
      </div>

    </div>
    <div class="body" v-if="video.status == 'downloading'">
      <h6 class="video-title">{{ video.title }}</h6>

      <div class="d-flex">
        <div class="bar-item time"><i class="fa-regular fa-clock"></i> {{ $filters.formatTime(video.length_seconds) }}
        </div>
        <div class="bar-item size"><i class="fa-solid fa-ruler-horizontal"></i> {{
          $filters.formatSize(video.size_in_bytes)
        }}</div>
        <div class="bar-item">
          <i class="fa-solid fa-down-long"></i>
          <div class="progress">
            <div class="progress-bar" role="progressbar" :style="{ width: video.progress + '%' }" aria-valuemin="0"
              aria-valuemax="100"></div>
          </div>
        </div>
      </div>

    </div>
    <div class="body" v-if="video.status == 'paused'">
      <h6 class="video-title">{{ video.title }}</h6>

      <div class="d-flex">
        <div class="bar-item time"><i class="fa-regular fa-clock"></i> {{ $filters.formatTime(video.length_seconds) }}
        </div>
        <div class="bar-item size"><i class="fa-solid fa-ruler-horizontal"></i> {{
          $filters.formatSize(video.size_in_bytes)
        }}</div>
        <div class="bar-item">
          <i class="fa-solid fa-down-long text-warning"></i>
          <div class="progress">
            <div class="progress-bar" role="progressbar" :style="{ width: video.progress + '%' }" aria-valuemin="0"
              aria-valuemax="100"></div>
          </div>
        </div>
      </div>

    </div>
    <div class="body" v-if="video.status == 'postponed'">
      <h6 class="video-title">{{ video.title }}</h6>

      <div class="d-flex">
        <div class="bar-item"><i class="fa-solid fa-circle-info"></i> Downloading is postponed</div>
      </div>
    </div>
    <div class="body" v-if="video.status == 'failed'">
      <h6 class="video-title">{{ video.title }}</h6>

      <div class="d-flex">
        <small style="font-size: 11px" class="text-danger">Failed</small>
      </div>
    </div>
    <div class="body" v-if="video.status == 'downloaded'">
      <h6 class="video-title">{{ video.title }}</h6>

      <div class="d-flex">
        <div class="bar-item time"><i class="fa-regular fa-clock"></i> {{ $filters.formatTime(video.length_seconds) }}
        </div>
        <div class="bar-item size"><i class="fa-solid fa-ruler-horizontal"></i> {{
          $filters.formatSize(video.size_in_bytes)
        }}</div>
        <div class="bar-item"><i class="fa-brands fa-youtube"></i> {{ video.format + ' . ' + video.quality_label }}
        </div>
      </div>

    </div>

    <div class="actions">
      <div class="close" @click="downloadsStore.removeDownloadItem(video)">❌</div>
      <div class="resume" v-if="video.status == 'paused'">Resume</div>
    </div>
  </section>
</template>

<script setup>
import { ref } from "vue";
import { useDownloadsStore } from "../stores/DownloadsStore";
import ContextMenu from "@imengyu/vue3-context-menu";
import { writeText } from '@tauri-apps/api/clipboard';

const downloadsStore = useDownloadsStore();
const selected = ref(null);

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
          downloadsStore.showInFolder(video);
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
          downloadsStore.removeDownloadItem(video);
        }
      },
      {
        label: "Delete File",
        onClick: () => {
          downloadsStore.deleteDownloadItem(video);
        }
      },
      {
        label: "Remove All",
        onClick: () => {
          downloadsStore.removeAllDownloads();
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
  width: 90px !important;
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
