<script setup lang="ts">
/* ----- imports ---------------------------------------------------------------------------------------------------- */
import { ref, onMounted} from 'vue'
import FluidCursor from "./components/cursor/FluidCursor.vue";
import gsap from "gsap";

/* ----- components ------------------------------------------------------------------------------------------------- */



/* 1. logo */
const board = ref(null);
const light = ref(null);
const lightHalo = ref(null);




/* 0. test */

const timeLine1 = gsap.timeline({paused: true});

onMounted(() => {
  const boardStyle = getComputedStyle(board.value);
  const relativeWidth = parseInt( boardStyle.getPropertyValue('--max-board-width'));

  console.log(relativeWidth);

  timeLine1.to(board.value, {
    width: relativeWidth,
    duration: 0.3,
    ease: "power4.out" // 强阻尼
  });

  timeLine1.to(light.value, {
    width: relativeWidth * 0.7,
    height: '50%',
    duration: 1,
    ease: "power4.out"
  }, "<")

  timeLine1.to(lightHalo.value, {
    width: relativeWidth * 1.2,
    height: '100%',
    duration: 1,
    ease: "power4.out"
  }, "<")})

function boardUnfold() {


  timeLine1.restart();
}

</script>





<template>
  <div class="windows">
    <div class="background"></div>
  <!-- Test Mode -->
  <div class="move-zone" ref="box">
  Move Zone
  </div>

  <div class="test-butt" @click="boardUnfold()"> testBandL</div>


  <div class="board-and-light">
    <div ref="board" id='board'></div>
    <div id="light-range">
      <div ref="light" id="light"></div>
    </div>
    <div ref="lightHalo" class="light-halo"></div>

  </div>


  <FluidCursor id="FluidCursor"/>
  </div>
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

.background {
  position: absolute;
  height: 100%;
  width: 100%;
  background-color: #eee;
  opacity: 0.9;
  z-index: -2;
}
.test-butt {
  position: absolute;
  width: 100px;
  height: 25px;
  background: white;
  border-radius: 10%;
}


.board-and-light {
  position: absolute;
  width: 10%;
  height: 50%;
  top: 50%;
  left: 50%;
  transform: translate(-50%);

  --max-board-width: 400px;
  --max-light-width: calc(var(--max-board-width) * 1.1);

  #board {
    position: absolute;
    left: 50%;
    width: 0px;
    height: 5px;
    background: #dcb882;
    transform: translate(-50%);
    z-index: 1;

    /* 渐变遮罩，左右逐渐透明 */
    mask-image: linear-gradient(to right, transparent, black 20%, black 80%, transparent);

  }

  #light-range {
    position: absolute;
    left: 50%;
    width: var(--max-light-width);
    height: 100%;
    overflow: hidden;
    transform: translate(-50%);


    #light {
      position: absolute;
      top: 0;
      left: 50%; /* 光束中心 */
      transform: translateX(-50%);
      width: 0px;  /* 光束宽度 */
      height: 0%;
      background: linear-gradient(to bottom, #008cff, #8972df);
      mask-image: radial-gradient(ellipse at 50% 0, rgba(0,0,0,1) 0%, rgba(0,0,0,0) 80%);
      filter: blur(20px); /* 光晕柔化 */
      pointer-events: none;

    }
  }
  .light-halo{
    position: absolute;
    top:20%;
    left: 50%;
    width: 0px;
    transform: translate(-50%, -50%);
    height: 0%;
    opacity: 0.7;
    background: radial-gradient(ellipse at 50% 50%, rgba(118, 169, 228, 0.6) 0%, rgba(62, 67, 149, 0.1) 70%);
    border-radius: 50%;
    filter: blur(20px); /* 光晕柔化 */
    z-index: -1;
  }





}



.neon-line {
  width: 300px;
  height: 4px;
  margin: 100px auto;

  background: linear-gradient(90deg, #ff00cc, #3333ff, #00ffcc);
  background-size: 300% 100%;

  animation: neon-glow 3s linear infinite;

  box-shadow:
      0 10px 5px #ff00cc,
      0 0 20px #3333ff,
      0 0 40px #00ffcc;
}

@keyframes neon-glow {
  0% { background-position: 0% 50%; }
  50% { background-position: 100% 50%; }
  100% { background-position: 0% 50%; }
}

.wall {
  width: 100%;
  height: 300px;
  background: #111; /* 墙面颜色 */
  position: relative;
  overflow: hidden;
}



#FluidCursor {
  position: absolute;
  z-index: -1;
}
</style>