<script lang="ts" setup>
const profilesStore = useProfilesStore()
const settingStore = useSettingStore()

const profileNames = computed(() => profilesStore.profiles.map(p => p[0]))
</script>

<template>
  <div class="flex flex-col gap-2">
    <div class="flex gap-2 items-center">
      <h2 class="w-40"> Profile on charging: </h2>
      <USelect v-model="settingStore.autoProfileNames.charge" :items="profileNames" class="w-64" />
    </div>
    <div class="flex gap-2 items-center">
      <h2 class="w-40"> Profile on battery: </h2>
      <USelect v-model="settingStore.autoProfileNames.discharge" :items="profileNames" class="w-64" />
    </div>
  </div>
</template>