<script setup lang="ts">
import { computed, reactive, ref } from 'vue'

import LocationList from '@/components/LocationList.vue'
import PrimosAdri from '@/components/PrimosAdri.vue'
import StatsPanel from '@/components/StatsPanel.vue'
import { Location } from '@/model/Location'
import { parseContent } from '@/utils/parser'

const number = ref(44733)

const file = ref(undefined)

const isLoading = ref(false)

let data;

const locations = reactive({ items: new Array<Location>() })

const stats = computed(() => {
  return {
    locations: locations.items.length,
    provinces: new Set(locations.items.map((x) => x.province)).size,
    cities: new Set(locations.items.map((x) => x.city)).size,
    series: locations.items.map((x) => x.series.length).reduce((a, b) => a + b, 0)
  }
})

const handleFileUpload = async() => {
            const reader = new FileReader();

            reader.addEventListener('load', function() {
              data = this.result;
              console.log(data);
              let items = parseContent(data).sort(
    (a: Location, b: Location) => b.series.length - a.series.length
  )

  locations.items = items
            });
            reader.readAsText(file.value.files[0]);
        }

async function onClick(number: any) {
  isLoading.value = true

  window.open("https://www.loteriasyapuestas.es/es/buscar-decimo");

  const response = await fetch('/44733')

  const data = await response.text()

  isLoading.value = false
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
  <div class="h-full">
    
    <label for="file_input">Upload file</label>
    <input id="file_input" type="file" @change="handleFileUpload()" ref="file">

    <div class="flex m-4 pt-0 items-center">
      <input
        id="number"
        type="number"
        min="0"
        max="99999"
        :onkeyup="setRightValue"
        placeholder="Introduce un número para buscar y descargar las localizaciones desde la web de Loterías"
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
    <!-- <PrimosAdri @selectedPrimoAdri="onSelectedPrimoAdri" /> -->
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
