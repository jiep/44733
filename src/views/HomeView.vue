<script setup lang="ts">
  import { computed, reactive, ref } from 'vue'

  import LocationList from '@/components/LocationList.vue'
  import PrimosAdri from '@/components/PrimosAdri.vue'
  import Stats from '@/components/Stats.vue'
  import { Location }from '@/model/Location'

  const number = ref(44733)

  const locations = reactive({items: new Array<Location>});

  const stats = computed(() => {
    return {
      locations: locations.items.length,
      provinces: new Set(locations.items.map(x => x.province)).size,
      cities: new Set(locations.items.map(x => x.city)).size,
      series: locations.items.map(x => x.series.length).reduce((a, b) => a + b, 0)
    }
  });

  async function onClick(number: any) {
    console.log(number);
    
    const response = await fetch(`.netlify/functions/number?number=${number.toString().padStart(5, '0')}`)
    const data = await response.json()

    let items: Array<Location> = data.locations;

    items = items.map(x => new Location(x.name, x.address, x.city, x.province, x.series))
             .sort((a: Location, b:Location) => b.series.length - a.series.length)

    locations.items = items;

    console.log(locations)
  }

  function onSelectedPrimoAdri(e: number) {
    number.value = e
    onClick(number.value)
  }

  onClick(number.value)

</script>

<template>
    <div class="flex m-4 pt-0 items-center">
      <input type="number" min="0" max="100000"
        placeholder="Introduce un número para buscar" 
        class="px-3 py-3 placeholder-slate-400 text-slate-600 relative bg-white bg-white text-sm border-0 shadow outline-none focus:outline-none focus:ring w-full" 
        v-model="number" 
      />
      <button class="bg-pink-500 text-white active:bg-pink-600 font-bold uppercase text-sm px-6 py-3 shadow hover:shadow-lg outline-none focus:outline-none ease-linear transition-all duration-150" type="button" @click="onClick(number)"
      >Buscar</button>
    </div>
    <PrimosAdri @selectedPrimoAdri="onSelectedPrimoAdri" />
    <Stats :lottery_number="number" :locations="stats.locations" :provinces="stats.provinces" :cities="stats.cities" :series="stats.series" />
    <LocationList :items="locations.items" />
</template>
