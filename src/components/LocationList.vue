<script setup lang="ts">
import LocationItem from '@/components/LocationItem.vue'
import Loading from '@/components/Loading.vue'

import {Location} from '@/model/Location'

defineProps<{
    items: Array<Location>,
    isLoading: boolean
}>()

</script>

<template>
    <section class="flex flex-col h-full overflow-y-auto">
        <template v-if="items.length == 0 && !isLoading">
            <div class="flex flex-col items-center justify-center h-full">
                <p class="flex align-middle mx-4 h-full text-gray-400 font-medium">No se encontró ninguna localización para el número solicitado</p>
            </div>
        </template>
        <template v-if="isLoading">
            <div class="flex flex-col items-center justify-center h-full">
                <Loading class="mb-2" />
                <p class="flex align-middle mx-4 h-full text-gray-400 font-medium">Cargando localizaciones...</p>
            </div>
        </template>
        <div class="my-1  mx-4 last:mb-4" v-for="item in items" v-if="!isLoading">  
            <LocationItem :location="item" />
        </div>
    </section>
      
</template>