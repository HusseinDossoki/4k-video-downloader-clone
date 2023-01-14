<template>
  <AppHeader />
  <EmptyList v-if="downloadsStore.isEmptyList" />
  <VideosList v-if="!downloadsStore.isEmptyList" />
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
