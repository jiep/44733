<script setup lang="ts">
import { computed } from 'vue'

import PhoneIcon from '@/components/PhoneIcon.vue'

import { Location } from '@/model/Location'
import { getColor } from '@/utils/utils'

const props = defineProps<{
  location: Location
}>()

const styles = computed(() => {
  return getColor(props.location.series.length)
})

const googleMapsEncoded = computed(() => {
  return encodeURI(
    `https://www.google.es/maps/place/${props.location.address},
    ${props.location.city},
    ${props.location.province}`
  )
})

const phoneHref = computed(() => {
  return `tel:${props.location.phone}`
})
</script>

<template>
  <div
    class="px-4 py-2 border border-l-4 bg-white hover:bg-yellow-100 w-full flex flex-row items-center"
    :class="styles.border"
  >
    <a :href="googleMapsEncoded" target="_blank" class="flex w-full">
      <div class="flex flex-col w-full">
        <div class="flex items-center justify-between w-auto">
          <p class="flex truncate text-lg font-semibold text-yellow-700 mr-2">
            <template v-if="location.name">
              {{ location.name }}
            </template>
            <template v-else> Sin nombre :( </template>
          </p>
          <div class="flex text-sm font-medium text-gray-500">
            <p class="mr-1 font-bold" :class="styles.color">{{ location.series.length }}</p>
            <p>series</p>
          </div>
        </div>
        <div class="flex flex-col sm:flex-row sm:items-center justify-between w-auto">
          <div class="flex truncate text-base font-medium text-gray-600 sm:mr-4">
            <div class="text-pink-500">
              {{ location.city }}
            </div>
            <p class="mr-1">,</p>
            <div class="text-yellow-500">
              {{ location.province }}
            </div>
          </div>
          <div class="flex truncate text-xs text-gray-600 break-all">
            {{ location.address }}
          </div>
        </div>
      </div>
    </a>
    <a class="flex flex-auto ml-4" :href="phoneHref">
      <PhoneIcon
        class="fill-pink-500 active:fill-pink-600 hover:fill-pink-600 outline-hidden focus:outline-hidden"
      />
    </a>
  </div>
</template>
