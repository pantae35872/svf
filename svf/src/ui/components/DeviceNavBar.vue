<script setup lang="ts">
import Swal from 'sweetalert2';
import router from '../router';
import { request_berry } from '../berry';

const settings = () => {
  router.push({ path: "/app/settings"});
};
</script>

<script lang="ts">
export default {
  methods: {
    async add_devices() {
      console.log(await request_berry());
    }
  }
}
</script>

<template>
  <div class="nav_div">
    <header>
      <button></button>
      <h1>Devices</h1>  
      <div class="button_div">
      <button @click="add_devices">
      <svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 -960 960 960" width="24px" fill="#e8eaed"><path d="M440-440H200v-80h240v-240h80v240h240v80H520v240h-80v-240Z"/></svg>
      </button>
      <button @click="settings">
        <svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 -960 960 960" width="24px" fill="currentColor"><path d="m370-80-16-128q-13-5-24.5-12T307-235l-119 50L78-375l103-78q-1-7-1-13.5v-27q0-6.5 1-13.5L78-585l110-190 119 50q11-8 23-15t24-12l16-128h220l16 128q13 5 24.5 12t22.5 15l119-50 110 190-103 78q1 7 1 13.5v27q0 6.5-2 13.5l103 78-110 190-118-50q-11 8-23 15t-24 12L590-80H370Zm70-80h79l14-106q31-8 57.5-23.5T639-327l99 41 39-68-86-65q5-14 7-29.5t2-31.5q0-16-2-31.5t-7-29.5l86-65-39-68-99 42q-22-23-48.5-38.5T533-694l-13-106h-79l-14 106q-31 8-57.5 23.5T321-633l-99-41-39 68 86 64q-5 15-7 30t-2 32q0 16 2 31t7 30l-86 65 39 68 99-42q22 23 48.5 38.5T427-266l13 106Zm42-180q58 0 99-41t41-99q0-58-41-99t-99-41q-59 0-99.5 41T342-480q0 58 40.5 99t99.5 41Zm-2-140Z"/></svg>
      </button>
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

img {
  color: var(--fg-color);
}

.nav_div {
  background-color: var(--bg-color-4);
  padding: 10px 10px 10px 10px;
  box-shadow: 0 10px 20px var(--bg-color-3);
}

button {
  background-color: transparent;
  border: none;
  cursor: pointer;
  padding-left: 1rem;
}

header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}


h1 {
  position: absolute;
  left: 50%;
  font-size: 1.5rem;
  transform: translateX(-50%);
  flex-grow: 1;
  text-align: center;
}

header {
  font-size: 1.5rem;
  text-align: center;
  flex-grow: 1;
}

</style>
