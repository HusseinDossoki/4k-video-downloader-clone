<template>
  <div class="root" v-if="videoDetails">
    <header>
      <div class="d-flex">
        <img :src="videoDetails?.thumbnail" class="card-img" alt="image">
        <div class="px-3 w-100">
          <p class="mb-2">{{ $filters.truncate(videoDetails?.title, 53) }}</p>
          <div class="d-flex">
            <div class="bar-item"><i class="fa-regular fa-clock"></i>
              {{ $filters.formatTime(videoDetails?.length_seconds) }}</div>
            <div class="bar-item"><i class="fa-solid fa-link"></i> <a target="_blank" :href="url">{{
              $filters.truncate(url, 40)
            }}</a></div>
          </div>
        </div>
      </div>
    </header>
    <section>
      <div class="d-flex first-section my-2">
        <select v-model="type">
          <option value="video">Download Video</option>
          <option value="audio">Extract Audio</option>
        </select>
        <label>Format:</label>
        <select v-model="format">
          <option v-for="f in formats" :key="f" :value="f">{{ f }}</option>
        </select>
      </div>
      <hr>
      <div class="second-section my-2">
        <div class="d-flex mb-3" v-for="(stream, i) in streams">
          <input type="radio" :id="i" name="fav_stream" :value="stream" v-model="selectedStream" :key="i">
          <label :for="i" class="w-100 d-flex">
            <span class="label-item">{{ $filters.humanize(stream?.quality) }}</span>
            <span class="label-item">{{ stream?.quality_label }}</span>
            <span class="label-item">{{ stream?.format }}</span>
            <span class="label-item">{{ $filters.formatSize(stream?.size_in_bytes) }}</span>
          </label>
        </div>
      </div>
      <hr>
      <div class="section-three">
        <span class="path" v-if="directory">{{ directory }}</span>
        <button class="button browse" @click="browseDirectory">Browse</button>
      </div>
    </section>
    <footer>
      <button class="button mx-2" @click="closeWindow">Cancel</button>
      <button @click="download" class="button submit" :class="{ disabled: !isValid }"
        :disabled="!isValid">Download</button>
    </footer>
  </div>
  <div class="loading" v-if="!videoDetails">
    <div class="spinner-border text-primary" style="width: 3rem; height: 3rem;" role="status">
      <span class="sr-only"></span>
    </div>
    <h5 class="mt-2">Retrieving information</h5>
    <h6>Parsing video...</h6>
  </div>
</template>

<script setup>
import { appWindow } from '@tauri-apps/api/window';
import { open } from '@tauri-apps/api/dialog';
import { ref, watch, computed } from "vue";
import { readText } from '@tauri-apps/api/clipboard';
import { emit } from '@tauri-apps/api/event';
import { useDownloadsStore } from "../stores/DownloadsStore";

const downloadsStore = useDownloadsStore();
const url = ref(null);
const directory = ref(null);
const type = ref("video");
const format = ref(null);
const formats = ref([]);
const streams = ref([]);
const selectedStream = ref(null);
const videoDetails = ref();

readText()
  .then(res => {
    url.value = res;

    downloadsStore.getVideoDetails(url.value)
      .then(res => {
        videoDetails.value = res;
        onTypeChnages();
        console.log(res);
      })
      .catch(err => {
        console.log(err);
      });

  });

watch(type, onTypeChnages);
watch(format, onFormatChanges);

const isValid = computed(() => selectedStream.value && type.value && format.value && directory.value);

function download() {
  const param = {
    url: url.value,
    directory: directory.value,
    format: format.value,
    quality: selectedStream.value.quality,
    quality_label: selectedStream.value.quality_label,
  };

  downloadsStore.queueNewDownload(param).then(x => {
    appWindow.close();
  });
}

function onTypeChnages() {
  selectedStream.value = null;
  if (type.value == 'video') {
    formats.value = videoDetails.value.video_streams?.map(x => x.format);
    streams.value = videoDetails.value.video_streams?.filter(x => x.format == format.value);
  } else if (type.value == 'audio') {
    formats.value = videoDetails.value.audio_streams?.map(x => x.format);
    streams.value = videoDetails.value.audio_streams?.filter(x => x.format == format.value);
  }
  formats.value = [...new Set(formats.value)];
  format.value = formats.value[0] || null;
}
function onFormatChanges() {
  selectedStream.value = null;
  if (type.value == 'video') {
    streams.value = videoDetails.value.video_streams?.filter(x => x.format == format.value);
  } else if (type.value == 'audio') {
    streams.value = videoDetails.value.audio_streams?.filter(x => x.format == format.value);
  }
}

async function closeWindow() {
  await appWindow.close();
}

async function browseDirectory() {
  directory.value = await open({
    title: '4K Downloder Output Directory',
    directory: true,
    defaultPath: '.'
  });
}

</script>

<style scoped lang="scss">
.root {
  background-color: #2F2F2F;
  height: 100%;
  position: fixed;
  bottom: 0;
  top: 0;
  right: 0;
  left: 0;
}

header,
section,
footer {
  font-size: 13px;
  padding: 12px;
  color: white;
}

header {
  height: 70px;
  background-color: #1e1e1e;
  border-bottom: 1px solid white;

  img {
    width: 100px;
    height: 50px;
    object-fit: cover;
  }

  a {
    color: #3F94F1;
  }
}

section {
  .first-section {
    label {
      margin-left: auto;
      margin-right: 5px;
    }
  }
}

.bar-item {
  margin-right: 10px;
  font-size: 12px;
  color: rgb(148, 147, 147);

  i {
    font-size: 15px;
  }
}

.button {
  width: 90px;
  background-color: #656565;
  border: none;
  border-radius: 5px;
  color: white;
}

.path::-webkit-scrollbar {
  display: none;
}

.path {
  background-color: #292929;
  padding: 6px;
  border-radius: 2px;
}

.browse {
  float: right;
}

footer {
  position: fixed;
  bottom: 0;
  right: 0;
  left: 0;
  text-align: right;
}

.button.disabled {
  color: gray;
}

select {
  -webkit-appearance: menulist-button;
  height: 22px;
  width: 165px;
}

.second-section {
  max-height: 210px;
  overflow-y: auto;

  label {
    line-height: 10px;
    margin-left: 10px;

    .label-item:nth-child(1) {
      margin-right: 30px;
      width: 70px;
    }

    .label-item:nth-child(2) {
      font-weight: bold;
      width: 50px;
    }

    .label-item:nth-child(3) {
      margin-left: auto;
      width: 60px;
    }

    .label-item:nth-child(4) {
      margin-left: 45px;
      width: 70px;
    }
  }
}

.loading {
  margin-top: 20%;
  text-align: center;
}

.button.submit {
  background-color: #3478F6;
}
</style>
