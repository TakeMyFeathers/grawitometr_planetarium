<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

const listPortsMsg = ref("");
const openPortMsg = ref("");
const portName = ref("");
const baud = ref("");
const timeout = ref("");
const readData = ref();

async function listPorts() {
  listPortsMsg.value = await invoke("serial_list_ports");
}

async function openPort() {
  openPortMsg.value = await invoke("serial_open", {
    msg: {
      port: portName.value,
      baud: parseInt(baud.value),
      read_timout_ms: parseInt(timeout.value),
    },
  });
}

listen("serial-read", (event) => {
  const values = event.payload as Uint8Array
  const avg = Math.floor(values.reduce((a, b) => a + b) / values.length)
  readData.value = avg

  console.log(avg)
});
</script>

<template>
  <form class="row" @submit.prevent="listPorts">
    <button type="submit">List Ports</button>
  </form>

  <form class="row" @submit.prevent="openPort">
    <input id="listPorts-input" v-model="portName" placeholder="Port name..." />
    <input id="listPorts-input" v-model="baud" placeholder="115200" />
    <input id="listPorts-input" v-model="timeout" placeholder="50" />
    <button type="submit">Open</button>
  </form>

  <p>{{ listPortsMsg }}</p>
  <p>{{ openPortMsg }}</p>
  <p>{{ readData }}</p>
</template>

<style scoped lang="scss">
input {
  background: black;
  color: white;
}
</style>
