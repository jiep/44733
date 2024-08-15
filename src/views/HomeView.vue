<script setup lang="ts">
import { computed, reactive, ref } from 'vue'

import LocationList from '@/components/LocationList.vue'
import StatsPanel from '@/components/StatsPanel.vue'
import { Location } from '@/model/Location'
import { parseContent } from '@/utils/parser'

const DEFAULT_FILENAME = 'Ningún fichero de localizaciones seleccionado'

const file = ref(undefined)
const filename = ref(DEFAULT_FILENAME)

const isLoading = ref(false)
const visible = ref(false)

const locations = reactive({ items: new Array<Location>() })

const stats = computed(() => {
  return {
    locations: locations.items.length,
    provinces: new Set(locations.items.map((x) => x.province)).size,
    cities: new Set(locations.items.map((x) => x.city)).size,
    series: locations.items.map((x) => x.series.length).reduce((a, b) => a + b, 0)
  }
})

const handleFileUpload = async () => {
  isLoading.value = true
  locations.items = []
  const reader = new FileReader()

  reader.addEventListener('load', function () {
    const data = this.result as string
    // @ts-ignore: Object is possibly 'null'.
    let items = parseContent(data).sort(
      (a: Location, b: Location) => b.series.length - a.series.length
    )
    locations.items = items
    isLoading.value = false
    visible.value = true
  })
  try {
    // @ts-ignore: Object is possibly 'undefined'.
    reader.readAsText(file.value.files[0])
    // @ts-ignore: Object is possibly 'undefined'.
    filename.value = file.value.files[0].name
  } catch (error) {
    isLoading.value = false
    visible.value = false
    filename.value = DEFAULT_FILENAME
  }
}
</script>

<template>
  <div class="h-full">
    <form class="m-4 items-center border border-yellow-600">
      <div class="flex flex-row items-center">
        <input type="file" id="file_input" hidden @change="handleFileUpload()" ref="file" accept=".txt" />
        <label
          for="file_input"
          class="border border-pink-500 bg-pink-500 text-white active:bg-pink-600 font-bold uppercase text-sm px-6 py-3 hover:bg-pink-600 outline-none focus:outline-none ease-linear transition-all duration-150 text-center"
        >
          Subir fichero
        </label>
        <label class="md:flex-1 flex-auto text-sm text-slate-500 p-3 bg-white w-full">{{
          filename
        }}</label>
      </div>
    </form>

    <StatsPanel
      :locations="stats.locations"
      :provinces="stats.provinces"
      :cities="stats.cities"
      :series="stats.series"
      :isLoading="isLoading"
      :visible="visible"
    />
    <LocationList class="mt-2" :items="locations.items" :isLoading="isLoading" />
  </div>
</template>
