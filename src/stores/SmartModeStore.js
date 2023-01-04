import { defineStore } from "pinia";

export const useSmartModeStore = defineStore("smartModeStore", {
  state: () => ({
    smartModeEnabled: false,
    format: null,
    quality: null,
    directory: null
  }),
  getters: {

  },
  actions: {
    async enable(format, quality, directory) {
      if (!format || !quality || !directory) {
        throw 'Format, quality and directory should not be empty.';
      }
      this.smartModeEnabled = true;
      this.format = format;
      this.quality = quality;
      this.directory = directory;
    },
    async disable() {
      this.smartModeEnabled = true;
    }
  }
});