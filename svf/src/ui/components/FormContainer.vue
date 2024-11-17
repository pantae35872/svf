<script setup lang="ts">
  defineProps<{ msg: string }>();
</script>

<template>
  <div class="login-container">
    <h2 class="form-title">
      {{ msg }}
    </h2>
    <slot />
  </div>
</template>

<style scoped>
.login-container {
  width: 100%;
  max-width: 600px;
  margin: 3rem 1rem;
  border-radius: 0.5rem;
  padding: 2rem 1.5rem;
  background: var(--bg-color-3);
  box-shadow: 0 10px 20px var(--bg-color-4);
}

.login-container .form-title {
  text-align: center;
  font-size: 1.37rem;
  font-weight: 600;
  margin-bottom: 1.87rem;
}

</style>
