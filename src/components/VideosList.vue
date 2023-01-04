<template>
  <section>
    <VideoListItem v-for="(video, index) in videos" :index="index" :key="index" />
  </section>
</template>

<script setup>
import VideoListItem from './VideoListItem.vue';
import { ref } from "vue";

const videos = ref([1, 2, 1, 1, 1, 1]);
</script>

<style scoped>

</style>
