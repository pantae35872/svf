<script setup lang="ts">
import { CallbackTypes } from 'vue3-google-login';
import AppNavBar from '../components/AppNavBar.vue';
import FormContainer from '../components/FormContainer.vue';
import GoogleButton from '../components/GoogleButton.vue';
import Seperator from '../components/Seperator.vue';
import UserIcon from '../assets/user.svg';
import PasswordIcon from '../assets/password.svg';
import InputField from '../components/InputField.vue';
import FormButton from '../components/FormButton.vue';
import TOSAgreement from '../components/TOSAgreement.vue';
import CryptoJS from 'crypto-js';
import Swal from 'sweetalert2';
import router from '../router';

</script>

<script lang="ts">
export default {
  data() {
    return {
      username: "",
      password: "",
      retype_password: "",
    };
  },
  mounted() {
  },
  methods: {
    async handleUsernameSignup() {
      const response = await fetch(`${this.$server}/signup/username`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          username: this.username,
          password_hash: CryptoJS.SHA256(this.password).toString(CryptoJS.enc.Hex),
        })
      });
      const data: "Ok" | { Error: string } = await response.json();
      if (data == "Ok") {
        router.push({ path: "/app/login"});
      } else {
        Swal.fire({ 
          title: "Error",
          icon: 'error',
          color: "var(--fg-color)",
          text: data.Error,
          background: "var(--bg-color-4)",
          customClass: {
            confirmButton: 'confirm-button-style',
          },
        });
      }
    },
    async handleGoogleSignup(response: CallbackTypes.TokenPopupResponse) {
      let success = false;
      while(!success) {
        const username = await Swal.fire({
          title: "Username",
          input: "text",
          confirmButtonText: "Continue",
          showCancelButton: true,
          color: "var(--fg-color)",
          background: "var(--bg-color-4)",
          confirmButtonColor: "var(--bg-color-3)",
          cancelButtonColor: "var(--bg-color)",
          customClass: {
            input: 'alert-input-style',
            confirmButton: 'confirm-button-style',
            cancelButton: 'cancel-button-style',
          },
          inputAttributes: {
            autocapitalize: "off",
          },
          allowOutsideClick: false,
        });
        if (username.isDismissed) {
          break;
        }
        if (typeof username.value == "string") {
            const res = await fetch(`${this.$server}/signup/google`, {
              method: 'POST',
              headers: {
                'Content-Type': 'application/json',
              },
              body: JSON.stringify({
                username: username.value, 
                google_access_token: response.access_token,
              })
            });
            const result: "Ok" | { Error: string } = await res.json();
            if (result == "Ok") {
              success = true;
              router.push({ path: "/app/login"});
            } else {
              await Swal.fire({ 
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
        }
    }
  },
}
</script>

<template>
  <div>
    <AppNavBar page="Sign Up"/>
  </div>
  <div class="signup-page">
    <FormContainer msg="Sign Up with">
      <GoogleButton :response="handleGoogleSignup" />
      <Seperator msg="or" />

      <form @submit.prevent="handleUsernameSignup" class="login-form">
        <InputField v-model:input="username" placeholder="Username" type="text" :icon="UserIcon"/>
        <InputField v-model:input="password" placeholder="Password" type="password" :icon="PasswordIcon"/>
        <InputField v-model:input="retype_password" placeholder="Retype Password" type="password" :icon="PasswordIcon"/>
        <TOSAgreement />
        <FormButton text="Sign Up"/>
      </form>
    </FormContainer>
  </div>
</template>

<style lang="css">
.alert-input-style {
  background-color: var(--bg-color);
  color: var(--fg-color);
}
.confirm-button-style {
  background-color: var(--bg-color);
  color: var(--fg-color);
}
.cancel-button-style {
  background-color: var(--bg-color);
  color: var(--fg-color);
}
</style>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=Caveat:wght@400..700&family=Montserrat:ital,wght@0,100..900;1,100..900&display=swap');

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
  font-family: "Montserrat", sans-serif;
  color: var(--fg-color);
}
.signup-page {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 100vh;
}
</style>
