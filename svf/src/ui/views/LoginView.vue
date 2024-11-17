<script setup lang="ts">
import { CallbackTypes } from 'vue3-google-login';
import AppNavBar from '../components/AppNavBar.vue';
import { VueCookies } from 'vue-cookies';
import { inject } from 'vue';
import GoogleButton from '../components/GoogleButton.vue';
import Seperator from '../components/Seperator.vue';
import FormContainer from '../components/FormContainer.vue';
import { RouterLink } from 'vue-router';
import InputField from '../components/InputField.vue';
import UserIcon from '../assets/user.svg';
import PasswordIcon from '../assets/password.svg';
import FormButton from '../components/FormButton.vue';

const $cookies = inject<VueCookies>('$cookies');
const handleGoogleLogin = async (response: CallbackTypes.TokenPopupResponse) => {
  const result = await fetch(`https://www.googleapis.com/oauth2/v1/userinfo?access_token=${response.access_token}`);
  const {
    email,
  } = await result.json();
  console.log(email);
  $cookies?.set("googleToken", response.access_token);
}

</script>

<script lang="ts">
export default {
  data() {
    return {
      username: "",
      password: ""
    };
  },
  methods: {
    handleUsernameLogin() {
      console.log(this.username);
      console.log(this.password);
    }
  },
}
</script>

<template>
  <AppNavBar page="Login" />
  <div class="login-page">
    <FormContainer msg="Login with">
      <GoogleButton :response="handleGoogleLogin" />
      <Seperator msg="or" />

      <form @summit.prevent="handleUsernameLogin" class="login-form">
        <InputField v-model:input="username" placeholder="Username" type="text" :icon="UserIcon"/>
        <InputField v-model:input="password" placeholder="Password" type="password" :icon="PasswordIcon"/>
        <router-link to="/app/reset_password" class="forgot-pass-text">Forgot Password?</router-link>
        <FormButton text="Log In"/>
      </form>

      <p class="signup-text">Don't have an account? <router-link to="/app/signup" class="signup-link">Signup now</router-link></p>
    </FormContainer>
  </div>
</template>


<style scoped>
@import url('https://fonts.googleapis.com/css2?family=Caveat:wght@400..700&family=Montserrat:ital,wght@0,100..900;1,100..900&display=swap');

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
  font-family: "Montserrat", sans-serif;
  color: var(--fg-color);
}

.login-page {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 100vh;
}

.forgot-pass-text {
  font-weight: 500;
  color: var(--fg-color-2);
  text-decoration: none;
}

.forgot-pass-text:hover {
  text-decoration: underline;
}

.signup-text {
  text-align: center;
  font-weight: 500;
  margin: 1.75rem 0 0.31rem;
}

.signup-link {
  color: var(--fg-color-2);
  font-weight: 500;
  text-decoration: none;
}

.signup-link:hover {
  text-decoration: underline;
}

</style>
