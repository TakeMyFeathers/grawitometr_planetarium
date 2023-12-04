import { defineStore } from 'pinia'

const useDurationStore = defineStore('duration', {
  state: () => {
    return { value: 0 }
  }
})

export default useDurationStore;