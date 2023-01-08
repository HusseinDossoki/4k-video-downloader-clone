<template>
  <div class="root">
    <header>
      Smart Mode allow you to download videos in one click. You just have to choose format, quality and output directory
      only one time and the application will apply your settings to all downloads.
    </header>
    <section>
      <div class="form-check">
        <input class="form-check-input" type="checkbox" v-model="smartModeEnabled" id="smartModeCheckbox">
        <label class="form-check-label" for="smartModeCheckbox">
          Enable Smart Mode
        </label>
      </div>

      <div class="row mt-2" :class="{ disabled: !smartModeEnabled }">
        <div class="col-2">
          <label>Format</label>
        </div>
        <div class="col-10">
          <select class="w-100" v-model="format" :disabled="!smartModeEnabled">
            <option :value="opt" v-for="opt in smartModeStore.lookups.formats">{{ opt }}</option>
          </select>
        </div>
      </div>

      <div class="row mt-3" :class="{ disabled: !smartModeEnabled }">
        <div class="col-2">
          <label>Quality</label>
        </div>
        <div class="col-10">
          <select class="w-100" v-model="quality" :disabled="!smartModeEnabled">
            <option :value="opt" v-for="opt in qualities">{{ opt }}</option>
          </select>
          <small class="d-block mt-1">Some videos may not be available in the selected quality. In such cases, the
            videos
            will
            be
            downloaded in the closest quality to the one you specified.</small>
        </div>
      </div>

      <div class="row mt-3" :class="{ disabled: !smartModeEnabled }">
        <div class="col-2">
          <label>Directory</label>
        </div>
        <div class="col-10">
          <span class="path" v-if="directory">{{ directory }}</span>
          <button class="button browse" @click="browseDirectory" :disabled="!smartModeEnabled">Browse</button>
        </div>
      </div>

    </section>
    <footer>
      <button class="button mx-2" @click="closeWindow">Cancel</button>
      <button class="button" @click="save" :disabled="!isValid" :class="{ disabled: !isValid }">Ok</button>
    </footer>
  </div>
</template>

<script setup>
import { appWindow } from '@tauri-apps/api/window';
import { open } from '@tauri-apps/api/dialog';
import { ref, computed, watch } from "vue";
import { useSmartModeStore } from "../stores/SmartModeStore";

const smartModeStore = useSmartModeStore();

const smartModeEnabled = ref(smartModeStore.enabled);
const format = ref(smartModeStore.format);
const quality = ref(smartModeStore.quality);
const directory = ref(smartModeStore.directory);
const isValid = computed(() => !smartModeEnabled.value || (format.value && quality.value && directory.value));

const qualities = ref([]);
watch(format, (from, to) => {
  if (from && to) {
    if (from?.includes('audio') && !to?.includes('audio')) {
      quality.value = null;
    } else if (to?.includes('audio') && !from?.includes('audio')) {
      quality.value = null;
    }
  }
  // quality.value = null;
  qualities.value = format.value.includes('audio') ? smartModeStore.lookups.audio_qualities : smartModeStore.lookups.quality_labels;
});

watch(smartModeStore.$state, () => {
  smartModeEnabled.value = smartModeStore.enabled;
  format.value = smartModeStore.format;
  quality.value = smartModeStore.quality;
  directory.value = smartModeStore.directory;
}, { deep: true });

async function closeWindow() {
  await appWindow.close();
}

async function save() {
  smartModeStore.update(smartModeEnabled.value, format.value, quality.value, directory.value)
    .then(async res => {
      await appWindow.close();
    })
    .catch(err => {
      console.log('err ==> ', err);
    });
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


.path::-webkit-scrollbar {
  display: none;
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

.row.disabled {
  color: gray;

  .button {
    color: gray;
  }

  select {
    background-color: gray;
  }
}

.button.disabled {
  color: gray;
}
</style>
