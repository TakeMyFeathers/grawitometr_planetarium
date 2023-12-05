<template>
  <div>
    <div class="grid grid-cols-2 items-center">
      <form class="row" @submit.prevent="listPorts">
        <button type="submit">List Ports</button>
      </form>
      <p class="text-xl">{{ portsMsg }}</p>
    </div>

    <form class="row" @submit.prevent="openPort">
      <input
        id="listPorts-input"
        v-model="portName"
        placeholder="Port name..."
      />
      <input id="listPorts-input" v-model="baud" placeholder="115200" />
      <button type="submit">Open</button>
    </form>

    <p>{{ readData }}</p>
  </div>
</template>

<script setup lang="ts">
import { computed, reactive, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { ListPortsPayload } from "../types/serial";
import router from "../router";

const availablePorts: ListPortsPayload = reactive({ ports: [] });
const openPortMsg = ref("");
const portName = ref("");
const baud = ref("");
const readData = ref();

let portsMsg = computed(() => {
  return availablePorts.ports.join(",");
});

async function listPorts() {
  const payload = (await invoke("serial_list_ports")) as ListPortsPayload;
  availablePorts.ports = payload.ports;
}

async function openPort() {
  openPortMsg.value = await invoke("serial_open", {
    msg: {
      port: portName.value,
      baud: parseInt(baud.value),
      read_timout_ms: 100,
    },
  });
  router.push({ path: "/" });
}
</script>

<style scoped lang="scss">
button {
  background-color: white;
  color: black;
  padding: 10px 20px;
  border-radius: 10px;
}

input {
  background: white;
  color: black;
  font-size: 20px;
  padding: 10px 20px;
  border-radius: 10px;
  margin: 10px;
}
</style>
