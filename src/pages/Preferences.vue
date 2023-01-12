<template>
  <div class="root">
    <header>
      <div class="link-container">
        <div class="link" @click="selected = 1" :class="{active: selected == 1}">
          <p class="link-icon"><i class="fa-solid fa-sliders"></i></p>
          <p class="link-text">General</p>
        </div>
        <div class="link" @click="selected = 2" :class="{ active: selected == 2 }">
          <p class="link-icon"><i class="fa-solid fa-earth-americas"></i></p>
          <p class="link-text">Connection</p>
        </div>
        <div class="link" @click="selected = 3" :class="{ active: selected == 3 }">
          <p class="link-icon"><i class="fa-regular fa-bell"></i></p>
          <p class="link-text">Notifications</p>
        </div>
      </div>
    </header>
    <section>
      <GeneralPreferences v-if="selected == 1" />
      <ConnectionPreferences v-if="selected == 2" />
      <NotificationsPreferences v-if="selected == 3" />
    </section>
  </div>
</template>

<script setup>
import { ref, watch } from "vue";
import GeneralPreferences from "../components/GeneralPreferences.vue";
import NotificationsPreferences from "../components/NotificationsPreferences.vue";
import ConnectionPreferences from "../components/ConnectionPreferences.vue";
import { appWindow, LogicalSize } from "@tauri-apps/api/window";
const selected = ref(1);

watch(selected, () => {
  switch (selected.value) {
    case 1:
      appWindow.setSize(new LogicalSize(620, 650));
      break;
    case 2:
      appWindow.setSize(new LogicalSize(620, 420));
      break;
    case 3:
      appWindow.setSize(new LogicalSize(620, 370));
      break;
  }
});
</script>

<style scoped lang="scss">
p {
  margin-bottom: 0;
}

.root {
  background-color: #323232;
  height: 100%;
  position: fixed;
  bottom: 0;
  top: 0;
  right: 0;
  left: 0;
}

header {
  height: 57px;
  background-color: #382A27;
  color: white;
}

.link-container {
  display: flex;
}

.link {
  margin-top: 3px;
  padding-left: 15px;
  padding-right: 15px;
  cursor: pointer;
}

.link.active {
  background-color: #463A34;
  color: #3478F6;
}

.link .link-icon {
  color: white;
  border-radius: 50%;
  display: block;
  width: 30px;
  height: 30px;
  text-align: center;
  margin: auto;
  font-size: 20px;
  font-weight: bold;

  i {
    zoom: 1.3;
  }
}

.link .link-text {
  margin-top: 2px;
  text-align: center;
  font-size: 12px;
}
</style>
