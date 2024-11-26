<script setup lang="ts">
  import NavBar from '../components/NavBar.vue';
   
  import { VueCookies } from 'vue-cookies';
  import { inject } from 'vue';
import router from '../router';

  inject<VueCookies>('$cookies');
</script>

<script lang="ts">
export default {
  mounted() {
    if (this.$cookies.get("accessToken") != null) {
      router.push({ path: "/app/devices"});
    }
  },
}
</script>

<template>
  <NavBar />
  <div class="home-container">
    <h1>Strawberry Vision Farm</h1>
  </div>
</template>

<style scoped>
.home-container {
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
