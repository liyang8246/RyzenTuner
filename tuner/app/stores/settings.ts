
export const useSettingStore = defineStore('setting', () => {
  const autoSetProfile = ref<SettingsState['autoSetProfile']>(true)
  const autoProfileNames = ref<SettingsState['autoProfileNames']>({
    charge: null,
    discharge: null
  })

  return {
    autoSetProfile,
    autoProfileNames
  }
}, { persist: true })