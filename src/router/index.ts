import {createRouter, createWebHistory} from 'vue-router'

import Splash from '@/windows/Splash.vue'
import Main from '@/windows/Main.vue'

import SerialTran from '@/components/SerialTran.vue'
import Explorer from "@/components/views/ExplorerView/index.vue"
import Retrans from "@/components/Retrans.vue";

const index = createRouter({
    history: createWebHistory(),

    routes: [
        {
            path: '/',
            component: Splash,
        },
        {
            path: '/Main',
            component: Main,
            children: [
                {
                    path: 'serial',
                    component: SerialTran
                },
                {
                    path: 'explorer',
                    component: Explorer
                },
                {
                    path: 'retrans',
                    component: Retrans
                }
            ]
        }
    ]
})

export default index;