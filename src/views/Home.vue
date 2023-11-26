<script setup lang="ts"></script>

<template>
  <div class="flex flex-col text-center items-center px-24">
    <div class="home-logo">
      <img class="active-logo" src="/images/active-logo.svg" />
      <img class="inactive-logo" src="/images/idle-logo.svg" />
    </div>
    <h1 class="text-[150px] uppercase tracking-widest">Grawitometr</h1>
    <p class="text-[75px]">
      Stań na wagę, poczekaj na znak i podskocz jak nawyżej potrafisz Zobaczysz
      jak wysoko wzniósł byś się na innych planetach
    </p>
  </div>
</template>

<style scoped lang="scss">
.home-logo {
  width: 800px;
  height: 800px;
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
