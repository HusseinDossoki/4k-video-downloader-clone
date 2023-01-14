import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/tauri";

/**
 * We need a way to watch the state and then update them in db.
 * There are some solutions for this case (Setup Store, Proxy, Object.assign)
 * https://pinia.vuejs.org/core-concepts/#setup-stores => Setup Store
 * https://github.com/vuejs/pinia/discussions/794 => (Proxy, Object.assign)
 * We will use the Proxy solution
 */
const usePreferencesStoreFactory = defineStore("preferencesStore", {
  state: () => ({
    id: null,
    // General
    threads_number: null,
    prevent_system_sleep: false,
    create_playlist_subdirectory: false,
    numerate_playlists_files: false,
    skip_playlists_duplicates: false,
    generate_playlists_m3u: false,
    embed_video_subtitles: false,
    search_audio_tags: false,
    remove_downloaded_items: false,
    submit_download_statistics: false,
    install_beta_version: false,
    language: null,

    // Connection
    speed: null,

    // Notifications
    show_notification_when_downloads_complete: false,
    show_notification_about_new_videos_from_channel_subscriptions: false,
    play_notification_sound: false,
    show_progress_on_dock_icon: false,
    confirm_app_exit_if_there_are_incomplete_downloads: false,
    confirm_before_item_deleting: false,
    confirm_before_subscription_deleting: false,
    ask_to_select_between_single_video_and_playlist_in_case_of_ambiguity: false,
    ask_to_download_channel_if_multiple_videos_were_downloaded_from_it: false,

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

      return invoke("get_preferences")
        .then(res => {
          this.$state = { ...this.$state, ...res };
          this.loading = false;
        })
        .catch(err => {
          function onlyUnique(value, index, self) {
            return self.indexOf(value) === index;
          }
          this.errors.push(err);
          this.errors = this.errors.filter(x => onlyUnique);
          this.loading = false;
          console.error(err);
        });
    },
    async updateGeneral() {
      this.loading = true;
      this.errors = [];

      return invoke("update_general_preferences",
        {
          params: { ...this.$state }
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
          console.error(err);
        });
    },
    async updateNotifications() {
      this.loading = true;
      this.errors = [];

      return invoke("update_notifications_preferences",
        {
          params: { ...this.$state }
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
          console.error(err);
        });
    },
  }
});


let store = null;
export const usePreferencesStore = new Proxy(usePreferencesStoreFactory, {
  apply: (target, thisArg, argumentsList) => {
    if (!store) {
      store = target.apply(thisArg, argumentsList);
      // store.init();
    }

    return store;
  }
});