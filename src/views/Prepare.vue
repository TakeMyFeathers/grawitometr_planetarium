<script setup lang="ts">
import anime from "animejs";
import { onMounted } from "vue";
import useSerialStore from "../stores/serial";
import useDurationStore from "../stores/duration";
import router from "../router";

const MIN_WEIGHT = 20;

const duration = useDurationStore();
let animation_completed = false;
let jumpStartTime = 0;
let jumpFinishTime = 0;

const ml4 = {
  opacityIn: [0, 1],
  scaleIn: [0.2, 1],
  scaleOut: 3,
  durationIn: 800,
  durationOut: 600,
  delay: 500,
};

const serial = useSerialStore();

serial.$subscribe((_mutation, state) => {
  if (!animation_completed) return;
  if (state.value < MIN_WEIGHT && !jumpStartTime) {
    if (state.value < MIN_WEIGHT / 2) jumpStartTime = Date.now();
  } else if (
    (state.value > MIN_WEIGHT && jumpStartTime && !jumpFinishTime) ||
    (state.value > MIN_WEIGHT &&
      jumpFinishTime == jumpStartTime &&
      jumpStartTime)
  ) {
    jumpFinishTime = Date.now();
  }
  if (jumpStartTime == jumpFinishTime || jumpFinishTime - jumpStartTime <= 50) {
    jumpFinishTime = 0;
  }

  if (!jumpFinishTime || !jumpStartTime) return;
  else {
    duration.value = jumpFinishTime - jumpStartTime;
    console.log("Wartość Duration:");
    console.log(duration.value);
    router.push({ path: "/overview" });
  }
});

const animation = anime.timeline({
  complete: () => {
    animation_completed = true;
  },
});

onMounted(() => {
  animation
    .add({
      targets: ".ml4 .letters-1",
      opacity: ml4.opacityIn,
      scale: ml4.scaleIn,
      duration: ml4.durationIn,
    })
    .add({
      targets: ".ml4 .letters-1",
      opacity: 0,
      scale: ml4.scaleOut,
      duration: ml4.durationOut,
      easing: "easeInExpo",
      delay: ml4.delay,
    })
    .add({
      targets: ".ml4 .letters-2",
      opacity: ml4.opacityIn,
      scale: ml4.scaleIn,
      duration: ml4.durationIn,
    })
    .add({
      targets: ".ml4 .letters-2",
      opacity: 0,
      scale: ml4.scaleOut,
      duration: ml4.durationOut,
      easing: "easeInExpo",
      delay: ml4.delay,
    })
    .add({
      targets: ".ml4 .letters-3",
      opacity: ml4.opacityIn,
      scale: ml4.scaleIn,
      duration: ml4.durationIn,
    });
  animation.restart();
});
</script>

<template>
  <div
    class="flex flex-col text-center justify-center items-center px-24 overflow-hidden"
  >
    <h1 class="ml4">
      <span class="letters letters-1">Przygotuj się</span>
      <span class="letters letters-2">Gotowy?</span>
      <span class="letters letters-3">Skacz!</span>
    </h1>
  </div>
</template>

<style scoped lang="scss">
.ml4 {
  position: relative;
  display: grid;
  align-items: center;
  font-size: 220px;
  letter-spacing: 0.15em;
  text-transform: uppercase;
}

.letters {
  grid-area: 1/2;
  opacity: 0;
}

.letters-1 {
  color: #fd5c1e;
}

.letters-2 {
  color: #2ba9fc;
}

.letters-3 {
  color: #fed62c;
}
</style>
