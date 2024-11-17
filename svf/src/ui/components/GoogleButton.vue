<script setup lang="ts">
import { CallbackTypes, googleTokenLogin } from 'vue3-google-login';

const props = defineProps<{ response: (token: CallbackTypes.TokenPopupResponse) => void }>();

const handleGoogleLogin = () => {
  googleTokenLogin().then(props.response)
  .catch((err) => {
    console.log(err.message);
  });
}
</script>

<template>
  <div>
    <button @click="handleGoogleLogin">
      <img src="../assets/google.svg" alt="Google" class="social-icon">
      Google
    </button>
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

button {
  display: flex;
  width: 100%;
  gap: 0.81rem;
  cursor: pointer;
  font-size: 1rem;
  padding: 0.75rem 0;
  align-items: center;
  border-radius: 0.31rem;
  background: var(--bg-color);
  border: 1px solid var(--bg-color-2);
  justify-content: center;
  transition: 0.2s ease;
}

button:hover {
  background: var(--bg-color-4);
}


div {
  display: flex;
  gap: 1.31rem;
}

.social-icon {
  width: 23px;
}
</style>
