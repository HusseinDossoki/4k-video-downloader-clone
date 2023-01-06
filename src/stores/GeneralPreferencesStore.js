import { defineStore } from "pinia";
import { watch } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

/**
 * We need a way to watch the state and then update them in db.
 * There are some solutions for this case (Setup Store, Proxy, Object.assign)
 * https://pinia.vuejs.org/core-concepts/#setup-stores => Setup Store
 * https://github.com/vuejs/pinia/discussions/794 => (Proxy, Object.assign)
 * We will use the Proxy solution
 */
const useGeneralPreferencesStoreFactory = defineStore("generalPreferencesStore", {
  state: () => ({
    // Data
    id: null,
    prevent_system_sleep: false,
    create_playlist_subdirectory: false,
    numerate_playlists_files: false,
    skip_playlists_duplicates: false,
    remove_downloaded_items: false,

    // Helpers
    loading: false,
    errors: []
  }),
  getters: {

  },
  actions: {
    async init() {
      this.loading = true;
      this.errors = [];

      return invoke("get_general_preferences")
        .then(res => {
          this.id = res.id;
          this.prevent_system_sleep = res.prevent_system_sleep;
          this.create_playlist_subdirectory = res.create_playlist_subdirectory;
          this.numerate_playlists_files = res.numerate_playlists_files;
          this.skip_playlists_duplicates = res.skip_playlists_duplicates;
          this.remove_downloaded_items = res.remove_downloaded_items;
          this.loading = false;
        })
        .catch(err => {
          function onlyUnique(value, index, self) {
            return self.indexOf(value) === index;
          }
          this.errors.push(err);
          this.errors = this.errors.filter(x => onlyUnique);
          this.loading = false;
        });
    },
    async update() {
      this.loading = true;
      this.errors = [];

      return invoke("update_general_preferences",
        {
          params: {
            id: this.id,
            prevent_system_sleep: this.prevent_system_sleep,
            create_playlist_subdirectory: this.create_playlist_subdirectory,
            numerate_playlists_files: this.numerate_playlists_files,
            skip_playlists_duplicates: this.skip_playlists_duplicates,
            remove_downloaded_items: this.remove_downloaded_items,
          }
        })
        .then(res => {
          this.loading = false;
        })
        .catch(err => {
          function onlyUnique(value, index, self) {
            return self.indexOf(value) === index;
          }
          this.errors.push(err);
          this.errors = this.errors.filter(x => onlyUnique);
          this.loading = false;
        });
    },
  }
});


let store = null;
export const useGeneralPreferencesStore = new Proxy(useGeneralPreferencesStoreFactory, {
  apply: (target, thisArg, argumentsList) => {
    if (!store) {
      store = target.apply(thisArg, argumentsList);

      store.init();

      watch(() => [
        store.prevent_system_sleep,
        store.create_playlist_subdirectory,
        store.numerate_playlists_files,
        store.skip_playlists_duplicates,
        store.remove_downloaded_items
      ], store.update, { deep: true });

    }

    return store;
  }
});