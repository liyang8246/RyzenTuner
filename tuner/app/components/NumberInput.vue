<script lang="ts" setup>
const { label, description = '' } = defineProps<{
  label: string
  description?: string
}>()
const num = defineModel<number | null>()
const enabled = ref(false)
</script>

<template>
  <div class="flex items-center gap-2 h-16">
    <div class="flex flex-col">
      <h1 class="text-default mb-1 w-72">{{ label }}</h1>
      <p class="text-muted text-xs">{{ description }}</p>
    </div>
    <div class="w-full flex flex-col gap-2">
      <div class="flex items-center gap-3">
        <UCheckbox v-model="enabled" size="lg" />
        <UInputNumber v-model="num" :disabled="!enabled" orientation="vertical" />
      </div>
      <USlider v-model="num!" :disabled="!enabled" />
    </div>
  </div>
</template>