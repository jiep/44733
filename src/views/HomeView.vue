<script setup lang="ts">
import { computed, reactive, ref } from 'vue'

import LocationList from '@/components/LocationList.vue'
import PrimosAdri from '@/components/PrimosAdri.vue'
import StatsPanel from '@/components/StatsPanel.vue'
import { Location } from '@/model/Location'

const number = ref(44733)

const isLoading = ref(false)

const locations = reactive({ items: new Array<Location>() })

const stats = computed(() => {
  return {
    locations: locations.items.length,
    provinces: new Set(locations.items.map((x) => x.province)).size,
    cities: new Set(locations.items.map((x) => x.city)).size,
    series: locations.items.map((x) => x.series.length).reduce((a, b) => a + b, 0)
  }
})

async function onClick(number: any) {
  console.log(number)

  isLoading.value = true

  const response = await fetch(
    `.netlify/functions/number?number=${number.toString().padStart(5, '0')}`
  )
  const data = await response.json()

  let items: Array<Location> = data.locations

  items = items
    .map((x) => new Location(x.name, x.address, x.city, x.province, x.series))
    .sort((a: Location, b: Location) => b.series.length - a.series.length)

  locations.items = items

  isLoading.value = false

  console.log(locations)
}

function onSelectedPrimoAdri(e: number) {
  number.value = e
  onClick(number.value)
}

function setRightValue(e: any) {
  if (e.target.value < 0) number.value = 0
  if (e.target.value > 100000) number.value = 99999
}

onClick(number.value)
</script>

<template>
  <div class="h-full overflow-y-auto">
    <div class="flex m-4 pt-0 items-center">
      <input
        type="number"
        min="0"
        max="99999"
        :onkeyup="setRightValue"
        placeholder="Introduce un número para buscar"
        class="border border-yellow-600 px-3 py-3 placeholder-slate-400 text-slate-600 relative bg-white text-sm outline-none focus:outline-none focus:ring w-full"
        v-model="number"
      />
      <button
        class="border border-pink-500 bg-pink-500 text-white active:bg-pink-600 font-bold uppercase text-sm px-6 py-3 hover:bg-pink-600 outline-none focus:outline-none ease-linear transition-all duration-150"
        type="button"
        @click="onClick(number)"
      >
        Buscar
      </button>
    </div>
    <PrimosAdri @selectedPrimoAdri="onSelectedPrimoAdri" />
    <StatsPanel
      :lottery_number="number"
      :locations="stats.locations"
      :provinces="stats.provinces"
      :cities="stats.cities"
      :series="stats.series"
      :isLoading="isLoading"
    />
    <LocationList class="mt-2" :items="locations.items" :isLoading="isLoading" />
  </div>
</template>
