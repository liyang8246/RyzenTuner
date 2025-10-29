export const useSettingStore = defineStore('setting', () => {
  const autoSetProfile = ref(true)
  const autoProfileNames = ref<{ charge: string | undefined, discharge: string | undefined }>({
    charge: undefined,
    discharge: undefined
  })

  return {
    autoSetProfile,
    autoProfileNames
  }
}, { persist: true })