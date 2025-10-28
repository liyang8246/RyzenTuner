<script lang="ts" setup>
import type { NavigationMenuItem } from '@nuxt/ui'

const items = ref<NavigationMenuItem[][]>([
  [
    {
      label: 'Overview',
      icon: 'i-lucide-area-chart',
      to: '/'
    },
    {
      label: 'Configration',
      icon: 'i-lucide-settings-2',
      to: '/config'
    },
    {
      label: 'Settings',
      icon: 'i-lucide-settings',
      to: '/settings'
    }
  ],
  [
    {
      label: 'GitHub',
      icon: 'i-simple-icons-github',
      to: 'https://github.com/nuxt/ui',
      target: '_blank'
    }
  ]
])
</script>

<template>
  <UNavigationMenu color="neutral" variant="pill" :items="items" class="w-full px-2" />
</template>