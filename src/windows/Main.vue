<script setup lang="ts">
import TitleBar from '../components/TitleBar.vue'
import Logo from '../components/Logo.vue'
import ParticlesBg from '../components/bckgnd/ParticlesBg.vue'
import Navigation from '../components/Navigation.vue'
import { OptItem } from '../components/Navigation.vue'
import {ref} from "vue";
import {useRouter} from "vue-router";



const router = useRouter();



const navFold = ref<boolean>(true);
const navLeft = ref<string>("-20%");
function toggleNav() {
  navFold.value = !navFold.value;
  navLeft.value = navFold.value? "-50px": "50px";
}

const options: OptItem[] = [
  {index: 0, url: 'serial', name: "通讯信息显示", width: 110},
  {index: 1, url: 'explorer', name: "Flash资源管理", width: 115},
  {index: 2, url: 'retrans', name: "消息转发", width: 80}
]





function onNavSelect(selectedIndex: number) {
  router.replace({path:'/Main/' + options[selectedIndex].url})
}







</script>

<template>
  <Logo style="position: absolute; z-index: 3"/>
  <TitleBar style="position: absolute; z-index: 2"/>

  <div
      class="absolute flex h-full w-full flex-col items-center justify-center overflow-hidden rounded-lg border bg-background md:shadow-xl z-0">
    <ParticlesBg class="absolute inset-0" :quantity="1000" :ease="100" :color="'#000'" :staticity="10" refresh/>
  </div>

<!--  <SerialTran style="position: absolute; z-index: 1"/>-->
  <RouterView style="height: 100%; width: 100%; position: absolute; z-index: 1"/>
  <div class="nav-box">
    <div class="unfold" @click="toggleNav">

    </div>
  <Navigation
      class="nav"
      :style="`left: ${navLeft}; z-index: 3`"
      :nav-items="options"
      @select="onNavSelect"
  />
  </div>
</template>

<style scoped>

.nav-box{

  .unfold {
    position: absolute;
    width: 10px;
    height: 50px;
    background-color: #eeeeee;
    transform-origin: center center;
    transform: translate(-50%, -50%);
    top: 50%;
    left: 1%;
    border-radius: 5px;
    box-shadow: 2px 2px 5px #888888;
    z-index: 5;
    transition: transform 0.5s ease;

  }
  .unfold:hover {
    transform: translate(-50%, -50%) scale(1.1);


  }

  .nav {
    position: absolute;
    transform: translate(-50%, -50%);
    top: 50%;
    transition: left 0.25s ease;
  }
}

</style>