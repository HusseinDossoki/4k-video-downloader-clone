<template>
  <RouterView />
</template>

<script setup>
import { checkUpdate, installUpdate } from "@tauri-apps/api/updater";
import { relaunch } from "@tauri-apps/api/process";
import { useAppUpdatesStore } from "./stores/AppUpdatesStore";

const appUpdatesStore = useAppUpdatesStore();
// checkTheUpdates();

async function checkTheUpdates() {
  if (appUpdatesStore.isUpdatesChecked) return;
  appUpdatesStore.setUpdatesChecked();
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
}
</script>