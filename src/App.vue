<script setup lang="ts">
import { RouterView } from 'vue-router'
import NavBar from './components/NavBar.vue'
</script>

<template>
  <NavBar class="sticky top-0 z-50" />
  <section class="flex flex-col min-h-full bg-yellow-50">
    <Suspense>
      <template #default>
        <RouterView />
      </template>
      <template #fallback> Loading... </template>
    </Suspense>
  </section>
</template>
