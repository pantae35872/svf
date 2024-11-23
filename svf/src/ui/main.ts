import { createApp } from 'vue';
import App from './App.vue';
import './style.css';
import router from './router';
import vue3GoogleLogin from 'vue3-google-login';
import VueCookies from 'vue-cookies';

const app = createApp(App);

app.config.globalProperties.$server = process.env.NODE_ENV === 'development' ? 'http://127.0.0.1:3000' : 'https://svf-backend.duckdns.org';

app.use(router);
app.use(vue3GoogleLogin, {
  clientId: '606126667998-c7vdmtohcqbvegnksl78a8htrogc9si3.apps.googleusercontent.com'
});
app.use(VueCookies, {
  expires: '7d',
});
app.mount('#app');
