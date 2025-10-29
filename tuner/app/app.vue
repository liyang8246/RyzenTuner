<script lang="ts" setup>
useSettingStore()
useProfilesStore()
</script>

<template>
  <UApp>
    <div class="flex flex-col h-screen">
      <HeaderBar />
      <NuxtPage class="flex-1 p-4" />
    </div>
  </UApp>
</template>
