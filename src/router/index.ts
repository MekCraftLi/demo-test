import {createRouter, createWebHistory} from 'vue-router'

import Splash from '../windows/Splash.vue'
import Main from '../windows/Main.vue'

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
        }
    ]
})

export default index;