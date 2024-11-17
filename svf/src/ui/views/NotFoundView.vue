<template>
  <div class="not-found">
    <h1>404 - Page Not Found</h1>
    <p>Sorry, the page you are looking for does not exist.</p>
    <router-link to="/" class="button">Go back to Home</router-link>
  </div>
</template>

<style scoped>
.not-found {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 100vh;
  text-align: center;
}

.button {
  position: relative;
  font-size: 1.3em;
  top: -30px;
}

p {
  position: relative;
  font-size: 1.3em;
  top: -30px;
}

h1 {
  font-size: 3.6em;
  line-height: 1.1;
}
</style>

