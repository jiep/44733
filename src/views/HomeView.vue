<script setup lang="ts">
  import LocationList from '@/components/LocationList.vue'
  import { Location }from '@/model/Location'

  const response = await fetch("/data/44733.json");
  const data = await response.json()
  
  let items: Array<Location> = data.locations;
  items = items.map(x => new Location(x.name, x.address, x.city, x.province, x.series)).sort((a: Location, b:Location) => b.series.length - a.series.length)

</script>

<template>
    <LocationList :items="items" />
</template>
