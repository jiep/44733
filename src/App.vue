<script setup lang="ts">
import { RouterView } from 'vue-router'
import NavBar from './components/NavBar.vue'
</script>

<template>
  <section class="h-full">
    <NavBar/>
    <Suspense>
      <RouterView />
    </Suspense>
  </section>
</template>