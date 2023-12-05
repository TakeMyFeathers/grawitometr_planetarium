<script setup lang="ts">
import { onMounted } from "vue";
import router from "../router";
import useSerialStore from "../stores/serial";
import anime from "animejs";

const MIN_WEIGHT = 20;
const LOADING_DURATION = 5000;
const RESET_DURATION = 1000;

const resetTimer = {
  active: false,
  timer: 0,
  reset: function () {
    if (!this.active) return;
    clearTimeout(this.timer);
    this.active = false;
  },
  start: function (cb: Function, duration: number) {
    if (this.active) return;
    this.timer = setTimeout(cb, duration);
    this.active = true;
  },
};

const serial = useSerialStore();

function nextStage() {
  router.push({ path: "/prepare" });
}

onMounted(() => {
  const animation = anime({
    targets: ".active-logo",
    clipPath: ["inset(0)", "inset(100% 0 0 0)"],
    easing: "linear",
    direction: "reverse",
    duration: LOADING_DURATION,
    autoplay: false,
    complete: () => {
      nextStage();
    },
  });

  if (serial.value > MIN_WEIGHT) {
    animation.play();
  }

  serial.$subscribe((_, state) => {
    const isWeightSufficient = state.value > MIN_WEIGHT;

    if (!isWeightSufficient) {
      animation.pause();
      resetTimer.start(() => {
        animation.restart();
        animation.pause();
      }, RESET_DURATION);
      return;
    }

    resetTimer.reset();
    animation.play();
  });
});
</script>

<template>
  <div class="flex flex-col text-center items-center px-24">
    <div
      class="home-logo mb-[120px] relative grid h-[400px] w-[400px] lg:w-[800px] lg:h-[800px]"
    >
      <img class="active-logo" src="/images/active-logo.svg" />
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
  img {
    grid-area: 1/2;
    width: 100%;
    height: 100%;
  }

  .active-logo {
    clip-path: inset(100% 0 0 0);
  }
}
</style>
