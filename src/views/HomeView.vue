<script setup lang="ts">
  import LocationList from '@/components/LocationList.vue'
  import { Location }from '@/model/Location'

  const response = await fetch("/data/44733.json")
  const data = await response.json()

  const response2 = await fetch(".netlify/functions/number")
  const data2 = await response2.json()
  console.log(data2)
  
  
  let items: Array<Location> = data.locations;
  items = items.map(x => new Location(x.name, x.address, x.city, x.province, x.series)).sort((a: Location, b:Location) => b.series.length - a.series.length)

</script>

<template>
    <LocationList :items="items" />
</template>
