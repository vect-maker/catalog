import { createMemoryHistory, createRouter } from 'vue-router'

const routes = [
  { path: '/', name: "catalog", component: () =>  import('./views/Catalog.vue') },
  { path: '/products/create', name: "productCreate", component: () =>  import('./views/CreateProduct.vue') },
  
]

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
})