<script setup lang="ts">
import TitleBar from '../components/TitleBar.vue'
import Logo from '../components/Logo.vue'
import ParticlesBg from '../components/bckgnd/ParticlesBg.vue'
</script>

<template>
<Logo style="position: absolute; z-index: 2"/>
  <TitleBar style="position: absolute; z-index: 1"/>

  <div
      class="absolute flex h-full w-full flex-col items-center justify-center overflow-hidden rounded-lg border bg-background md:shadow-xl z-0"
  >


    <ParticlesBg
        class="absolute inset-0"
        :quantity="100"
        :ease="100"
        :color="'#000'"
        :staticity="10"
        refresh
    />
  </div>

</template>

<style scoped>

</style>