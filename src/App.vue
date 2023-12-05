<template>
  <router-view v-slot="{ Component }">
    <transition name="scale" mode="out-in">
      <component :is="Component" />
    </transition>
  </router-view>
</template>

<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import useSerialStore from "./stores/serial";
import { readTextFile, BaseDirectory, exists } from "@tauri-apps/api/fs";
import router from "./router";
import { invoke } from "@tauri-apps/api/tauri";

const CONFIG_FILE = "config.json";

exists(CONFIG_FILE, { dir: BaseDirectory.AppConfig }).then((isAvailable) => {
  if (!isAvailable) {
    console.info("Config file not found");
    console.info("Redirecting to the setup. Try to specify config manually.");
    router.replace({ path: "/setup" });
    return;
  }

  readTextFile(CONFIG_FILE, { dir: BaseDirectory.AppConfig }).then(
    (content) => {
      const config = JSON.parse(content);
      const isConfigValid =
        "port" in config &&
        typeof config.port === "string" &&
        "baud" in config &&
        typeof config.baud === "number";

      if (isConfigValid) {
        invoke("serial_open", {
          msg: {
            port: config.port,
            baud: config.baud,
            read_timout_ms: 100,
          },
        }).then((payload) => {
          if (typeof payload === "string") {
            if (payload === "Port not found") {
              console.error(`Specified port (${config.port}) not found`);
              console.info(
                "Redirecting to the setup. Try to specify config manually.",
              );
              router.replace({ path: "/setup" });
              return;
            }
          }
        });
      } else {
        console.error("Config file is invalid");
        console.info(
          "Redirecting to the setup. Try to specify config manually.",
        );
        router.replace({ path: "/setup" });
      }
    },
  );
});

listen("serial-read", (event) => {
  const serial = useSerialStore();

  if (
    !Array.isArray(event.payload) ||
    !event.payload.some((v) => typeof v === "number")
  ) {
    return;
  }
  const reading = Math.max(...(event.payload as number[]));

  const isReadingInvalid =
    reading == Number.POSITIVE_INFINITY || reading == Number.NEGATIVE_INFINITY;

  if (isReadingInvalid) return;
  serial.value = reading;
});
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
