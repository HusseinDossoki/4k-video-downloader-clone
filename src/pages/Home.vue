<template>
  <AppHeader/>
  <EmptyList v-if="empty" />
  <VideosList v-if="!empty" />
  <AppFooter/>
</template>

<script setup>
import AppHeader from '../components/AppHeader.vue';
import EmptyList from '../components/EmptyList.vue';
import AppFooter from '../components/AppFooter.vue';
import VideosList from '../components/VideosList.vue';
import {ref} from "vue";

const empty = ref(false);

</script>

<style scoped>

</style>
