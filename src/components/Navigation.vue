<script setup lang="ts">

import {computed, ref} from "vue";

const selected = ref(null)



interface OptItem {
  index: number;
  url: string;
  name: string;
}

const Options: OptItem[] = [
  {index: 0, url: '', name: "通讯信息显示",},
  {index: 1, url: '', name: "Flash资源管理"}
]

const navBoxStyle = computed(() => {
  return 'height: '+ (Options.length * 50 + 60)  + 'px;';
});

const navBase = computed(() => {

})


</script>

<template>

  <div class="nav-base-box" :style="navBoxStyle">
    <svg  viewBox="0 0 100 100" width="100%" height="100%" preserveAspectRatio="none">

      <path
          :d="`M`+navBase "
          fill="none"
          stroke="blue"
          stroke-width="4"
          filter="url(#neon-glow)"
      />

    </svg>
  </div>

</template>

<style scoped>
.nav-base-box {
  position: absolute;
  width: 100px;
  height: 50px;
  background: #ccc;
  top: 50%;
  left: 4%;
  transform: translate(-50%, -50%);
}
</style>