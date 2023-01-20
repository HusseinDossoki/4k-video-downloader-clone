<template>
  <RouterView />
</template>

<script setup>
import { checkUpdate, installUpdate } from "@tauri-apps/api/updater";
import { relaunch } from "@tauri-apps/api/process";
import { appWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/tauri";

appWindow.listen("resume_downloading_all_pending", e => invoke("download_all_pending"));
appWindow.listen("check_app_update", async e => {
  try {
    const { shouldUpdate, manifest } = await checkUpdate();
    if (shouldUpdate) {
      // display dialog
      await installUpdate();
      // install complete, restart the app
      await relaunch();
    }
  } catch (error) {
    console.warn(error);
  }
});
appWindow.emit("js_listeners_ready");

</script>