import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Home.vue'
import Overview from '../views/Overview.vue'
import Prepare from '../views/Prepare.vue'
import type { RouteRecordSingleView, Router} from 'vue-router'

const routes: RouteRecordSingleView[] = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/overview',
    name: 'Overview',
    component: Overview
  },
  {
    path: '/prepare',
    name: 'Prepare',
    component: Prepare
  }
]
const router: Router = createRouter({
  history: createWebHistory('/'),
  routes
})

export default router
