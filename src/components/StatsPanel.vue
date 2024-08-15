<script setup lang="ts">
import LoadingCircle from '@/components/LoadingCircle.vue'

defineProps<{
  locations: number
  provinces: number
  cities: number
  series: number
  isLoading: boolean
  visible: boolean
}>()
</script>

<template>
  <div v-if="visible" class="max-w-full mx-4">
    <div class="sm:flex sm:space-x-4">
      <div
        class="mb-2 inline-block align-bottom bg-white text-left overflow-hidden border border-yellow-500 border-t-4 border-t-yellow-500 transform transition-all w-full sm:w-1/3 sm:my-8"
      >
        <div class="bg-white p-2">
          <div
            class="flex flex-col items-center sm:items-start place-content-center sm:mt-0 sm:ml-2 sm:text-left"
          >
            <h3 class="text-sm leading-6 font-medium text-gray-400 flex">Localizaciones</h3>
            <p v-if="!isLoading" class="text-center flex text-3xl font-bold text-yellow-700">
              {{ locations }}
            </p>
            <LoadingCircle v-else :isLoading="isLoading" />
          </div>
        </div>
      </div>
      <div
        class="mb-2 inline-block align-bottom bg-white text-left overflow-hidden border border-yellow-500 border-t-4 border-t-yellow-500 transform transition-all w-full sm:w-1/3 sm:my-8"
      >
        <div class="bg-white p-2">
          <div
            class="flex flex-col items-center sm:items-start place-content-center sm:mt-0 sm:ml-2 sm:text-left"
          >
            <h3 class="text-sm leading-6 font-medium text-gray-400 flex">Provincias</h3>
            <p v-if="!isLoading" class="text-center flex text-3xl font-bold text-yellow-700">
              {{ provinces }}
            </p>
            <LoadingCircle v-else :isLoading="isLoading" />
          </div>
        </div>
      </div>
      <div
        class="mb-2 inline-block align-bottom bg-white text-left overflow-hidden border border-yellow-500 border-t-4 border-t-yellow-500 transform transition-all w-full sm:w-1/3 sm:my-8"
      >
        <div class="bg-white p-2">
          <div
            class="flex flex-col items-center sm:items-start place-content-center sm:mt-0 sm:ml-2 sm:text-left"
          >
            <h3 class="text-sm leading-6 font-medium text-gray-400 flex">Ciudades</h3>
            <p v-if="!isLoading" class="text-center flex text-3xl font-bold text-yellow-700">
              {{ cities }}
            </p>
            <LoadingCircle v-else :isLoading="isLoading" />
          </div>
        </div>
      </div>
      <div
        class="mb-2 inline-block align-bottom bg-white text-left overflow-hidden border border-yellow-500 border-t-4 border-t-yellow-500 transform transition-all w-full sm:w-1/3 sm:my-8"
      >
        <div class="bg-white p-2">
          <div
            class="flex flex-col items-center sm:items-start place-content-center sm:mt-0 sm:ml-2 sm:text-left"
          >
            <h3 class="text-sm leading-6 font-medium text-gray-400 flex">Series totales</h3>
            <p v-if="!isLoading" class="text-center flex text-3xl font-bold text-yellow-700">
              {{ series }}
            </p>
            <LoadingCircle v-else :isLoading="isLoading" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
