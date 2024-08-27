import { createRouter, createWebHistory } from 'vue-router';
import Home from './../components/Home.vue';

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home,
  },
  {
    path: '/services',
    name: 'Services',
    component: () => import('./../components/Services.vue'),
  },
  {
    path: '/about',
    name: 'About',
    component: () => import('./../components/About.vue'),
  },
  {
    path: '/contact',
    name: 'Contact',
    component: () => import('./../components/Contact.vue'),
  }
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
