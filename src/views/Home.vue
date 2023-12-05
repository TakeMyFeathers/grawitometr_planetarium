<script setup lang="ts">
import { ref } from "vue";
import router from "../router";
import useSerialStore from "../stores/serial";
let active = ref(false);

const MIN_WEIGHT = 20;

let timer: number;
const serial = useSerialStore();

let dateTime1: number;
let dateTime2: number;

if (serial.value > MIN_WEIGHT) {
  active.value = true;
  timer = setTimeout(() => {
    router.push({ path: "/prepare" });
  }, 5000);
}

serial.$subscribe((_mutation, state) => {
  if (state.value > MIN_WEIGHT) {
    active.value = true;
    dateTime1 = Date.now();
    timer = setTimeout(() => {
      router.push({ path: "/prepare" });
    }, 5000);
  }

  if (state.value < MIN_WEIGHT) {
    dateTime2 = Date.now();
    if (dateTime2 - dateTime1 >= 1000) {
      clearTimeout(timer);
      active.value = false;
    }
  }
});
</script>

<template>
  <div class="flex flex-col text-center items-center px-24">
    <div class="home-logo h-[400px] w-[400px] lg:w-[800px] lg:h-[800px]">
      <img
        :class="{ loading: active }"
        class="active-logo"
        src="/images/active-logo.svg"
      />
      <img class="inactive-logo" src="/images/idle-logo.svg" />
    </div>
    <h1 class="text-6xl lg:text-[150px] uppercase tracking-widest">
      Grawitometr
    </h1>
    <p class="leading-normal p-10 text-4xl lg:text-[75px]">
      Stań na wagę, poczekaj na znak i podskocz jak nawyżej potrafisz.
      <br />Zobaczysz jak wysoko wzniósł byś się na innych planetach
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

  .active-logo {
    clip-path: inset(100% 0 0 0);
  }
}

.loading {
  animation: reveal 5s linear infinite reverse;
}

@keyframes reveal {
  0% {
    clip-path: inset(0);
  }

  100% {
    clip-path: inset(100% 0 0 0);
  }
}
</style>
