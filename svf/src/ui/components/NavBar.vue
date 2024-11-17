<template>
  <div>
    <header>
      <router-link to="/">
        <img src="../assets/logo.svg" alt="SVF">
      </router-link>
      <nav>
        <router-link class="link" to="/term-of-service">Term of Service</router-link>
        <router-link class="link" to="/privacy">Privacy Policies</router-link>
      </nav>
      <div class="btn-contatiner">
        <router-link to="/app/logout" class="button">Logout</router-link>
        <router-link to="/app/login" class="button">Login</router-link>
        <router-link to="/app/signup" class="button">Sign up</router-link>
      </div>
    </header>
  </div>
</template>

<style lang="css" scoped>
* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

.btn-container {
  display: flex;
  gap: 0.5rem; /* Space between buttons */
}

.button {
  margin-right: 5px;
  margin-left: 5px;
  border-radius: 8px;
  border: 1px solid var(--bg-color-2);
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: var(--fg-color);
  background-color: var(--bg-color);
  cursor: pointer;
  transition: background-color 0.25s;
  box-shadow: 0 10px 20px var(--bg-color);
}

.button:hover {
  background-color: var(--bg-color-4);
}

.button:focus,
.button:focus-visible {
  outline: 4px auto -webkit-focus-ring-color;
}


nav .link {
  margin: 0 1rem;
  text-decoration: none;
  color: var(--fg-color);
  font-weight: bold;
}

nav {
  position: absolute;
  left: 50%;
  transform: translateX(-50%);
  flex-grow: 1;
  text-align: center;
}

header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1rem; 
  background-color: var(--bg-color-4);
  box-shadow: 0 10px 20px var(--bg-color-3);
}
</style>
