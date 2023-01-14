<template>
  <div class="p-4">
    <section class="d-flex">
      <p>Notifications</p>
      <div class="toggles">
        <div class="form-check m-auto">
          <input class="form-check-input" type="checkbox" id="1"
            v-model="store.show_notification_when_downloads_complete">
          <label class="form-check-label" for="1">
            Show notification when downloads complete
          </label>
        </div>
        <div class="form-check m-auto">
          <input class="form-check-input" type="checkbox" id="2"
            v-model="store.show_notification_about_new_videos_from_channel_subscriptions">
          <label class="form-check-label" for="2">
            Show notification about new videos from channel subscriptions
          </label>
        </div>
        <div class="form-check m-auto">
          <input class="form-check-input" type="checkbox" id="3" v-model="store.play_notification_sound">
          <label class="form-check-label" for="3">
            Play notification sound
          </label>
        </div>
        <div class="form-check m-auto">
          <input class="form-check-input" type="checkbox" id="4" v-model="store.show_progress_on_dock_icon">
          <label class="form-check-label" for="4">
            Show progress on dock icon
          </label>
        </div>
      </div>
    </section>
    <hr>
    <section class="d-flex">
      <p>Confirmations</p>
      <div class="toggles">
        <div class="form-check m-auto">
          <input class="form-check-input" type="checkbox" id="5"
            v-model="store.confirm_app_exit_if_there_are_incomplete_downloads">
          <label class="form-check-label" for="5">
            Ask to confirm the app exit if there are incomplete downloads
          </label>
        </div>
        <div class="form-check m-auto">
          <input class="form-check-input" type="checkbox" id="6" v-model="store.confirm_before_item_deleting">
          <label class="form-check-label" for="6">
            Ask confirmation before item deleting
          </label>
        </div>
        <div class="form-check m-auto">
          <input class="form-check-input" type="checkbox" id="7" v-model="store.confirm_before_subscription_deleting">
          <label class="form-check-label" for="7">
            Ask confirmation before subscription deleting
          </label>
        </div>
        <div class="form-check m-auto">
          <input class="form-check-input" type="checkbox" id="8"
            v-model="store.ask_to_select_between_single_video_and_playlist_in_case_of_ambiguity">
          <label class="form-check-label" for="8">
            Ask to select between a single video and playlist in case of ambiguity
          </label>
        </div>
        <div class="form-check m-auto">
          <input class="form-check-input" type="checkbox" id="9"
            v-model="store.ask_to_download_channel_if_multiple_videos_were_downloaded_from_it">
          <label class="form-check-label" for="9">
            Ask to download channel if multiple videos were downloaded from it
          </label>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup>
import { watch } from "vue";
import { usePreferencesStore } from "../stores/PreferencesStore";
const store = usePreferencesStore();

store.init().then(res => {
  watch(() => [
    store.show_notification_when_downloads_complete,
    store.show_notification_about_new_videos_from_channel_subscriptions,
    store.play_notification_sound,
    store.show_progress_on_dock_icon,
    store.confirm_app_exit_if_there_are_incomplete_downloads,
    store.confirm_before_item_deleting,
    store.confirm_before_subscription_deleting,
    store.ask_to_select_between_single_video_and_playlist_in_case_of_ambiguity,
    store.ask_to_download_channel_if_multiple_videos_were_downloaded_from_it,
  ], () => {
    store.updateNotifications();
  }, { deep: true });
});

</script>

<style scoped lang="scss">
section {
  font-size: 13px;
  color: rgb(234, 231, 231);

  p {
    width: 95px;
    margin-right: 15px;
    text-align: right;
    font-weight: bold;
  }
}
</style>
