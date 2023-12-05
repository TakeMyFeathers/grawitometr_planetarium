import { defineStore } from "pinia";

const useErrorStore = defineStore("error", {
  state: () => {
    return { msg: "" };
  },
});

export default useErrorStore;
