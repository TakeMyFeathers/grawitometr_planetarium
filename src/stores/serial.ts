import { defineStore } from 'pinia'

export const useSerialStore = defineStore('serial', {
  state: () => {
    return { value: 0 }
  }
})