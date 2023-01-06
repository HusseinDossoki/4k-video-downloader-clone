import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/tauri";

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
    async init() {
      this.loading = true;
      this.errors = [];

      return invoke("get_downloads")
        .then(res => {
          console.log('Download => ', res);
          this.list = res;
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
    }

    return store;
  }
});