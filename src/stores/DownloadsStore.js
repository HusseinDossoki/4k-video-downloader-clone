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
    downloads: [],

    // Helpers
    loading: false,
    errors: []
  }),
  getters: {
    isEmptyList(state) {
      return !state.downloads || state.downloads.length == 0;
    },
    total(state) {
      return state.downloads.length;
    }
  },
  actions: {
    async init() {
      this.loading = true;
      this.errors = [];

      return invoke("get_downloads")
        .then(res => {
          this.downloads = res;
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
    async showInFolder(downloadItem) {
      this.loading = true;
      this.errors = [];

      return invoke("show_in_folder", { path: `${downloadItem.directory}/${downloadItem.file_name}` })
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
    async removeDownloadItem(downloadItem) {
      this.loading = true;
      this.errors = [];

      return invoke("remove_download_item", { id: downloadItem.id })
        .then(res => {
          const targetIndex = this.downloads.indexOf(x => x.id == downloadItem.id);
          if (targetIndex > -1) {
            this.downloads.splice(targetIndex, 1);
          }
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
    async deleteDownloadItem(downloadItem) {
      this.loading = true;
      this.errors = [];

      return invoke("delete_file", { path: `${downloadItem.directory}/${downloadItem.file_name}`, id: downloadItem.id })
        .then(res => {
          const targetIndex = this.downloads.indexOf(x => x.id == downloadItem.id);
          if (targetIndex > -1) {
            this.downloads.splice(targetIndex, 1);
          }
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
    async removeAllDownloads() {
      this.loading = true;
      this.errors = [];

      return invoke("remove_all_downloads")
        .then(res => {
          this.downloads = [];
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
    async getVideoDetails(url) {
      return invoke("get_video_details", { url: url });
    },
    async queueNewDownload(options) {
      this.loading = true;
      this.errors = [];

      console.log(options);
      return invoke("queue_new_download", {
        newDownloadItem: options
      })
        .then(res => {
          this.downloads = [res, ...this.downloads];
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
    async parsingVideo(downloadItem) {
      this.loading = true;
      this.errors = [];

      return invoke("parsing_video", {
        download_item: downloadItem
      })
        .then(res => {
          this.downloads = this.downloads.map(item => item.id == downloadItem.id ? res : item);
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
    async downloadVideo(downloadItem) {
      this.loading = true;
      this.errors = [];

      return invoke("download_video", {
        download_item: downloadItem
      })
        .then(res => {
          this.downloads = this.downloads.map(item => item.id == downloadItem.id ? { ...item, status: 'downloading' } : item);
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
    update_download_progress(payload) {
      let target = this.downloads?.find(x => x.id == payload.id);
      if (!target) return;
      target.progress = payload.current_chunk / target.size_in_bytes * 100;
    },
    afterDownloadCompleted(payload) {
      let target = this.downloads?.find(x => x.id == payload);
      if (!target) return;
      target.status = 'downloaded';
    },
  }
});


let store = null;
export const useDownloadsStore = new Proxy(useDownloadsStoreFactory, {
  apply: (target, thisArg, argumentsList) => {
    if (!store) {
      store = target.apply(thisArg, argumentsList);
      store.init();

      appWindow.listen('on_download_progress', event => store.update_download_progress(event.payload));
      appWindow.listen('after_download_completed', event => store.afterDownloadCompleted(event.payload));
    }

    return store;
  }
});