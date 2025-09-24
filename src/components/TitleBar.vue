<script setup lang="ts">
import {invoke} from '@tauri-apps/api/core'

async function maximizeHandle () {
  await invoke('maximize_window', {
    label: 'MEKs-Embedded-Kommand',
  });
}

async function minimizeHandle () {
  await invoke('minimize_window', {
    label: 'MEKs-Embedded-Kommand',
  });
}

async function closeHandle () {
  await invoke('close_window', {
    label: 'MEKs-Embedded-Kommand',
  });
}

async function resetHandle() {
  await invoke('reset_window', {
    opts: {
      label: "MEKs-Embedded-Kommand",
      height: 720,
      width: 1280,
      url: '',
    }
  })
}

</script>

<template>
<div class="title-bar">

  <!-- 图标 -->
  <div class="buttons" style="right: 125px; width: 60px; border-radius: 15px">
    <button class="button" style="width: 40px; border-radius: 8px; background-color:#eae2b7 " @click="resetHandle">
      <span class="content">
        Re
      </span>

    </button>
  </div>

  <div class="buttons">
    <div class="button" @click="minimizeHandle" id="minimized"></div>
    <div class="button" @click="maximizeHandle" id="maximized"></div>
    <div class="button" @click="closeHandle" id="closed"></div>
  </div>



</div>

</template>

<style scoped>

.title-bar {
  position: absolute;
  display: flex;

  background:linear-gradient(to right, rgba(0, 0, 0, 0) 20%, #635dc5);
  width: 100%;
  height: 30px;
  -webkit-app-region: drag; /* 指定可拖动区域 */
  align-content: center;
  justify-content: center;
}

.icon {
  height: 80%;
  position: absolute;
  left: 8px;
  top: 10%;
}

.icon:hover {
  background-color: rgba(10, 10, 10, 0.1);
  border-radius: 30%;

}

@font-face {
  font-family: "Arial Black";
  src: url("@/assets/font/ariblk.ttf") format("ttf");
  font-weight: normal;
  font-style: normal;
}

.content {
  position: absolute;
  color: #3a0ca3;
  font-family: "Arial Black";
  transform: translate(-50%, -50%);
  left: 50%;
  top: 50%;

}

.buttons {
  position:absolute;
  height: 80%;
  width: 100px;
  right: 10px;
  top: 10%;
  border-radius: 15px;
  display: flex;
  align-items: center; /* 垂直居中 */
  justify-content: space-around;
  background-color: #7695FF;
  box-shadow: inset 2px 1px 2px #03346E;
}

.button {
  width: 17px;
  height: 17px;
  border-radius: 50%;
  box-shadow: 2px 1px 2px #03346E;
}

.button:hover {
  scale: 1.1;
}

#maximized {
  background-color: gold;
}

#minimized {
  background-color: limegreen;
}

#closed {
  background-color: red;
}

</style>