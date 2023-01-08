import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/tauri";

/**
 * We need a way to watch the state and then update them in db.
 * There are some solutions for this case (Setup Store, Proxy, Object.assign)
 * https://pinia.vuejs.org/core-concepts/#setup-stores => Setup Store
 * https://github.com/vuejs/pinia/discussions/794 => (Proxy, Object.assign)
 * We will use the Proxy solution
 */
const useSmartModeStoreFactory = defineStore("smartModeStore", {
  state: () => ({
    id: null,
    enabled: false,
    format: null,
    quality: null,
    directory: null,
    loading: false,
    lookups: {},
    errors: []
  }),
  getters: {

  },
  actions: {
    async init_lookups() {
      this.loading = true;
      this.errors = [];

      return invoke("get_smart_mode_lookups")
        .then(res => {
          this.lookups = res;
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
    refreshState(newState) {
      this.id = newState.id;
      this.enabled = newState.enabled;
      this.format = newState.format;
      this.quality = newState.quality;
      this.directory = newState.directory;
    }
  }
});

let store = null;
export const useSmartModeStore = new Proxy(useSmartModeStoreFactory, {
  apply: (target, thisArg, argumentsList) => {
    if (!store) {
      store = target.apply(thisArg, argumentsList);
      store.init_lookups();
      store.init();
    }

    return store;
  }
});