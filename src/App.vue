<script setup lang="ts">
import Footer from "./components/Footer.vue";
</script>

<template>
  <router-view v-slot="{ Component }">
    <component :is="Component" />
  </router-view>
  <Footer />
</template>
