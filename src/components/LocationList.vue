<script setup lang="ts">
import LocationItem from '@/components/LocationItem.vue'
import {Location} from '@/model/Location'

defineProps<{
    items: Array<Location>
}>()

</script>

<template>
    <section class="h-full overflow-y-auto md:mx-auto">
        <div class="mx-4 my-2" v-for="item in items">  
            <LocationItem :location="item" />
        </div>
    </section>
      
</template>