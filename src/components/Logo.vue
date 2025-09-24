<script setup lang="ts">

/*-------- import ----------------------------------------------------------------------------------------------------*/
import {computed} from "vue";
import {createArcPath} from "../lib/utils.ts"


/*-------- typedef ---------------------------------------------------------------------------------------------------*/
const linePath = computed(() => {
  return createArcPath({x:20, y:60}, {x:80, y:60}, {x:90, y:10}, 10)
});



</script>

<template>

<div class="neon-line-wrapper">
  <div class="logo-container">
  <img class="logo" src="../assets/icons/processed.png" />
  </div>
  <div class="title">
    <span class="title-text" >MEK's Embedded Kommand</span>
  </div>
  <svg viewBox="0 0 100 100" width="100%" height="100%" preserveAspectRatio="none">
    <defs>
      <!-- 发光模糊 filter -->
      <filter id="neon-glow" x="-50%" y="-50%" width="200%" height="200%">
        <feGaussianBlur in="SourceGraphic" stdDeviation="2" result="blur1" />
        <feMerge>
          <feMergeNode in="blur1" />
          <feMergeNode in="SourceGraphic"/>
        </feMerge>
      </filter>

      <!-- 更强的 glow 层（多次模糊以增强） -->
      <filter id="glow" x="-60%" y="-60%" width="220%" height="220%">
        <feGaussianBlur in="SourceGraphic" stdDeviation="6" result="blur2" />
        <feGaussianBlur in="blur2" stdDeviation="3" result="blur3" />
        <feMerge>
          <feMergeNode in="blur3"/>
          <feMergeNode in="blur2"/>
          <feMergeNode in="SourceGraphic"/>
        </feMerge>
      </filter>
    </defs>


    <path
        :d="`M`+linePath "
        fill="none"
        stroke="blue"
        stroke-width="4"
        filter="url(#neon-glow)"
    />


    <path
        :d="`M 0,0 0,60 `+linePath+ ` 92,0 0,0` "
        fill="#006989"
        filter="url(#neon-glow)"
    />
    />
  </svg>
</div>
</template>

<style scoped>
/*------- 图像与文字 -------------------------------------------------------------------------------------------------*/
.logo-container {
  position: absolute;
  display: flex;
  justify-content: center;
  align-items: center;
  width: 17%;
  aspect-ratio: 1;

  left: 2%;
  border-radius: 0 0 50% 50%;
  background-color: lavender;
  box-shadow: 2px 2px 5px rgba(0, 0, 0, 0.4);

  .logo {
    width: 90%;

  }
}
.title {
  position: absolute;
  top: 12%;
  right: 0%;
  width: 78%;
  height: 50%;
}
@font-face {
  font-family: "Cascadia Code";
  src: url("@/assets/font/CascadiaCode.ttf") format("ttf");
  font-weight: normal;
  font-style: normal;
}

.title-text {
  font-family: "Cascadia Code";
  font-weight: bold;
  color: #F3F2EC;
  font-size: clamp(8px, 1.625vw, 20px);
}

/*------- 边框 -------------------------------------------------------------------------------------------------------*/
.neon-line-wrapper {
  position: absolute;
  width: 35%;
  max-width: 450px;
  min-width: 200px;
  aspect-ratio: 4 / 1;
}
</style>