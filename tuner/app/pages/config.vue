<script lang="ts" setup>
import type { AccordionItem } from '@nuxt/ui'

const profilesStore = useProfilesStore()

const profileNames = computed(() => profilesStore.profiles.map(p => p[0]))
const currProfileName = ref(profileNames.value[0])
const currProfile = computed(() => profilesStore.profiles.find(p => p[0] === currProfileName.value)![1])

const newProfileName = ref('')
const handleNewProfile = (closePopover: () => void) => {
  profilesStore.profiles.push([newProfileName.value, new ApuTuningConfig()])
  currProfileName.value = newProfileName.value
  closePopover()
  setTimeout(() => {
    newProfileName.value = ''
  }, 500)
}
const handleDeleteProfile = () => {
  profilesStore.profiles = profilesStore.profiles.filter(p => p[0] !== currProfileName.value)
  currProfileName.value = profileNames.value[0]
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

    <div v-if="currProfileName === undefined" class="flex-1 flex flex-col items-center justify-center">
      <h1 class="text-default text-xl text-center mx-auto"> Create a profile to continue </h1>
    </div>
    <!-- config -->
    <div v-else class="flex-1">
      <UAccordion type="multiple" :items="accItems">
        <template #content="{ item }">
          <div v-if="item.label === 'Temperature Tuning'" class="flex flex-col gap-2 p-2">
            <NumberInput v-model="currProfile.temperature_limit" label="Temperature Limit ℃" description="Controls the temperature limit at which
the APU starts soft throttling" />
            <NumberInput v-model="currProfile.skin_temperature_limit" label="Skin Temperature Limit ℃" description="Controls the laptop chassis temperature
limit at which the APU starts throttling" />
          </div>
          <div v-if="item.label === 'Power Tuning'" class="flex flex-col gap-2 p-2">
            <NumberInput v-model="currProfile.stapm_power_limit" label="STAPM Power Limit (W)"
              description="Sustained Thermal and Power Management limit" />
            <NumberInput v-model="currProfile.slow_power_limit" label="Slow Power Limit (W)"
              description="The long-term power limit of the APU" />
            <NumberInput v-model="currProfile.slow_boost_duration" label="Slow Boost Duration (s)"
              description="The duration for which the APU can maintain the slow power limit" />
            <NumberInput v-model="currProfile.fast_power_limit" label="Fast Power Limit (W)"
              description="The short-term power limit of the APU" />
            <NumberInput v-model="currProfile.fast_boost_duration" label="Fast Boost Duration (s)"
              description="The duration for which the APU can maintain the fast power limit" />
          </div>
          <div v-if="item.label === 'VRM Tuning'" class="flex flex-col gap-2 p-2">
            <p>No VRM tuning options available for this device.</p>
          </div>
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
            <UForm @submit="handleNewProfile(close)">
              <UFieldGroup>
                <UInput v-model="newProfileName" placeholder="Profile name" name="profileName" />
                <UButton type="submit" color="neutral" variant="outline">New</UButton>
              </UFieldGroup>
            </UForm>
          </template>
        </UPopover>
      </div>
      <UButton icon="i-lucide-trash-2" color="error" variant="outline" @click="handleDeleteProfile" />
    </div>
  </div>
</template>
