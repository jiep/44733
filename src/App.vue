<script setup lang="ts">
import { RouterView } from 'vue-router'
import NavBar from './components/NavBar.vue'
</script>

<template>
  <section class="h-full bg-yellow-50">
    <NavBar/>
    <Suspense>
      <template #default>
        <RouterView />
      </template>
      <template #fallback>
        Loading...
      </template>
    </Suspense>
  </section>
</template>