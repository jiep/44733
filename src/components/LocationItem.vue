<script setup lang="ts">
import { computed } from 'vue'

import { Location } from '@/model/Location';
import { getColor } from '@/utils/utils';

const props = defineProps<{
    location: Location
}>()

const color = computed(() => {
  return getColor(props.location.series.length)
});

const googleMapsEncoded = computed(() => {
  return encodeURI(`https://www.google.es/maps/place/${props.location.address}, ${props.location.city}, ${props.location.province}`)
});

</script>

<template>
    <div class="px-4 py-2 border border-l-4 hover:bg-yellow-100" :class="color">
        <a :href="googleMapsEncoded" target="_blank" class="flex justify-between">
            <div class="flex flex-col">
                <p class="mb-2 text-lg font-semibold text-gray-900">
                    <template v-if="location.name">
                        {{ location.name }}
                    </template>
                    <template v-else>
                        Sin nombre :(
                    </template>
                </p>
                <p class="truncate text-base text-gray-700">
                    {{location.province}} | {{location.city}}
                </p>
                
                <p class="mb-1 truncate text-xs text-gray-600 break-all">
                    {{location.address}}
                </p>
                <span class="truncate text-sm text-gray-600">
                    {{location.series.length}} series
                </span>
            </div>
        </a>
    </div>  
</template>