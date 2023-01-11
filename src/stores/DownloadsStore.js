import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from '@tauri-apps/api/window';

/**
 * We need a way to watch the state and then update them in db.
 * There are some solutions for this case (Setup Store, Proxy, Object.assign)
 * https://pinia.vuejs.org/core-concepts/#setup-stores => Setup Store
 * https://github.com/vuejs/pinia/discussions/794 => (Proxy, Object.assign)
 * We will use the Proxy solution
 */
const useDownloadsStoreFactory = defineStore("downloadsStore", {
  state: () => ({
    // Data
    list: [],

    // Helpers
    loading: false,
    errors: []
  }),
  getters: {
    emptyList(state) {
      return !state.list || state.list.length == 0;
    }
  },
  actions: {
    update_progress(id, current_chunk) {
      let item = this.list?.find(x => x.id == id);
      if (!item) return;
      item.progress = current_chunk / item.size_in_bytes * 100;
    },
    async init() {
      this.loading = true;
      this.errors = [];

      return invoke("get_downloads")
        .then(res => {
          console.log('Downloads => ', res);
          this.list = res;

          // Set progress
          this.list.forEach(item => {
            item.progress = item.current_chunk / item.size_in_bytes * 100;
          });

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
    async deleteDownloadItem(id) {
      this.loading = true;
      this.errors = [];

      return invoke("delete_download_item", { id: id })
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
export const useDownloadsStore = new Proxy(useDownloadsStoreFactory, {
  apply: (target, thisArg, argumentsList) => {
    if (!store) {
      store = target.apply(thisArg, argumentsList);
      store.init();

      // listen for event to reload the download list
      appWindow.listen('downloads-changed', (event) => {
        store.init();
      });
      appWindow.listen('download-progress', (event) => {
        store.update_progress(event.payload.id, event.payload.current_chunk);
      });
    }

    return store;
  }
});