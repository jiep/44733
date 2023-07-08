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

</script>

<template>
    <div class="px-4 py-2 border border-l-4" :class="color">
        <div class="flex justify-between">
            <div class="flex">
            <div class="min-w-0 flex-auto">
                <p class="mb-2 text-lg font-semibold text-gray-900">
                    <template v-if="location.name">
                        {{ location.name }}
                    </template>
                    <template v-else>
                        Sin nombre :(
                    </template>
                </p>
                <p class="truncate text-lg text-gray-700">
                    {{location.province}} | {{location.city}}
                </p>
                
                <p class="mb-1 truncate text-sm text-gray-600">
                    {{location.address}}
                </p>
                <span class="truncate text-sm text-gray-400">
                    {{location.series.length}} series
                </span>
            </div>
            </div>
        </div>
    </div>  
</template>