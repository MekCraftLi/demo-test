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


</script>

<template>
<div class="title-bar">

  <!-- 图标 -->
  <img class="icon" src="../assets/icons/processed.png" alt="▢" @dblclick="closeHandle" />

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

  background:linear-gradient(to right, #b7bddf, #d5d8ec);
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

.buttons {
  position:absolute;
  height: 80%;
  width: 100px;
  right: 10px;
  top: 8%;
  border-radius: 15px;
  display: flex;
  align-items: center; /* 垂直居中 */
  justify-content: space-around;
  background-color: rgba(100, 100, 100, 0.1);
}

.button {
  width: 17px;
  height: 17px;
  border-radius: 50%;
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