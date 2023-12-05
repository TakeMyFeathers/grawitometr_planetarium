<script setup lang="ts">
import { onMounted, ref } from "vue";
import router from "../router";
import useSerialStore from "../stores/serial";
import anime from "animejs";
let active = ref(false);

const MIN_WEIGHT = 20;
const LOADING_DURATION = 5000;

let loadingTimer: number;
let resetTimer: number;

const serial = useSerialStore();

function nextStage() {
  router.push({ path: "/prepare" });
  active.value = false;
}

const animation = anime({
  targets: ".active-logo",
  clipPath: ["inset(0)", "inset(100% 0 0 0)"],
  easing: "linear",
  direction: "reverse",
  loop: true,
  duration: LOADING_DURATION,
  autoplay: false,
});

// onMounted(() => {
//   animation.play();
// });

if (serial.value > MIN_WEIGHT && !active.value) {
  active.value = true;

  loadingTimer = setTimeout(() => {
    nextStage();
  }, LOADING_DURATION);
}

serial.$subscribe((_mutation, state) => {
  const isWeightSufficient = state.value > MIN_WEIGHT;
  const isActive = active.value;

  if (!isActive && isWeightSufficient) {
    active.value = true;

    loadingTimer = setTimeout(() => {
      nextStage();
    }, LOADING_DURATION);
  }

  if (isActive && !isWeightSufficient) {
    // resetTimer = setTimeout(() => {
    clearTimeout(loadingTimer);
    active.value = false;
    // }, 1000);
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
