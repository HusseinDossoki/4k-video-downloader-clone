import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from '@tauri-apps/api/window';
import { emit } from '@tauri-apps/api/event';

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
    errors: [],
    interval: null
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
          this.initQueuProcess();
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

      return invoke("show_in_folder", { directory: downloadItem.directory, fileName: downloadItem.file_name })
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
          this.downloads = this.downloads.filter(x => x.id != downloadItem.id);
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

      return invoke("delete_file", { directory: downloadItem.directory, fileName: downloadItem.file_name, id: downloadItem.id })
        .then(res => {
          this.downloads = this.downloads.filter(x => x.id != downloadItem.id);
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
    async queueParsedDownload(options) {
      this.loading = true;
      this.errors = [];

      return invoke("queue_parsed_download", {
        options: options
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
        downloadItem: downloadItem
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

      this.downloads = this.downloads.map(item => item.id == downloadItem.id ? { ...item, status: 'downloading' } : item);

      return invoke("download_video", {
        downloadItem: downloadItem
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
    updateDownloadProgress(payload) {
      this.downloads = this.downloads.map(item => item.id == payload.id ? { ...item, progress: payload.current_chunk / item.size_in_bytes * 100 } : item);
    },
    onDownloadStatusChanges(payload) {
      this.downloads = this.downloads.map(item => item.id == payload.id ? { ...item, status: payload.status } : item);
    },
    initQueuProcess() {
      if (this.interval) return;
      this.interval = setInterval(() => {

        let currentDownloading = this.downloads.filter(x => x.status == 'downloading').length;

        this.downloads.forEach(item => {
          if (item.status == 'queued') {
            this.parsingVideo(item);
          } else if (item.status == 'parsed' && currentDownloading == 0) {
            currentDownloading++;
            this.downloadVideo(item);
          }
        });

      }, 2000);
    }
  }
});


let store = null;
export const useDownloadsStore = new Proxy(useDownloadsStoreFactory, {
  apply: (target, thisArg, argumentsList) => {
    if (!store) {
      store = target.apply(thisArg, argumentsList);
      store.init();

      appWindow.listen('on_download_progress', event => store.updateDownloadProgress(event.payload));
      appWindow.listen('on_download_status_changes', event => store.onDownloadStatusChanges(event.payload));
    }

    return store;
  }
});