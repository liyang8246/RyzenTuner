<script lang="ts" setup>
import type { AccordionItem } from '@nuxt/ui'

const profilesStore = useProfilesStore()

const profileNames = computed(() => profilesStore.profiles.map(p => p[0]))
const currProfileName = ref(profileNames.value[0])

const newProfileName = ref('')
const onSubmit = (closePopover: () => void) => {
  profilesStore.profiles.push([newProfileName.value, new ApuTuningConfig()])
  currProfileName.value = newProfileName.value
  closePopover()
  setTimeout(() => {
    newProfileName.value = ''
  }, 500)
}

const accItems: AccordionItem[] = [
  {
    label: 'Temperature Tuning',
    icon: 'i-lucide-thermometer'
  },
  {
    label: 'Power Tuning',
    icon: 'i-lucide-trending-up'
  },
  {
    label: 'VRM Tuning',
    icon: 'i-lucide-zap'
  }
]
</script>

<template>
  <div class="flex flex-col">
    <!-- config -->
    <div class="flex-1">
      <UAccordion type="multiple" :items="accItems">
        <template #content="{ item }">
          <p class="pb-3.5 text-sm text-muted">
            This is the {{ item.label }} panel.
          </p>
        </template>
      </UAccordion>
    </div>
    <!-- footer -->
    <USeparator />
    <div class="flex items-center justify-between mt-2">
      <div class="flex gap-2">
        <USelect v-model="currProfileName" :items="profileNames" placeholder="Create one to start ->"
          icon="i-lucide-activity" class="w-64" />
        <UPopover>
          <UButton icon="i-lucide-file-plus" color="neutral" variant="outline" />
          <template #content="{ close }">
            <UForm @submit="onSubmit(close)">
              <UFieldGroup>
                <UInput v-model="newProfileName" placeholder="Profile name" name="profileName" />
                <UButton type="submit" color="neutral" variant="outline">New</UButton>
              </UFieldGroup>
            </UForm>
          </template>
        </UPopover>
      </div>
      <UButton icon="i-lucide-trash-2" color="error" variant="outline" />
    </div>
  </div>
</template>
