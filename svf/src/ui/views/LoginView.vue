<script setup lang="ts">
import { CallbackTypes } from 'vue3-google-login';
import AppNavBar from '../components/AppNavBar.vue';
import GoogleButton from '../components/GoogleButton.vue';
import Seperator from '../components/Seperator.vue';
import FormContainer from '../components/FormContainer.vue';
import { RouterLink } from 'vue-router';
import InputField from '../components/InputField.vue';
import UserIcon from '../assets/user.svg';
import PasswordIcon from '../assets/password.svg';
import FormButton from '../components/FormButton.vue';
import router from '../router';
import Swal from 'sweetalert2';
import { inject } from 'vue';
import { VueCookies } from 'vue-cookies';
import CryptoJS from 'crypto-js';

inject<VueCookies>('$cookies');
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
    async handleUsernameLogin() {
      const res = await fetch(`${this.$server}/login/password-challenge`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          username: this.username,
        }),
        credentials: 'include',
      });
      const result: { PasswordChallenge: string } | { Error: string } = await res.json();
      if ("PasswordChallenge" in result) {
        const sha256 = CryptoJS.algo.SHA256.create();

        sha256.update(CryptoJS.enc.Utf8.parse(CryptoJS.SHA256(this.password).toString(CryptoJS.enc.Hex)));
        sha256.update(CryptoJS.enc.Utf8.parse(result.PasswordChallenge));

        const hash = sha256.finalize().toString(CryptoJS.enc.Hex);
        const login_res = await fetch(`${this.$server}/login/username`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({
            username: this.username,
            password_challenge_hash: hash,
          }),
          credentials: 'include',
        });
        const login_result: { AccessToken: string } | { Error: string } = await login_res.json();
        if ("AccessToken" in login_result) {
          this.$cookies.set("accessToken", login_result.AccessToken);
          router.push({ path: "/app/devices"});
        } else {
          Swal.fire({ 
            title: "Error",
            icon: 'error',
            color: "var(--fg-color)",
            text: login_result.Error,
            background: "var(--bg-color-4)",
            customClass: {
              confirmButton: 'confirm-button-style',
            },
          });
        }
      } else {
        Swal.fire({ 
          title: "Error",
          icon: 'error',
          color: "var(--fg-color)",
          text: result.Error,
          background: "var(--bg-color-4)",
          customClass: {
            confirmButton: 'confirm-button-style',
          },
        });
      }

    },
    async handleGoogleLogin(response: CallbackTypes.TokenPopupResponse) {
      const res = await fetch(`${this.$server}/login/google`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          google_access_token: response.access_token,
        }),
        credentials: 'include',
      });
      const result: { AccessToken: string } | { Error: string } = await res.json();
      if ("AccessToken" in result) {
        this.$cookies.set("accessToken", result.AccessToken);
        router.push({ path: "/app/devices"});
      } else {
        Swal.fire({ 
          title: "Error",
          icon: 'error',
          color: "var(--fg-color)",
          text: result.Error,
          background: "var(--bg-color-4)",
          customClass: {
            confirmButton: 'confirm-button-style',
          },
        });
      }
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

      <form @submit.prevent="handleUsernameLogin" class="login-form">
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
