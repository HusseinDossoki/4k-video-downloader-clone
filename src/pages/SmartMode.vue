<template>
  <div class="root">
    <header>
      Smart Mode allow you to download videos in one click. You just have to choose format, quality and output directory
      only one time and the application will apply your settings to all downloads.
    </header>
    <section>
      <div class="form-check">
        <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault">
        <label class="form-check-label" for="flexCheckDefault">
          Enable Smart Mode
        </label>
      </div>

      <div class="row mt-2">
        <div class="col-2">
          <label>Format</label>
        </div>
        <div class="col-10">
          <select class="w-100">
            <option value="MP4">MP4 - Video</option>
            <option value="MP4">MKV - Video</option>
            <option value="MP4">MP3 - Video</option>
          </select>
        </div>
      </div>

      <div class="row mt-3">
        <div class="col-2">
          <label>Quality</label>
        </div>
        <div class="col-10">
          <select class="w-100">
            <option value="MP4">MP4 - Video</option>
            <option value="MP4">MKV - Video</option>
            <option value="MP4">MP3 - Video</option>
          </select>
          <small class="d-block mt-1">Some videos may not be available in the selected quality. In such cases, the videos
            will
            be
            downloaded in the closest quality to the one you specified.</small>
        </div>
      </div>

      <div class="row mt-3">
        <div class="col-2">
          <label>Directory</label>
        </div>
        <div class="col-10">
          <span class="path">/Users/husseindossoki/Downloads</span>
          <button class="button browse">Browse</button>
        </div>
      </div>

    </section>
    <footer>
      <button class="button mx-2" @click="closeWindow">Cancel</button>
      <button class="button">Ok</button>
    </footer>
  </div>
</template>

<script setup>
import { appWindow } from '@tauri-apps/api/window';

async function closeWindow() {
  await appWindow.close();
}

</script>

<style scoped>
.root {
  background-color: color(srgb 0.1843 0.1843 0.1843);
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
  background-color: #1e1e1e;
  border-bottom: 1px solid white;
}

select {
  -webkit-appearance: menulist-button;
  height: 20px;
}

.button {
  width: 70px;
  background-color: color(srgb 0.396 0.3961 0.3961);
  border: none;
  border-radius: 5px;
  color: white;
}

.path {
  background-color: color(srgb 0.1608 0.1608 0.1608);
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
</style>
