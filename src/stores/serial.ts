import { defineStore } from 'pinia'

const useSerialStore = defineStore('serial', {
  state: () => {
    return { value: 0 }
  }
})

export default useSerialStore;