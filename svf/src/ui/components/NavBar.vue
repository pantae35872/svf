<script lang="ts">
export default {
  data() {
    return {
      isOpen: false,
    };
  },
  methods: {
    navOpen() {
      this.isOpen = !this.isOpen;
    },
  }
}
</script>

<template>
  <div>
    <header>
      <router-link to="/">
        <img src="/icon.png" alt="SVF" width="70" height="70">
      </router-link>
      <button @click="navOpen"><svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 -960 960 960" width="24px" fill="var(--fg-color)"><path d="M120-240v-80h720v80H120Zm0-200v-80h720v80H120Zm0-200v-80h720v80H120Z"/></svg></button>
      <nav :class="{'nav-open': isOpen}">
        <router-link class="router-link link" to="/term-of-service">Term of Service</router-link>
        <router-link class="router-link link" to="/privacy">Privacy Policies</router-link>
        <router-link to="/app/login" class="router-link button">Login</router-link>
        <router-link to="/app/signup" class="router-link button">Sign up</router-link>
      </nav>
      <div class="link-container">
        <router-link class="link" to="/term-of-service">Term of Service</router-link>
        <router-link class="link" to="/privacy">Privacy Policies</router-link>
      </div>
      <div class="btn-container"> 
        <router-link to="/app/login" class="router-link button">Login</router-link>
        <router-link to="/app/signup" class="router-link button">Sign up</router-link>
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


@media (width <= 768px) {
  .btn-container {
    display: none;
  }

  .link-container {
    display: none;
  }

  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem; 
    background-color: var(--bg-color-4);
    box-shadow: 0 10px 20px var(--bg-color-3);
  }
  
  button {
    border: none;
    background-color: rgba(0, 0, 0, 0);
    cursor: pointer;
  }
  
  nav {
    position: absolute;
    top: 64px;
    left: 0;
    width: 100%;
    padding: 2rem;
    list-style: none;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-direction: column;
    gap: 0.5rem;
    background-color: var(--bg-color-4);
    transition: 0.5s;
    z-index: -1;
    transform: translateY(-100%);
  }

  .nav-open {
    z-index: 1;
    transform: translateY(10%);
  }

  .router-link {
    border: 1px solid var(--bg-color);
    width: 100%;
    background-color: var(--bg-color-3);
    padding-top: 1rem;
    padding-bottom: 1rem;
    text-align: center;
    border-radius: 8px;
    color: var(--fg-color);
    transition: 0.2s;
  }

  .router-link:hover {
    background-color: var(--bg-color-4);
  }
}

@media (width > 768px) {
  button {
    display: none;
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

  nav {
    display: none;
  }

  .link-container .link {
    margin: 0 1rem;
    text-decoration: none;
    color: var(--fg-color);
    font-weight: bold;
  }

  .link-container {
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
    padding: 0.2rem; 
    padding-right: 1rem;
    background-color: var(--bg-color-4);
    box-shadow: 0 10px 20px var(--bg-color-3);
  }
}

</style>
