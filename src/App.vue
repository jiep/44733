<script setup lang="ts">
import { RouterView } from 'vue-router'
import NavBar from './components/NavBar.vue'
</script>

<template>
  <section class="h-full">
    <NavBar/>
    <Suspense class="h-full bg-yellow-50">
      <template #default>
        <RouterView class="h-full bg-yellow-50" />
      </template>
      <template #fallback>
        Loading...
      </template>
    </Suspense>
  </section>
</template>