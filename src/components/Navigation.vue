<script setup lang="ts">

import {computed, ComputedRef, onMounted, ref, watch} from "vue";
import {gsap} from "gsap";
import {MorphSVGPlugin} from "gsap/MorphSVGPlugin"



/* 导航栏参数 */

// 宽
const navBaseWidth = 30;

// 高
const navBaseHeight: ComputedRef<number> = computed(() => {
  return (props.navItems.length * R * 2 + 70);
})

// 索引
const selected = ref(-1);

// 索引图形中心高度
const centerHeight: ComputedRef<number> = computed(() => {
  return 35 + R * (selected.value * 2 + 1);
})

const R = 27; // 凹陷的半径

// 基底形状
const navBasePath = ref<string>("");
const pathRef = ref<SVGPathElement | null>(null);


/*----- 选项内容 -------------------------------------------*/

export interface OptItem {
  index: number;
  url: string;
  name: string;
  width: number;
}


const props = defineProps<{
  navItems: OptItem[],
}>()

const emit = defineEmits<{ (e: 'select', index: number) : void}>();

watch(selected, (selectedIndex: number) => {
  emit("select", selectedIndex);
})




/* 样式 */

const navBoxStyle: ComputedRef<string> = computed(() => {
  return 'height: ' + (props.navItems.length * 50 + 80) + 'px; border-radius: ' + props.navItems.length / 2 + 'px';
});

const navSelectedStyle: ComputedRef<string> = computed(() => {
  return "left: " + "calc(50% + " + (navBaseWidth / 2) + "px); width: " + R * 1.6 + "px;height: " + R * 1.6 + "px;"
})

function getNavBase() {

  interface Point {
    x: number;
    y: number;
  }




  // 下半圆弧
  let arc1start: Point = {x: 0, y: navBaseHeight.value - navBaseWidth / 2};
  let arc1end: Point = {x: navBaseWidth, y: navBaseHeight.value - navBaseWidth / 2};

  // 上半圆弧
  let arc2start: Point = {x: navBaseWidth, y: navBaseWidth / 2};
  let arc2end: Point = {x: 0, y: navBaseWidth / 2};

  // 用到的参量
  let r = 10; //圆角的半径


  let l: number = Math.sqrt((r + R) * (r + R) + r * r);
  let theta: number = Math.atan(r / (r + R));
  let x: number = R * Math.sin(theta);
  let y: number = R * Math.cos(theta);

  //中间圆弧的下圆角
  let arc3start: Point = {x: navBaseWidth, y: centerHeight.value + l};
  let arc3end: Point = {x: navBaseWidth - x, y: centerHeight.value + y};

  //凹陷
  let arc4end: Point = {x: navBaseWidth - x, y: centerHeight.value - y};

  //上圆角
  let arc5end: Point = {x: navBaseWidth, y: centerHeight.value - l};

  let arc1: string = `L ${arc1start.x},${arc1start.y} A ${navBaseWidth / 2} ${navBaseWidth / 2} 0 0 0 ${arc1end.x},${arc1end.y} `;
  let arc2: string = `L ${arc2start.x},${arc2start.y} A ${navBaseWidth / 2} ${navBaseWidth / 2} 0 0 0 ${arc2end.x},${arc2end.y} `;
  let arc3: string = `L ${arc3start.x},${arc3start.y} A ${r} ${r} 0 0 0 ${arc3end.x},${arc3end.y} `;
  let arc4: string = `A ${R} ${R} 0 0 1 ${arc4end.x},${arc4end.y} `;
  let arc5: string = `A ${r} ${r} 0 0 0 ${arc5end.x},${arc5end.y} `;

  if (selected.value < 0) {
    let resetPath = `M 0,${navBaseWidth / 2} ${arc1} ${arc2} Z`;
    return resetPath;
  }

  return `M 0,${navBaseWidth / 2} ` + arc1 + arc3 + arc4 + arc5 + arc2 + "Z";

}



/* 钩子函数 */
gsap.registerPlugin(MorphSVGPlugin)


onMounted(() => {
  navBasePath.value = getNavBase();
})


function changeSelected(index: number) {

  navBasePath.value = getNavBase();

  selected.value = index;

  let pathTemp: string = getNavBase();

  gsap.to(pathRef.value, {
    duration: 0.2,
    morphSVG: pathTemp,
    ease: "power1.Out",
  });

}


</script>

<template>
  <div class="root" :style="`width: ${navBaseWidth + 50}px;height: ${navBaseHeight + 50}px`">

    <div class="nav-base-box" :style="navBoxStyle" >
      <svg class="nav-base-img" :viewBox="`-5 -5 ` + (navBaseWidth + 10) + ' ' + (navBaseHeight + 10)"
           :width="(navBaseWidth + 15)  + `px`" :height="(navBaseHeight + 15) + `px`"
           preserveAspectRatio="none">
        <path
            ref="pathRef"
            :d="navBasePath"
            stroke-width="1"
        />
      </svg>
    </div>


    <div
        class="nav-option"
        v-for="opt in props.navItems"
        key="index"
        :style="`
        top: calc(50% - ${navBaseHeight / 2 - (35 + R * (opt.index * 2 + 1))}px);
        left: 4%;
        `"
    >

      <!-- 圆形 -->
      <div
          class="nav-box"
          :class="{'nav-selected-box': (opt.index == selected)}"
          @click="changeSelected(opt.index)"
          :style="(opt.index != selected) ? `
            height: ` + (navBaseWidth + 8) + `px;
            width: ${navBaseWidth + 8}px;
           ` :
            navSelectedStyle
          "
      ></div>

      <!-- 文字 -->
      <div class="info-visual-window">

        <div class="nav-text-box"
             :style="`
        left: ${-props.navItems[opt.index].width}px;
        width: ${props.navItems[opt.index].width}px;
       `"
        >
        <span
            class="nav-selected-info-text"
            :style="`left:${R - 15}px`"
        >
          {{ props.navItems[opt.index].name }}

        </span>

        </div>

      </div>
    </div>

  </div>
</template>

<style scoped>

.nav-base-box {
  position: absolute;
  width: 100px;
  height: 50px;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);

  .nav-base-img {
    position: absolute;
    transform: translate(-50%, -50%);
    top: 50%;
    left: 50%;

    fill: rgba(24, 49, 151, 0.2); /* 半透明 */
    backdrop-filter: blur(2px); /* 背景模糊 */
    stroke: rgb(147, 154, 211)
  }
}

.nav-option{
  position: absolute;
  transform: translate(50%, -50%);
  left: 0%;
  width: 50%;
  height: 20px;

  .nav-box {
    position: absolute;
    background-color: #98a6d5;
    transform: translate(-50%, -50%);
    top: 50%;
    left: 50%;
    border-radius: 50%;
    z-index: 3;
    transition: width 0.5s ease,
    height 0.5s ease,
    top 0.5s ease,
    left 0.5s ease,
    background-color 0.5s ease,
    transform 0.2s ease;

  }
  .nav-box:hover + .info-visual-window .nav-text-box{
    transform: translate(115%, -50%);
  }

  .nav-box:hover {
    transform: translate(-50%, -50%) scale(1.1);
  }


  .nav-selected-box:hover + .info-visual-window .nav-text-box{
    transform: translate(140%, -50%);
  }



  .info-visual-window {
    position: absolute;
    transform: translate(0%, -50%);
    top: 50%;
    left: 50%;
    min-width: 150px;
    min-height: 20px;
    overflow: hidden;


    .nav-text-box {
      position: absolute;
      background-color: #f2f6e1;
      transform: translate(0, -50%);
      height: 25px;
      top: 50%;
      border-radius: 10px;

      transition: transform 0.5s ease;

      @font-face {
        font-family: "MapleMonoNFCNThin";
        src: url("../assets/font/MapleMono-NF-CN-Thin.ttf") format("ttf");
        font-weight: normal;
        font-style: normal;
      }

      .nav-selected-info-text {
        position: absolute;
        font-family: MapleMonoNFCNThin;
        font-size: 15px;
      }
    }
  }




  .nav-selected-box {
    background-color: #7695FF;
    border-radius: 50%;
    box-shadow: 2px 2px 5px rgba(50, 50, 50, 0.5);
  }

}




</style>
