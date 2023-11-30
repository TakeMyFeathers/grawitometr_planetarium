<script setup lang="ts">
import { Ref, inject, watch } from 'vue';

const serialValue: number = inject('currSerialValue', 0)
watch(serialValue, (newValue, _) => {
  console.log(newValue)
})
</script>

<template>
  <div class="flex flex-col text-center items-center px-24">
    <div class="home-logo h-[400px] w-[400px] lg:w-[800px] lg:h-[800px]">
      <img class="active-logo" src="/images/active-logo.svg" />
      <img class="inactive-logo" src="/images/idle-logo.svg" />
    </div>
    <h1 class="text-6xl lg:text-[150px] uppercase tracking-widest ">Grawitometr</h1>
    <p class="leading-normal p-10 text-4xl lg:text-[75px]">
      Stań na wagę, poczekaj na znak i podskocz jak nawyżej potrafisz. <br />Zobaczysz jak wysoko wzniósł byś się na
      innych
      planetach
    </p>
  </div>
</template>

<style scoped lang="scss">
.home-logo {
  margin-bottom: 120px;
  position: relative;
  display: grid;

  img {
    grid-area: 1/2;
    width: 100%;
    height: 100%;
  }
}
</style>
