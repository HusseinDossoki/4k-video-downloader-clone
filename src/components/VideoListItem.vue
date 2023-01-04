<template>
  <section class="d-flex" :class="{ odd: index % 2 == 0 }">
    <img src="../assets/empty-list.png" alt="video image">
    <div class="body">
      <h6 class="video-title">Demo youtube video title</h6>
      <div class="d-flex">
        <div class="bar-item">🕑 03:00</div>
        <div class="bar-item">ᴞ 66.5 MB</div>
        <div class="bar-item">⬇
          <div class="progress">
            <div class="progress-bar" role="progressbar" style="width: 25%" aria-valuenow="25" aria-valuemin="0"
              aria-valuemax="100"></div>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup>
import { defineProps } from "vue";
const props = defineProps({
  index: Number
});
</script>

<style scoped>
section {
  background-color: transparent;
  color: white;
  height: 58px;
  padding: 5px;
  overflow: hidden;
}

section.active {
  background-color: blue;
}

section.odd {
  background-color: color(srgb 0.1608 0.1608 0.1608);
}

.body {
  margin-left: 10px;
}

.body .video-title {
  font-size: 13px;
}

.bar-item {
  margin-right: 10px;
  font-size: 12px;
  color: rgb(148, 147, 147);
  font-weight: 500;
}

img {
  width: 90px;
  /* height: 100%; */
  margin: 0;
  padding: 0;
  border-radius: 2px;
}

.progress {
  width: 150px;
  height: 6px;
  border-radius: 15px;
  display: inline-flex;
  background-color: color(srgb 0.247 0.2471 0.2471);
}
</style>
