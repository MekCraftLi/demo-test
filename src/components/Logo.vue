<script setup lang="ts">

/*-------- import ----------------------------------------------------------------------------------------------------*/
import {computed} from "vue";
import {createArcPath} from "../lib/utils.ts"


/*-------- typedef ---------------------------------------------------------------------------------------------------*/
type Point = { x: number; y: number };

const props = defineProps({
  // 点数组：相对于 viewBox 0..100 的百分比坐标（或你可以传 0..100 的值）
  points: {
    type: Array as () => Point[],
    default: () => [
      { x: 30, y: 80 },
      { x: 80, y: 80 }, // 这是钝角顶点，可以调节成钝角
      { x: 90, y: 30 }
    ]
  },
  width: { type: Number, default: 500 },   // 容器像素宽
  height: { type: Number, default: 150 },  // 容器像素高
  color: { type: String, default: '#7ef9ff' },      // 主线颜色（冷霓虹）
  glowColor: { type: String, default: '#0ff0ff' },  // 发光颜色
  strokeWidth: { type: Number, default: 3 },        // 主线宽度
  pulse: { type: Boolean, default: true },          // 是否脉冲动画
  pulseDuration: { type: Number, default: 1600 },   // 脉冲时长 ms
  glowOpacity: { type: Number, default: 0.25 },     // 底层 glow 透明度
  innerOpacity: { type: Number, default: 0.5 },     // 中间层透明度
  useStrongGlow: { type: Boolean, default: true }   // 使用更强的 glow filter（true/false）
});

const pointsStr = computed(() =>
    props.points.map(p => `${p.x},${p.y}`).join(' ')
);

const linePath = computed(() => {
  return createArcPath({x:20, y:60}, {x:80, y:60}, {x:90, y:10}, 10)
});

const mainStyle = computed(() => {
  if (props.pulse) {
    return {
      filter: `drop-shadow(0 0 ${props.strokeWidth * 0.6}px ${props.glowColor})`,
      transition: 'filter 300ms ease'
    } as Record<string, string | number>;
  }
  return {};

});
</script>

<template>

<div class="neon-line-wrapper">
  <div class="logo-container">
  <img class="logo" src="../assets/icons/processed.png" />
  </div>
  <div class="title">
    <span class="title-text">MEK's Embedded Kommand</span>
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
  font-size: 20px;
  color: #F3F2EC;
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