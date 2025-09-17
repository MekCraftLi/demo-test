<script setup lang="ts">
import BubblesBg  from "./components/bckgnd/BubblesBg.vue";
import FluidCursor from "./components/cursor/FluidCursor.vue"
import { cn } from "@/lib/utils";
</script>

<template>
    <!-- Test Mode -->
    <div class="move-zone">

    </div>

    <BubblesBg id="BubblesBg" />
    <FluidCursor id="FluidCursor" />

</template>

<style scoped>

.move-zone {
  position: absolute;
  -webkit-app-region: drag; /* 指定可拖动区域 */
  height: 100px;
  width: 100%;
  background-color: beige;
  opacity: 0.2;
}

#BubblesBg {
  position: absolute;
  right:3%;
  top:6%;
  width: 50%;
  height: 88%;
  opacity: 0.7;
  border-radius: 10%;
}

#FluidCursor {
  position: absolute;
}
</style>