<script setup lang="ts">
/* ----- imports ---------------------------------------------------------------------------------------------------- */
import {ref, onMounted} from 'vue'
import FluidCursor from "./components/cursor/FluidCursor.vue";
import gsap from "gsap";
import {invoke} from "@tauri-apps/api/core";

/* ----- components ------------------------------------------------------------------------------------------------- */


/* 1. logo */
const board = ref(null);
const light = ref(null);
const lightHalo = ref(null);

const lM = ref(null);
const lE = ref(null);
const lK = ref(null);

const lE2 = ref(null);
const lK2 = ref(null);
const timeLine1 = gsap.timeline({paused: true});

/* 背景卡片 */

const cards = Array.from({length: 8}, (_, i) => `Card ${i + 1}`);

const cardRefs = ref([]);

/* 文字 */

/* onMouted hook */

onMounted(() => {
  const boardStyle = getComputedStyle(board.value);
  const relativeWidth = parseInt(boardStyle.getPropertyValue('--max-board-width'));


  timeLine1.to(board.value, {
    width: relativeWidth,
    duration: 0.3,
    ease: "power4.out",

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
  }, "<")

  timeLine1.to(".background-img", {
    opacity: 0.3,
    duration: 3,
  }, "<")

  timeLine1.from(lM.value, {
    y: 50,
    opacity: 0,
    duration: 1,
    ease: "power3.out",
    delay: 0.2
  }, "<");

  timeLine1.from(lE.value, {
    y: 50,
    opacity: 0,
    duration: 1,
    ease: "power3.out",

  }, "<");


  timeLine1.from(lK.value, {
    y: 50,
    opacity: 0,
    duration: 1,
    ease: "power3.out",
  }, "<");

  cardRefs.value.forEach((el, index) => {
    const angle = 80 - index * 10;
    timeLine1.to(el, {
      y: -50,
      rotation: angle,
      duration: 2,
      ease: "elastic.out(1,1)",
      delay: 0.05
    }, "<");
  });


  timeLine1.to(board.value, {
    opacity: 0,
    duration: 0.5,
    ease: "power4.out",
    delay: 1,

  }, "<");

  timeLine1.to(light.value, {
    opacity: 0,
    duration: 0.5,
    ease: "power4.out"
  }, "<")

  timeLine1.to(lightHalo.value, {
    opacity: 0,
    duration: 0.5,
    ease: "power4.out"
  }, "<")


  timeLine1.to(".title", {
    y: 200,
    x: -280,
    duration: 0.6,
    scale: 0.6,
    color: "white",
    ease: "power4.out",

  }, "<")

  timeLine1.to(".letter-container", {
    mask: "none",
  }, "<")

  timeLine1.to(".title-letter", {
    color: "rgba(255,255,255,1)",
    fontWeight: "normal",

  }, "<");

  timeLine1.to("#letter-E2", {
    x: -200,
    y: 194,
    mask: "none",
    fontWeight: "normal",
    opacity: 1,
    scale: 0.6,
    color: "white",
    duration: 0.6,
    ease: "power4.out",
  }, "<")


  timeLine1.to("#letter-K2", {
    x: 47,
    y: 194,
    mask: "none",
    opacity: 1,
    scale: 0.6,
    color: "rgba(62,67,149,0.1)",
    duration: 0.8,
    ease: "power4.out",
  }, "<")

  timeLine1.to(".normal-text", {
    opacity: 1,
    duration: 0.8,
    ease: "power4.out",
    delay: 0.3,
  }, "<")


  timeLine1.to({}, {
    delay: 1.3,
    toStart: () => {
      handleCloseWindow()
    }
  })


  timeLine1.to({}, {
    toStart: () => {
      handleCreateWindow()
    }
  }, "<")


  timeLine1.restart();
})

async function handleCreateWindow() {
  console.log("Creating Window...");

  await invoke("create_window", {
    opts: {
      label: "MEKs-Embedded-Kommand",
      resizable: true,
      maximized: true,

    }
  })
}

async function handleCloseWindow() {
  console.log("closing Window...");
  await invoke("close_window", {
    label: "Loading",
  })
}

</script>


<template>
  <div class="windows">
    <div class="noselect">
      <!--<div class="background"></div>
      <div class="background-img"><img class="yuzusoft" src="./assets/svg-img/welcome/yuzu-logo.svg"></div>
      -->




      <!-- 隔板与灯光 ------------------------------------------------------------------------------------------------ -->
      <div class="board-and-light">
        <div ref="board" id='board'></div>
        <div id="light-range">
          <div ref="light" id="light"></div>
        </div>
        <div ref="lightHalo" class="light-halo"></div>
      </div>

      <!-- 文字 ------------------------------------------------------------------------------------------------------ -->
      <div class="text-container">
        <div class="normal-text" id="text-1">
          mbedded
        </div>
        <div class="normal-text" id="text-2">
          ommand
        </div>
        <div class="title">
          <div ref="lM" id="letter-M" class="letter-container">
            <span class="title-letter" id="text-M">M</span>
          </div>
          <div ref="lE" id="letter-E" class="letter-container">
            <span class="title-letter" id="text-E">E</span>
          </div>

          <div ref="lK" id="letter-K" class="letter-container">
            <span class="title-letter" id="text-K">K</span>
          </div>
        </div>

        <div ref="lE2" id="letter-E2" class="letter-container">
          <span class="title-letter" id="text-E2">E</span>
        </div>


        <div ref="lK2" id="letter-K2" class="letter-container">
          <span class="title-letter" id="text-K2">K</span>
        </div>


      </div>

      <!-- 背景卡片 -------------------------------------------------------------------------------------------------- -->
      <div
          v-for="(_, index) in cards"
          :key="index"
          class="bckgnd-card"
          ref="cardRefs"
      >

      </div>


      <FluidCursor id="FluidCursor"/>
    </div>
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

.windows {
  position: absolute;
  height: 500px;
  width: 750px;
  background-color: transparent;
  opacity: 0.9;
  z-index: -2;
  border-radius: 30px;
  overflow: hidden;
}

.background-img {
  position: absolute;
  z-index: -3;
  width: 180%;
  height: 180%;
  top: -20%;
  left: -10%;
  opacity: 0;
}

.noselect {
  user-select: none; /* 标准语法 */
  -webkit-user-select: none; /* Safari / Chrome */
  -moz-user-select: none; /* Firefox */
  -ms-user-select: none; /* 旧版 IE/Edge */
  user-select: none; /* 禁止文字选中 */
  -webkit-user-drag: none; /* 禁止拖动图片（Chrome / Safari） */
  -moz-user-select: none; /* Firefox */
  -ms-user-select: none; /* IE/Edge */
  pointer-events: none; /* 如果不希望鼠标事件影响图片 */
}

.test-butt {
  position: absolute;
  width: 100px;
  height: 25px;
  background: white;
  border-radius: 10%;
  z-index: 10;
}

/* -------- 隔板与灯光 -----------------------------------------------------------------------------------------------*/
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
      width: 0px; /* 光束宽度 */
      height: 0%;
      background: linear-gradient(to bottom, #008cff, #8972df);
      mask-image: radial-gradient(ellipse at 50% 0, rgba(0, 0, 0, 1) 0%, rgba(0, 0, 0, 0) 80%);
      filter: blur(20px); /* 光晕柔化 */
      pointer-events: none;

    }
  }

  .light-halo {
    position: absolute;
    top: 20%;
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

/* -------- 文字 -----------------------------------------------------------------------------------------------------*/
@font-face {
  font-family: "Cascadia Code";
  src: url("@/assets/font/CascadiaCode.ttf") format("ttf");
  font-weight: normal;
  font-style: normal;
}

body {
  font-family: "Vintage Type", serif;
}


.text-container {
  position: absolute;
  width: 100%;
  height: 100%;

  pointer-events: none;

  .title {
    position: absolute;
    top: 25%;
    height: 120px;
    width: 100%;
    overflow: hidden;


  }

  #letter-M {
    left: 40%;
  }

  #letter-E {
    left: 50%;
  }

  #letter-E2 {
    top: 25%;
    left: 50%;
    opacity: 0;
  }


  #letter-K {
    left: 60%;
  }

  #letter-K2 {
    top: 25%;
    left: 50%;
    opacity: 0;
  }

  .normal-text {
    color: white;
    font-size: 40px;
    opacity: 0;
  }

  #text-1 {
    position: absolute;
    top: 370px;
    left: 222px;
  }

  #text-2 {
    position: absolute;
    top: 370px;
    left: 470px;
  }
}

.letter-container {
  position: absolute;
  transform: translate(-50%);
  mask-image: linear-gradient(to bottom, rgba(0, 0, 0, 1) 0%, rgba(0, 0, 0, 1) 60%, rgba(0, 0, 0, 0) 80%);
  overflow: hidden;

  .title-letter {
    font-family: "Cascadia Code", serif;
    font-size: 100px;
    font-weight: bold;
    color: #222;

  }
}

/* -------- 背景卡片 -------------------------------------------------------------------------------------------------*/
.bckgnd-card {
  position: absolute;
  width: 1000px;
  height: 1000px;
  right: -0%;
  bottom: -1050px;
  border-radius: 60px;
  opacity: 0.8;


  transform-origin: 80% 60%;
  box-shadow: 0 10px 5px #7f3791,
  0 0 20px #524d4d;

  background: linear-gradient(to bottom right, rgba(229, 60, 29, 100) 0%, rgba(0, 63, 221, 100) 20%);
  z-index: -3;
}


#FluidCursor {
  position: absolute;
  z-index: -1;
}
</style>