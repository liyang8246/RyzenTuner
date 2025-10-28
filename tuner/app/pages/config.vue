<script lang="ts" setup>
import type { AccordionItem } from '@nuxt/ui'
const items = ref(['Backlog', 'Todo', 'In Progress', 'Done'])
const value = ref('Backlog')

const accItems: AccordionItem[] = [
  {
    label: 'Icons',
    icon: 'i-lucide-smile'
  },
  {
    label: 'Colors',
    icon: 'i-lucide-swatch-book'
  },
  {
    label: 'Components',
    icon: 'i-lucide-box'
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
          <USelect v-model="value" :items="items" />
        </template>
      </UAccordion>
    </div>
    <!-- footer -->
    <USeparator />
    <div class="flex items-center justify-between mt-2">
      <div class="flex gap-2">
        <USelect v-model="value" :items="items" icon="i-lucide-activity" class="w-64" />
        <UButton icon="i-lucide-file-plus" color="neutral" variant="outline" />
      </div>
      <div class="flex gap-2">
        <UButton icon="i-lucide-save" />
        <UButton icon="i-lucide-undo-2" color="neutral" variant="outline" />
        <UButton icon="i-lucide-trash-2" color="error" variant="outline" />
      </div>
    </div>
  </div>
</template>
