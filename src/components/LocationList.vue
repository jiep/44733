<script setup lang="ts">
import LocationItem from '@/components/LocationItem.vue'
import {Location} from '@/model/Location'

defineProps<{
    items: Array<Location>
}>()

</script>

<template>
    <div class="m-4" v-for="item in items">  
        <LocationItem :location="item" />
    </div>  
</template>