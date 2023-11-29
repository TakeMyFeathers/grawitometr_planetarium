<script setup lang="ts">
import DevNav from "./components/DevNav.vue";
import Footer from "./components/Footer.vue";
</script>

<template>
  <router-view v-slot="{ Component }">
    <component :is="Component" />
  </router-view>

  <DevNav />
</template>
