<script setup lang="ts">

/*----- 1. import ----------------------------------------------------------------------------------------------------*/

import {ref, watch, nextTick, onMounted} from "vue";
import {invoke, Channel} from "@tauri-apps/api/core"
import {DynamicScroller, DynamicScrollerItem} from "vue-virtual-scroller"

/*----- 2. channel ---------------------------------------------------------------------------------------------------*/

/* 1. typedef */

type SerialData = {
  millionSec: number;
  index: number;
  data: number[];
}


/* 4. call the function when on mounted */


/*---- 3. virtual list -----------------------------------------------------------------------------------------------*/

/* 1. define data list */
const messages = ref<SerialData[]>([]);

async function testSerialPort() {

  /* 2. create channel */
  const onEvent = new Channel<SerialData>();


  /* 3. register callback function */
  onEvent.onmessage = (message: SerialData) => {
    messages.value.push(message);
    console.log(messages.value.length);
  }

  console.log("开始调用串口")
  console.log("使用通道ID:", onEvent.id);
  await invoke("start_receive_comm", {
    onEvent: onEvent,
    label: "MEKs-Embedded-Kommand"
  });
  console.log("串口调用完成")
}

async function stopSerialPort() {
  console.log(messages.value);
  await invoke("stop_receive_comm");
}


const msgScroller = ref<InstanceType<typeof DynamicScroller> | null>(null);
// 在 setup 中使用 <DynamicScroller ref="msgScroller">

let autoScroll = true; // 是否自动滚动

// 用户滚动事件
function handleScroll(event: Event) {
  const target = event.target as HTMLElement;
  const threshold = 10; // 允许误差
  autoScroll =
      target.scrollHeight - target.scrollTop - target.clientHeight < threshold;
}


// 新消息加入
watch(messages, async () => {
  if (autoScroll) {
    await nextTick();
    let index = messages.value.length - 1;
    console.log("跳转到index:", index)
    msgScroller.value?.scrollToBottom();
    console.log(msgScroller.value);
  }
}, {deep: true});


function formatTime(ms: number) {
  const hours = Math.floor(ms / 3600000).toString().padStart(2, "0");
  const minutes = Math.floor(ms / 60000 % 60).toString().padStart(2, "0");
  const seconds = Math.floor(ms / 1000 % 60).toString().padStart(2, "0");
  const milliseconds = (ms % 1000).toString().padStart(3, "0");

  return `${hours}:${minutes}:${seconds}.${milliseconds}`;
}

</script>

<template>
<div class="root">
  <button
      style="
      position: absolute;
      top: 20%;
      left: 20%;
      width: 100px;
      height: 50px;
      background: #ccc;"

      @click="testSerialPort">
    串口检测
  </button>
  <button
      style="
position: absolute;
top: 40%;
left: 20%;
width: 100px;
height: 50px;
background: #ccc;"
      @click="stopSerialPort"
  >
    停止串口
  </button>
  <DynamicScroller
      class="scroller"
      ref="msgScroller"
      :items="messages"
      :min-item-size="60"
      key-field="millionSec"
      v-slot="{ item, index, active }"
      @scroll="handleScroll"
  >
    <DynamicScrollerItem
        :item="item"
        :active="active"
        :size-dependencies="[
            item.data.length,
        ]"
        :data-index="index"
        :data-active="active"
    >
      <div class="message-box">
        <div class="time">
          <span>{{ formatTime(item.millionSec) }}</span>
        </div>
        <div class="data">
        <span class="message-text">
          {{ String.fromCharCode(...item.data) }}
        </span>
        </div>
      </div>
    </DynamicScrollerItem>

  </DynamicScroller>

</div>
</template>

<style scoped>

.root {
  position: absolute;
  width: 100%;
  height: 100%;
  z-index: -2;
}

.scroller {
  position: absolute;
  top: 10%;
  left: 40%;
  height: 90%;
  width: 60%;
  background-color: #eee;
  border-radius: 15px;
}

.message-box {
  padding: 16px;
  border-bottom: 1px solid #ccc;
  background-color: white;
  transition: all 0.2s ease;
  left: 5%;
  width: 90%;

  .time {
    background-color: #7695FF;
    height: 20px;
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .message-text {
    white-space: pre-wrap;
    tab-size: 16; /* 4 个空格宽度 */
    display: inline-block; /* 或 inline-flex */
    word-break: break-word; /* 长字符串换行 */
  }

}
</style>