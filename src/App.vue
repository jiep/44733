<script setup lang="ts">
import { RouterView } from 'vue-router'
import NavBar from './components/NavBar.vue'
</script>

<template>
  <section class="h-full">
    <NavBar/>
    <Suspense class="h-full">
      <RouterView class="h-full" />
    </Suspense>
  </section>
</template>