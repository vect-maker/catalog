import { createMemoryHistory, createRouter } from 'vue-router'
import Catalog from './views/Catalog.vue'

const routes = [
  { path: '/', component: Catalog },
  
]

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
})