<script lang="ts" setup>
const { label, description = '', defaultNum = 0 } = defineProps<{
  label: string,
  description?: string
  defaultNum?: number
}>()
const num = defineModel<number | null>()

const enabled = computed({
  get: () => num.value !== null,
  set: (value: boolean) => {
    num.value = value ? defaultNum : null
  }
})
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