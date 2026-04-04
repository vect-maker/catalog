import { createMemoryHistory, createRouter } from 'vue-router'
import { useAuthStore } from './stores/useAuthStore';

const routes = [
  { path: '/', name: "catalog", component: () => import('./views/Catalog.vue'), meta: { requiresAuth: false } },
  { path: '/products/create', name: "productCreate", component: () => import('./views/CreateProduct.vue'), meta: { requiresAuth: true } },
  { path: '/login', name: "login", component: () => import("./views/Login.vue"), meta: { requiresAuth: false } }

]

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
})

router.beforeEach((to, _, next) => {
  const auth = useAuthStore();

  const isProtected = to.matched.some(record => record.meta.requiresAuth);

  if (isProtected && !auth.isAuthenticated) {
    next({ name: 'login', query: { redirect: to.fullPath } });
  } else if (to.name === 'login' && auth.isAuthenticated) {
    next({ name: 'catalog' });
  } else {
    next();
  }
});