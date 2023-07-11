<script setup lang="ts">
    const _ = defineProps<{
        lottery_number: number
        locations: number,
        provinces: number,
        cities: number,
        series: number
    }>()
</script>

<template>
    <div class="max-w-full mx-4">
        <div class="sm:flex sm:space-x-4">
            <div class="inline-block align-bottom bg-white text-left overflow-hidden border border-yellow-500 border-t-4 border-t-yellow-500 transform transition-all w-full sm:w-1/3 sm:my-8">
                <div class="bg-white p-2">
                    <div class="sm:flex sm:items-start">
                        <div class="text-center sm:mt-0 sm:ml-2 sm:text-left">
                            <h3 class="text-sm leading-6 font-medium text-gray-400">Número</h3>
                            <p class="text-3xl font-bold text-yellow-700">{{ lottery_number }}</p>
                        </div>
                    </div>
                </div>
            </div>
            <div class="inline-block align-bottom bg-white text-left overflow-hidden border border border-yellow-500 border-t-4 border-t-yellow-500 transform transition-all w-full sm:w-1/3 sm:my-8">
                <div class="bg-white p-2">
                    <div class="sm:flex sm:items-start">
                        <div class="text-center sm:mt-0 sm:ml-2 sm:text-left">
                            <h3 class="text-sm leading-6 font-medium text-gray-400">Localizaciones</h3>
                            <p class="text-3xl font-bold text-yellow-700">{{ locations }}</p>
                        </div>
                    </div>
                </div>
            </div>
            <div class="inline-block align-bottom bg-white text-left overflow-hidden border border border-yellow-500 border-t-4 border-t-yellow-500 transform transition-all w-full sm:w-1/3 sm:my-8">
                <div class="bg-white p-2">
                    <div class="sm:flex sm:items-start">
                        <div class="text-center sm:mt-0 sm:ml-2 sm:text-left">
                            <h3 class="text-sm leading-6 font-medium text-gray-400">Provincias</h3>
                            <p class="text-3xl font-bold text-yellow-700">{{ provinces }}</p>
                        </div>
                    </div>
                </div>
            </div>
            <div class="inline-block align-bottom bg-white text-left overflow-hidden border border border-yellow-500 border-t-4 border-t-yellow-500 transform transition-all w-full sm:w-1/3 sm:my-8">
                <div class="bg-white p-2">
                    <div class="sm:flex sm:items-start">
                        <div class="text-center sm:mt-0 sm:ml-2 sm:text-left">
                            <h3 class="text-sm leading-6 font-medium text-gray-400">Ciudades</h3>
                            <p class="text-3xl font-bold text-yellow-700">{{ cities }}</p>
                        </div>
                    </div>
                </div>
            </div>
            <div class="inline-block align-bottom bg-white text-left overflow-hidden border border border-yellow-500 border-t-4 border-t-yellow-500 transform transition-all w-full sm:w-1/3 sm:my-8">
                <div class="bg-white p-2">
                    <div class="sm:flex sm:items-start">
                        <div class="text-center sm:mt-0 sm:ml-2 sm:text-left">
                            <h3 class="text-sm leading-6 font-medium text-gray-400">Series totales</h3>
                            <p class="text-3xl font-bold text-yellow-700"> {{ series }}</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>