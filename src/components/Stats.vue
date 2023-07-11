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
    <div class="max-w-full m-4">
        <div class="sm:flex sm:space-x-4">
            <div class="inline-block align-bottom bg-white text-left overflow-hidden border transform transition-all w-full sm:w-1/3 sm:my-8">
                <div class="bg-white p-5">
                    <div class="sm:flex sm:items-start">
                        <div class="text-center sm:mt-0 sm:ml-2 sm:text-left">
                            <h3 class="text-sm leading-6 font-medium text-gray-400">Localizaciones</h3>
                            <p class="text-3xl font-bold text-black">{{ lottery_number }}</p>
                        </div>
                    </div>
                </div>
            </div>
            <div class="inline-block align-bottom bg-white text-left overflow-hidden border transform transition-all w-full sm:w-1/3 sm:my-8">
                <div class="bg-white p-5">
                    <div class="sm:flex sm:items-start">
                        <div class="text-center sm:mt-0 sm:ml-2 sm:text-left">
                            <h3 class="text-sm leading-6 font-medium text-gray-400">Localizaciones</h3>
                            <p class="text-3xl font-bold text-black">{{ locations }}</p>
                        </div>
                    </div>
                </div>
            </div>
            <div class="inline-block align-bottom bg-white text-left overflow-hidden border transform transition-all w-full sm:w-1/3 sm:my-8">
                <div class="bg-white p-5">
                    <div class="sm:flex sm:items-start">
                        <div class="text-center sm:mt-0 sm:ml-2 sm:text-left">
                            <h3 class="text-sm leading-6 font-medium text-gray-400">Provincias</h3>
                            <p class="text-3xl font-bold text-black">{{ provinces }}</p>
                        </div>
                    </div>
                </div>
            </div>
            <div class="inline-block align-bottom bg-white text-left overflow-hidden border transform transition-all w-full sm:w-1/3 sm:my-8">
                <div class="bg-white p-5">
                    <div class="sm:flex sm:items-start">
                        <div class="text-center sm:mt-0 sm:ml-2 sm:text-left">
                            <h3 class="text-sm leading-6 font-medium text-gray-400">Ciudades</h3>
                            <p class="text-3xl font-bold text-black">{{ cities }}</p>
                        </div>
                    </div>
                </div>
            </div>
            <div class="inline-block align-bottom bg-white text-left overflow-hidden border transform transition-all w-full sm:w-1/3 sm:my-8">
                <div class="bg-white p-5">
                    <div class="sm:flex sm:items-start">
                        <div class="text-center sm:mt-0 sm:ml-2 sm:text-left">
                            <h3 class="text-sm leading-6 font-medium text-gray-400">Series totales</h3>
                            <p class="text-3xl font-bold text-black"> {{ series }}</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>