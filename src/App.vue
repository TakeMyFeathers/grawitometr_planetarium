<template>
  <router-view v-slot="{ Component }">
    <transition name="scale" mode="out-in">
      <component :is="Component" />
    </transition>
  </router-view>

</template>

<script setup lang="ts">
import { listen} from '@tauri-apps/api/event';
import useSerialStore from './stores/serial';

listen('serial-read', (event) => {
  const serial = useSerialStore();

  let prev_state = serial.value;

  serial.value = Math.max(...(event.payload as Uint32Array));

  //Jeżeli z jakiegoś błędu będzie wartość infinity to sprawdzaj póki jej nie będzie
  if(serial.value == Number.POSITIVE_INFINITY || serial.value == Number.NEGATIVE_INFINITY)
  {
    serial.value = prev_state
  }

  event.payload = [];


})

</script>

<style scoped lang="scss">
.scale-enter-active,
.scale-leave-active {
  transition: all 0.5s ease;
}

.scale-enter-from,
.scale-leave-to {
  opacity: 0;
  transform: scale(0.9);
}
</style>
 