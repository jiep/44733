<script setup lang="ts">
  import { reactive, ref } from 'vue'

  import LocationList from '@/components/LocationList.vue'
  import Tabs from '@/components/Tabs.vue'
  import { Location }from '@/model/Location'

  const number = ref(44733)

  async function onClick() {
    console.log(number.value);
    
    const response = await fetch(`.netlify/functions/number?number=${number.value.toString().padStart(5, '0')}`)
    const data = await response.json()

    let items: Array<Location> = data.locations;
    items = items.map(x => new Location(x.name, x.address, x.city, x.province, x.series))
               .sort((a: Location, b:Location) => b.series.length - a.series.length)
  }

  const response = await fetch(`.netlify/functions/number?number=${number.value}`)

  const data = await response.json()

  let items: Array<Location> = data.locations;
  items = items.map(x => new Location(x.name, x.address, x.city, x.province, x.series))
               .sort((a: Location, b:Location) => b.series.length - a.series.length)

</script>

<template>
    <Tabs />
    <div class="flex m-4 pt-0 items-center">
      <input type="number" min="0" max="100000"
        placeholder="Introduce un número para buscar" 
        class="px-3 py-3 placeholder-slate-400 text-slate-600 relative bg-white bg-white text-sm border-0 shadow outline-none focus:outline-none focus:ring w-full" 
        v-model="number" 
      />
      <button class="bg-pink-500 text-white active:bg-pink-600 font-bold uppercase text-sm px-6 py-3 shadow hover:shadow-lg outline-none focus:outline-none mr-1 mb-1 ease-linear transition-all duration-150" type="button" @click="onClick()"
      >Buscar</button>
    </div>
    <LocationList :items="items" />
</template>
