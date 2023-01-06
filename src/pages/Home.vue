<template>
  <AppHeader />
  <EmptyList v-if="downloadsStore.emptyList" />
  <VideosList v-if="!downloadsStore.emptyList" />
  <AppFooter />
</template>

<script setup>
import AppHeader from '../components/AppHeader.vue';
import EmptyList from '../components/EmptyList.vue';
import AppFooter from '../components/AppFooter.vue';
import VideosList from '../components/VideosList.vue';
import { useDownloadsStore } from "../stores/DownloadsStore";

const downloadsStore = useDownloadsStore();

</script>

<style scoped>

</style>
