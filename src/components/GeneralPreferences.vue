<template>
  <section class="toggles">
    <div class="form-check m-auto">
      <input class="form-check-input" type="checkbox" v-model="store.prevent_system_sleep" id="1">
      <label class="form-check-label" for="1">
        Prevent system sleep until downloads are finished
      </label>
    </div>
    <div class="form-check m-auto">
      <input class="form-check-input" type="checkbox" v-model="store.create_playlist_subdirectory" id="2">
      <label class="form-check-label" for="2">
        Create subdirectory for downloaded playlists and channels
      </label>
    </div>
    <div class="form-check m-auto">
      <input class="form-check-input" type="checkbox" v-model="store.numerate_playlists_files" id="3">
      <label class="form-check-label" for="3">
        Numerate files in playlists
      </label>
    </div>
    <div class="form-check m-auto">
      <input class="form-check-input" type="checkbox" v-model="store.skip_playlists_duplicates" id="4">
      <label class="form-check-label" for="4">
        Skip duplicates in playlists
      </label>
    </div>
    <div class="form-check m-auto">
      <input class="form-check-input" type="checkbox" v-model="store.remove_downloaded_items" id="5">
      <label class="form-check-label" for="5">
        Remove downloaded items from the list
      </label>
    </div>
  </section>
  <hr>
  <section>
    <div class="row mt-2">
      <div class="col-2">
        <label>Language</label>
      </div>
      <div class="col-10">
        <select class="w-100">
          <option value="MP4">System</option>
          <option value="MP4">English</option>
        </select>
      </div>
    </div>
  </section>
</template>

<script setup>
import { useGeneralPreferencesStore } from "../stores/GeneralPreferencesStore";
const store = useGeneralPreferencesStore();
</script>

<style scoped>
section {
  padding: 20px;
  font-size: 13px;
}

section.toggles {
  padding-left: 100px;
  padding-right: 100px;
}

select {
  -webkit-appearance: menulist-button;
  height: 25px;
}
</style>
