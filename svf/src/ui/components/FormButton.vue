<script setup lang="ts">
  defineProps<{ text: string }>();
</script>

<template>
  <button type="submit">{{ text }}</button>
</template>      

<style scoped>
button {
  width: 100%;
  height: 54px;
  cursor: pointer;
  outline: none;
  border: none;
  color: var(--fg-color);
  font-weight: 500;
  font-size: 1.125rem;
  background: var(--bg-color);
  margin-top: 2.19rem;
  border-radius: 0.31rem;
  transition: 0.2s ease;
}

button:hover {
  background: var(--bg-color-4);
}
</style>
