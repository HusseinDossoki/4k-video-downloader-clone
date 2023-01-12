<template>
  <footer class="d-flex">
    <div>
      Developed by <strong>
        <a target="_blank" href="https://www.linkedin.com/in/husseindossoki/">Hussein Dossoki</a>
      </strong> ©2022
    </div>
    <div class="text-center" style="margin-left: auto;">
      {{downloadsStore.total}} items
    </div>
  </footer>
</template>

<script setup>
import { useDownloadsStore } from "../stores/DownloadsStore";
const downloadsStore = useDownloadsStore();
</script>

<style scoped>
footer {
  height: 25px;
  bottom: 0;
  width: 100%;
  background-color: #2d2d2d;
  position: fixed;
  color: rgba(238, 238, 238, 0.646);
  font-size: 11px;
  line-height: 21px;
  padding-left: 15px;
  padding-right: 15px;
}
</style>
