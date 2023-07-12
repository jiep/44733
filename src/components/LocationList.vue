<script setup lang="ts">
import LocationItem from '@/components/LocationItem.vue'
import {Location} from '@/model/Location'

defineProps<{
    items: Array<Location>
}>()

</script>

<template>
    <section class="flex flex-col md:mx-auto h-full overflow-y-auto">
        <div class="mx-4 my-1 last:mb-4" v-for="item in items">  
            <LocationItem :location="item" />
        </div>
    </section>
      
</template>