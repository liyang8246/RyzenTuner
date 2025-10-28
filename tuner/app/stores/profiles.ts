export const useProfilesStore = defineStore('profiles', () => {
  const profiles = ref<[string, ApuTuningConfig][]>([])

  return {
    profiles
  }
}, { persist: true })
