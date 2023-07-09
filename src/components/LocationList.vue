<script setup lang="ts">
import LocationItem from '@/components/LocationItem.vue'
import {Location} from '@/model/Location'

defineProps<{
    items: Array<Location>
}>()

</script>

<template>
    <section class="h-full overflow-y-auto bg-yellow-50 md:mx-auto">
        <div class="m-4" v-for="item in items">  
            <LocationItem :location="item" />
        </div>
    </section>
      
</template>