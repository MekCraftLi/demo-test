<script setup lang="ts">


/* 路径栏节点的颜色起始与结束 */
import gsap from "gsap";

let startRGB = [196, 199, 245]
let endRGB = [161, 125, 250]


function interpolateColor(c1: number[], c2: number[], t: number): number[] {
  return [
    c1[0] + (c2[0] - c1[0]) * t,  // R
    c1[1] + (c2[1] - c1[1]) * t,  // G
    c1[2] + (c2[2] - c1[2]) * t   // B
  ];
}


/* 路径节点类 */
import {nextTick, ref} from "vue";

interface PathNode {
  name: string;
  fullPath: string;
  parent?: PathNode;
  children?: PathNode[];
  level: number;
  sRGB: number[];
  eRGB: number[];
  lastSRGB: number[];
  lastERGB: number[];
}

const paths = ref<PathNode[]>([{
  name: "^Flash^",
  fullPath: "Flash:/",
  level: 0,
  sRGB: startRGB,
  eRGB: endRGB,
  lastERGB: endRGB,
  lastSRGB: startRGB,
}])

/* 路径栏节点参数 */
const nodeHeight = ref<number>(30);
const nodeWidth = ref<number>(100);
const nodesPointer = ref<number>(0)


/* 动画 */
let tweenAddUnder = null;
let tweenAddAbove = null;
let tweenRemoveUnder = null;
let tweenRemoveAbove = null;



/* 路径栏添加节点 */

function addPathNode(name: string) {
  if(paths.value.length > 5) {
    return;
  }
  nodesPointer.value += 1;

  if(tweenAddUnder) {
    tweenAddUnder.progress(1);
  }
  if(tweenAddAbove) {
    tweenAddAbove.progress(1);
  }
  if(tweenRemoveUnder) {
    tweenRemoveUnder.progress(1);
  }
  if(tweenRemoveAbove) {
    tweenRemoveAbove.progress(1);
  }

  for (var eachNode of paths.value) {

    eachNode.lastSRGB = eachNode.sRGB;
    eachNode.lastERGB = eachNode.eRGB;
    eachNode.sRGB = interpolateColor(startRGB, endRGB, eachNode.level / (paths.value.length + 1));
    eachNode.eRGB = interpolateColor(startRGB, endRGB, (eachNode.level + 1) / (paths.value.length + 1));
    const el =  document.getElementById(`color-animate-above-${eachNode.level}`) as unknown as SVGAnimateElement | null;
    const el2 =  document.getElementById(`color-animate-above-${eachNode.level}-2`) as unknown as SVGAnimateElement | null;
    el?.beginElement();
    el2?.beginElement();
  }

  if (paths.value.length > 0) {
    let newNode: PathNode = {
      name: name,
      fullPath: paths.value.at(-1)?.fullPath + name + "/",
      sRGB: interpolateColor(startRGB, endRGB, paths.value.length / (paths.value.length + 1)),
      eRGB: endRGB,
      lastERGB: paths.value.at(-1).lastERGB,
      lastSRGB: endRGB,
      level: paths.value.length,
    }

    paths.value.push(newNode);

  }


  nextTick(() => {
    const el =  document.getElementById(`color-animate-above-${paths.value.length - 1}`) as unknown as SVGAnimateElement | null;
    const el2 =  document.getElementById(`color-animate-above-${paths.value.length - 1}-2`) as unknown as SVGAnimateElement | null;
    console.log(el, el2)
    el?.beginElement();
    el2?.beginElement();


    const elUnder = document.querySelector(`#node-viewbox-under-${nodesPointer.value}`);
    const elAbove = document.querySelector(`#node-viewbox-above-${nodesPointer.value}`);

    tweenAddUnder = gsap.fromTo(elUnder,
        {
          height: 0,
          y: nodeHeight.value * 0.35,
        },
        {

          height: nodeHeight.value * 0.7,
          y: 0,
          duration: 0.4,
          ease: "power3",
          onComplete: () => {
            tweenAddUnder = null;
          }
        });

    tweenAddAbove = gsap.fromTo(elAbove,
        {
          height: 0,
          y: -nodeHeight.value * 0.35,
        },
        {

          height: nodeHeight.value * 0.7,
          y: 0,
          duration: 0.4,
          delay: 0.4,
          ease: "power3",
          onComplete: () => {
            tweenAddAbove = null;
          }
        });
  })
}


/* 删除节点 */
function deletePathNode() {
  if(nodesPointer.value <= 0) {
    return;
  }
  const elUnder = document.querySelector(`#node-viewbox-under-${nodesPointer.value}`);
  const elAbove = document.querySelector(`#node-viewbox-above-${nodesPointer.value}`);
  nodesPointer.value -= 1;


  if(tweenAddUnder) {
    tweenAddUnder.progress(1);
  }
  if(tweenAddAbove) {
    tweenAddAbove.progress(1);
  }
  if(tweenRemoveUnder) {
    tweenRemoveUnder.progress(1);
  }

  if (tweenRemoveAbove) {
    tweenRemoveAbove.progress(1);
  }

  if (!elUnder || !elAbove) {
    return;
  }

  tweenRemoveAbove = gsap.fromTo(elAbove,
      {
        height: nodeHeight.value * 0.7,
        y: 0,
        ease: "sine",
      },
      {
        height: 0,
        y: -nodeHeight.value * 0.35,
        duration: 0.4,
        onComplete: () => {
          tweenRemoveAbove = null;
        }
      });


  tweenRemoveUnder = gsap.fromTo(elUnder,
      {
        height: nodeHeight.value * 0.7,
        y: 0,
        ease: "sine",
      },
      {
        height: 0,
        y: nodeHeight.value * 0.35,
        duration: 0.4,
        delay: 0.4,
        onComplete: () => {
          paths.value.pop();
          tweenRemoveUnder = null;

          for (var eachNode of paths.value) {
            eachNode.lastSRGB = eachNode.sRGB;
            eachNode.lastERGB = eachNode.eRGB;
            eachNode.sRGB = interpolateColor(startRGB, endRGB, eachNode.level / (paths.value.length));
            eachNode.eRGB = interpolateColor(startRGB, endRGB, (eachNode.level + 1) / (paths.value.length));
            const el =  document.getElementById(`color-animate-above-${eachNode.level}`) as unknown as SVGAnimateElement | null;
            const el2 =  document.getElementById(`color-animate-above-${eachNode.level}-2`) as unknown as SVGAnimateElement | null;
            el?.beginElement();
            el2?.beginElement();
          }
        }
      });

}


const inputText = ref<string>("");
</script>

<template>
  <div class="root">

    <input v-model="inputText" type="text"
           style="position: absolute; width:300px; height: 50px; top:20%; left: 20%; background: #eeeeee"/>
    <button style="position: absolute; width: 100px; height: 50px; top: 40%; left: 20%; background-color: rgb(90, 90, 90);"
            @click="addPathNode(inputText)"></button>
    <button style="position: absolute; width: 100px; height: 50px; top: 40%; left: 40%; background-color: #888;"
            @click="deletePathNode"></button>

    <svg>
      <defs>
        <filter id="backdropBlur" x="0" y="0" width="100%" height="100%">
          <feFlood flood-color="rgba(255,255,255,0)" result="transparent"/>
          <feComposite in="SourceGraphic" in2="BackgroundImage" operator="over"/>
          <feGaussianBlur stdDeviation="10" result="blurred"/>
          <feBlend in="SourceGraphic" in2="blurred" mode="normal"/>
        </filter>
      </defs>
    </svg>


    <div class="path-bar"  :style="`height: ${nodeHeight}px; --margin-left: ${-nodeWidth / 3 + 5}px;`">

      <div class="path-center"/>

      <!-- 前层节点 -->
      <div class="nodes" id="nodes-above" :style="`height: ${nodeHeight * 0.7}px;`">
        <div class="node-box-above"
             :style="`
           width: ${nodeWidth}px;
           `"
             v-for="node in paths"
             :key="node.level"
             :id="`node-viewbox-above-${node.level}`"
        >
          <div
              class="node-text-wrap"
              :style="`
                width: ${nodeWidth / 2}px;
                height: ${nodeHeight * 0.7}px;
              `"
          >
            <span
                class="node-text"
            >
              {{node.name}}
            </span>
          </div>

          <svg class="ribbon-node-above"
               viewBox="0 0 180 100"
               width="100%"
               :height="`${nodeHeight * 0.7}px`"
               preserveAspectRatio="none"
          >
            <defs>
              <linearGradient :id="`gradient${node.level}`" x1="0%" y1="0%" x2="100%" y2="0%">
                <stop offset="0%" :stop-color="`rgba( ${node.lastSRGB[0]}, ${node.lastSRGB[1]}, ${node.lastSRGB[2]}, 0.9)`">
                  <animate
                      :id="`color-animate-above-${node.level}`"
                      attributeName="stop-color"
                      :values="`
                      rgba( ${node.lastSRGB[0]}, ${node.lastSRGB[1]}, ${node.lastSRGB[2]}, 0.9);
                      rgba(${node.sRGB[0]}, ${node.sRGB[1]}, ${node.sRGB[2]}, 0.9)
                      `"
                      dur="0.7s"
                      begin="indefinite"
                      fill="freeze"
                  />
                </stop>
                <stop offset="100%" :stop-color="`rgba(${node.lastERGB[0]}, ${node.lastERGB[1]}, ${node.lastERGB[2]}, 0.9)`">
                  <animate
                      :id="`color-animate-above-${node.level}-2`"
                      attributeName="stop-color"
                      :values="`
                      rgba(${node.lastERGB[0]}, ${node.lastERGB[1]}, ${node.lastERGB[2]}, 0.9);
                      rgba(${node.eRGB[0]}, ${node.eRGB[1]}, ${node.eRGB[2]}, 0.9);
                      `"
                      dur="0.7s"
                      begin="indefinite"
                      fill="freeze"
                  />
                </stop>
              </linearGradient>
            </defs>

            <path
                d="M 0,0 C 10,0 20,23.8 30,50 C 40,76.18 50,100 60,100 L 180,100 C 170,100 160,76.18 150,50 C 140,23.8 130,0, 120,0 Z "
                :fill="`url(#gradient${node.level})`"
                stroke="transparent"
            />
          </svg>
        </div>
      </div>

<!--后层节点-->
      <div class="nodes" id="nodes-under" :style="`height: ${nodeHeight * 0.7}px; left:${nodeWidth/3 + 5}px;}`">
        <div class="node-box-under"
             :style="`
           width: ${nodeWidth}px;
           `"
             v-for="node in paths.slice(0, -1)"
             :key="node.level + 1"
             :id="`node-viewbox-under-${node.level + 1}`"
        >
          <svg class="ribbon-node-under"
               viewBox="0 0 180 100"
               width="100%"
               :height="`${nodeHeight * 0.7}px`"
               preserveAspectRatio="none"
          >
            <defs>
              <linearGradient :id="`gradient${node.level}-back`" x1="0%" y1="0%" x2="100%" y2="0%">
                <stop offset="0%" :stop-color="`rgba(${node.lastSRGB[0]}, ${node.lastSRGB[1]}, ${node.lastSRGB[2]}, 0.9)`">
                  <animate
                      attributeName="stop-color"
                      :values="`
                      rgba(${node.lastSRGB[0]}, ${node.lastSRGB[1]}, ${node.lastSRGB[2]}, 0.9);
                      rgba(${node.sRGB[0]}, ${node.sRGB[1]}, ${node.sRGB[2]}, 0.9)
                      `"
                      dur="0.7s"
                      begin="indefinite"
                      fill="freeze"
                  />
                </stop>
                <stop offset="100%" :stop-color="`rgba(${node.lastERGB[0]}, ${node.lastERGB[1]}, ${node.lastERGB[2]}, 0.9)`">
                  <animate
                      attributeName="stop-color"
                      :values="`
                      rgba(${node.lastERGB[0]}, ${node.lastERGB[1]}, ${node.lastERGB[2]}, 0.9);
                      rgba(${node.eRGB[0]}, ${node.eRGB[1]}, ${node.eRGB[2]}, 0.9)
                      `"
                      dur="0.7s"
                      begin="indefinite"
                      fill="freeze"
                  />
                </stop>
              </linearGradient>
            </defs>

            <path
                d="M 0,100 C 10,100 20,76.18 30,50 C 40,23.8 50,0 60,0 L 180,0 C 170,0 160,23.8 150,50 C 140,76.18 130,100, 120,100 Z "
                :fill="`url(#gradient${node.level}-back)`"
                stroke="transparent"
            />
          </svg>


        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>

.root {
  position: absolute;
}

.path-bar {
  position: relative;
  border-radius: 15px;
  background-color: rgba(227, 224, 202, 0.7);
  box-shadow: 2px 2px 8px rgba(208, 204, 175, 0.4);
  --margin-left: 0px;

  .path-center {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    height: 50%;
    width: 98%;
    border-radius: 9999px;
    background-color: #03346E;
    box-shadow: 2px 2px 4px #7695FF;
    opacity: 0.5;
    z-index: 3;
  }

  .nodes {
    position: absolute;
    transform: translate(0, -50%);

    top: 50%;
    width: 100%;
    display: flex;
    align-items: center;

    .node-box-above {
      position: relative;
      height: 100%;
      overflow: hidden;

      .node-text-wrap {
        position: absolute;
        transform: translate(-50%, -50%);
        top: 50%;
        left: 50%;
        overflow: hidden;

        .node-text {
          position: absolute;
          transform: translate(-50%, -50%);
          top: 50%;
          left: 50%;
          overflow: hidden;
        }
      }


    }

    .node-box-under {
      height: 100%;
      overflow: hidden;
      z-index: 2;

      .ribbon-node-under {
        position: absolute;
        bottom: 0;
      }
    }

    .node-box-above:not(:first-child) {
      margin-left: var(--margin-left);
    }

    .node-box-under:not(:first-child) {
      margin-left: var(--margin-left);
    }
  }

  #nodes-above {
    z-index: 4;
  }
}
</style>