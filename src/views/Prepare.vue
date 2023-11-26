<script setup lang="ts">
import anime from "animejs";
import { nextTick } from "vue";

const ml4 = {
  opacityIn: [0, 1],
  scaleIn: [0.2, 1],
  scaleOut: 3,
  durationIn: 800,
  durationOut: 600,
  delay: 500,
};

nextTick(() => {
  anime
    .timeline({ loop: true })
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
    })
    .add({
      targets: ".ml4 .letters-3",
      opacity: 0,
      scale: ml4.scaleOut,
      duration: ml4.durationOut,
      easing: "easeInExpo",
      delay: ml4.delay,
    })
    .add({
      targets: ".ml4",
      opacity: 0,
      duration: 500,
      delay: 500,
    });
});
</script>

<template>
  <div class="flex flex-col text-center justify-center items-center px-24">
    <h1 class="ml4">
      <span class="letters letters-1">Do skoku</span>
      <span class="letters letters-2">Gotowi</span>
      <span class="letters letters-3">Skacz!</span>
    </h1>
  </div>
</template>

<style scoped lang="scss">
.ml4 {
  position: relative;
  display: grid;
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
