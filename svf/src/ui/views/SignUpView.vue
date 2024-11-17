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

const handleGoogleSignup = async (response: CallbackTypes.TokenPopupResponse) => {
  const result = await fetch(`https://www.googleapis.com/oauth2/v1/userinfo?access_token=${response.access_token}`);
  const {
    //id,
    email,
    //picture,
  } = await result.json();
  console.log(email);
}
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
  methods: {
    handleUsernameSignup() {
      console.log(this.username);
      console.log(this.password);
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

      <form @summit.prevent="handleUsernameSignup" class="login-form">
        <InputField v-model:input="username" placeholder="Username" type="text" :icon="UserIcon"/>
        <InputField v-model:input="password" placeholder="Password" type="password" :icon="PasswordIcon"/>
        <InputField v-model:input="retype_password" placeholder="Retype Password" type="password" :icon="PasswordIcon"/>
        <TOSAgreement />
        <FormButton text="Sign Up"/>
      </form>
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

.signup-page {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 100vh;
}
</style>
