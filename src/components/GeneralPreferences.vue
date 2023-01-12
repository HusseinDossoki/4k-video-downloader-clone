<template>
  <div class="p-4">
    <section class="d-flex">
      <p>Intensity</p>
      <div class="toggles w-100">
        <select class="w-100 mb-1">
          <option value="1">Safe - 1 thread</option>
          <option value="2">Safe - 2 threads</option>
        </select>
        <small>
          High intensity might increase overall download performance but it may also lead to temporary IP address ban by
          YouTube.
        </small>
      </div>
    </section>
    <hr>
    <section class="d-flex">
      <p></p>
      <div class="toggles">
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
          <input class="form-check-input" type="checkbox" v-model="store.skip_playlists_duplicates" id="4">
          <label class="form-check-label" for="4">
            Generate .m3u file for downloaded playlists
          </label>
        </div>
        <div class="form-check m-auto">
          <input class="form-check-input" type="checkbox" v-model="store.skip_playlists_duplicates" id="4">
          <label class="form-check-label" for="4">
            Emebed subtitles in video file if possible
          </label>
        </div>
        <div class="form-check m-auto">
          <input class="form-check-input" type="checkbox" v-model="store.skip_playlists_duplicates" id="4">
          <label class="form-check-label" for="4">
            Search audio tags based on track title
          </label>
        </div>
        <div class="form-check m-auto">
          <input class="form-check-input" type="checkbox" v-model="store.remove_downloaded_items" id="5">
          <label class="form-check-label" for="5">
            Remove downloaded items from the list
          </label>
        </div>
        <div class="form-check m-auto">
          <input class="form-check-input" type="checkbox" v-model="store.remove_downloaded_items" id="5">
          <label class="form-check-label" for="5">
            Submit download statistics
          </label>
        </div>
      </div>
    </section>
    <hr>
    <section class="d-flex">
      <p></p>
      <div class="toggles w-100" style="margin-left: 11px;">
        <div class="form-check m-auto w-100">
          <input class="form-check-input" type="checkbox" id="1">
          <label class="form-check-label" for="1">
            Install Beta versions
          </label>
        </div>
        <small>
          Beta versions could be less stable than the final ones, however, they grant you access to new features before
          the offical release. Please report bugs and other issues, your feedback helps us improve the application
        </small>
      </div>
    </section>
    <hr>
    <section class="d-flex">
      <p>Language</p>
      <div class="toggles w-100">
        <select class="w-100 mb-1">
          <option value="1">System</option>
          <option value="2">English</option>
        </select>
      </div>
    </section>
  </div>
</template>

<script setup>
import { useGeneralPreferencesStore } from "../stores/GeneralPreferencesStore";
const store = useGeneralPreferencesStore();
</script>

<style scoped lang="scss">
section {
  font-size: 13px;
  color: rgb(234, 231, 231);

  p {
    width: 70px;
    margin-right: 15px;
    text-align: right;
    font-weight: bold;
  }

  select {
    -webkit-appearance: menulist-button;
    height: 25px;
  }
}
</style>