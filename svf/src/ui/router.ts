import { createRouter, createWebHistory } from "vue-router";

import HomeView from './views/HomeView.vue';
import NotFoundView from './views/NotFoundView.vue';
import LoginView from "./views/LoginView.vue";
import LogoutView from "./views/LogoutView.vue";
import AppView from "./views/AppView.vue";
import TOSView from "./views/TOSView.vue";
import PrivacyView from "./views/PrivacyView.vue";
import SignUpView from "./views/SignUpView.vue";

const routes = [
  { path: '/', component: HomeView },
  {
    path: '/privacy', component: PrivacyView
  },
  {
    path: '/term-of-service', component: TOSView
  },
  { path: '/app', component: AppView, children: [
    {
      path: 'login',
      component: LoginView,
    },
    {
      path: 'logout',
      component: LogoutView,
    },
    {
      path: 'signup',
      component: SignUpView,
    }
  ]},
  { path: '/:pathMatch(.*)*', component: NotFoundView }
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

export default router;
