import { defineStore } from "pinia";

export const useAppUpdatesStore = defineStore("appUpdatesStore", {
  state: () => ({
    isUpdatesChecked: false
  }),
  actions: {
    setUpdatesChecked() {
      this.updatesChecked = true;
    },
  }
})