<script setup lang="ts">
defineProps<{ msg: string }>();
</script>

<template>
  <div>
    <p><span>{{ msg }}</span></p>
  </div>
</template>

<style scoped>
p {
  margin: 1.5rem 0;
  text-align: center;
  position: relative;
}

span {
  position: relative;
  z-index: 1;
  background: var(--bg-color-3);
  padding: 0 0.9rem;
  font-size: 1.06rem;
  font-weight: 500;
}

p::after {
  position: absolute;
  content: "";
  left: 0;
  top: 50%;
  width: 100%;
  height: 1px;
  background-color: var(--fg-color);
}
</style>
