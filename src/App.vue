<template>
  <router-view v-slot="{ Component }">
    <transition name="scale" mode="out-in">
      <component :is="Component" />
    </transition>
  </router-view>

  <DevNav />
</template>

<script setup lang="ts">
import { provide, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import DevNav from "./components/DevNav.vue";

type SerialReadPayload = Uint32Array

interface ListPortsPayload {
  ports: string[];
}

const currValue = ref(0);
provide('currSerialValue', currValue);

invoke("serial_list_ports").then((list) => {
  invoke("serial_open", {
    msg: {
      port: ((list as ListPortsPayload).ports as string[])[0],
      baud: 115200,
      read_timout_ms: 50,
    },
  }).then(() => {
    listen("serial-read", (event) => {
      const values = event.payload as SerialReadPayload
      currValue.value = Math.max(...values)
    });
  })
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
 