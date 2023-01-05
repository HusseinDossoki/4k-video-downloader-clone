import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/tauri";

export const useSmartModeStore = defineStore("smartModeStore", {
  state: () => ({
    id: null,
    enabled: false,
    format: null,
    quality: null,
    directory: null,
    loading: false,
    errors: []
  }),
  getters: {

  },
  actions: {
    async init() {
      this.loading = true;
      this.errors = [];

      return invoke("get_smart_mode")
        .then(res => {
          this.id = res.id;
          this.enabled = res.enabled;
          this.format = res.format;
          this.quality = res.quality;
          this.directory = res.directory;
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
    async update(enabled, format, quality, directory) {
      this.loading = true;
      this.errors = [];

      return invoke("update_smart_mode",
        {
          params: {
            id: this.id,
            enabled: enabled,
            format: format,
            quality: quality,
            directory: directory,
          }
        })
        .then(res => {
          this.id = res.id;
          this.enabled = res.enabled;
          this.format = res.format;
          this.quality = res.quality;
          this.directory = res.directory;
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