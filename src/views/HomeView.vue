<script setup lang="ts">
  import { reactive, ref } from 'vue'

  import LocationList from '@/components/LocationList.vue'
  import { Location }from '@/model/Location'

  const number = ref(44733)

  const locations = reactive(new Array<Location>);
  
  const response = await fetch(`.netlify/functions/number?number=${number.value}`)
  const data = await response.json()

  let items: Array<Location> = data.locations;
  items = items.map(x => new Location(x.name, x.address, x.city, x.province, x.series))
               .sort((a: Location, b:Location) => b.series.length - a.series.length)

  locations.push(...items)

  async function onClick() {
    console.log(number.value);
    
    const response = await fetch(`.netlify/functions/number?number=${number.value.toString().padStart(5, '0')}`)
    const data = await response.json()

    const d: Array<Location> = data.locations;

    items = d.map(x => new Location(x.name, x.address, x.city, x.province, x.series))
             .sort((a: Location, b:Location) => b.series.length - a.series.length)

    locations.length = 0;
    locations.push(...items)
  }

</script>

<template>
    <LocationList :items="items" />
</template>
