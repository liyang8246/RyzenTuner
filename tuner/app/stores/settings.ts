export const useSettingStore = defineStore('setting', () => {
  const test = ref(0)

  return {
    test
  }
}, { persist: true })