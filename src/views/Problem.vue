<template>
  <p class="text-5xl text-red-500 uppercase">{{ error.msg }}</p>
</template>

<script setup lang="ts">
import useErrorStore from '../stores/error'
const error = useErrorStore()
</script>