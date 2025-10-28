export const useMyProfilesStore = defineStore('profiles', () => {
  const profiles = ref<Map<string, ApuTuningConfig>>(new Map())

  return {
    profiles
  }
}, { persist: true })
