import { createApp } from 'vue';
import App from './App.vue';
import './style.css';
import router from './router';
import vue3GoogleLogin from 'vue3-google-login';
import VueCookies from 'vue-cookies';

createApp(App).use(router).use(vue3GoogleLogin, {
  clientId: '606126667998-c7vdmtohcqbvegnksl78a8htrogc9si3.apps.googleusercontent.com'
}).use(VueCookies, {
  expires: '7d',
}).mount('#app')
