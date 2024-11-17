<script setup lang="ts">
  import AppNavBar from '../components/AppNavBar.vue';

  const logout = () => {
  }
</script>

<template>
  <div>
    <AppNavBar page="Logout"/>
    <div class="logout_container">
      <button @click="logout">Logout</button>
    </div>
  </div>
</template>
