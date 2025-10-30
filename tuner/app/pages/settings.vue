<script lang="ts" setup>
const profilesStore = useProfilesStore()
const settingStore = useSettingStore()

const profileNames = computed(() => profilesStore.profiles.map(p => p[0]))

const chargeProfile = computed({
  get: () => settingStore.autoProfileNames.charge ?? undefined,
  set: (value) => settingStore.autoProfileNames.charge = value ?? null
})

const dischargeProfile = computed({
  get: () => settingStore.autoProfileNames.discharge ?? undefined,
  set: (value) => settingStore.autoProfileNames.discharge = value ?? null
})
</script>

<template>
  <div class="flex flex-col gap-2">
    <div class="flex gap-2 items-center">
      <h2 class="w-40"> Auto set profile: </h2>
      <USwitch v-model="settingStore.autoSetProfile" />
    </div>
    <div class="flex gap-2 items-center">
      <h2 class="w-40"> Profile on charging: </h2>
      <USelect v-model="chargeProfile" :items="profileNames" class="w-64" />
    </div>
    <div class="flex gap-2 items-center">
      <h2 class="w-40"> Profile on battery: </h2>
      <USelect v-model="dischargeProfile" :items="profileNames" class="w-64" />
    </div>
  </div>
</template>