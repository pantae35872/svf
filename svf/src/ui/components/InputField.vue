<script setup lang="ts">
  defineProps<{ placeholder: string, type: string, icon: string, input: string}>();
</script>

<template>
  <div>
    <input v-model="value" :type="type" :placeholder="placeholder" required />
    <i><img :src="icon" alt=""></i>
  </div>
</template>

<script lang="ts">
export default {
  data() {
    return { 
      value: this.input
    }
  },
  watch: {
    value(newValue) {
      this.$emit("update:input", newValue);
    }
  }
}
</script>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=Caveat:wght@400..700&family=Montserrat:ital,wght@0,100..900;1,100..900&display=swap');

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
  font-family: "Montserrat", sans-serif;
  color: var(--fg-color);
}

div {
  height: 54px;
  width: 100%;
  margin-bottom: 1.5rem;
  position: relative;
}

input {
  height: 100%;
  width: 100%;
  outline: none;
  border-radius: 0.31rem;
  border: 1px solid var(--fg-color);
  background-color: var(--bg-color);
  padding: 0 1.25rem 0 3.12rem;
  font-size: 1.06rem;
  transition: 0.2s ease;
}

input:focus {
  border-color: var(--fg-color-2);
}

input::placeholder {
  color: var(--fg-color);
}

i {
  color: var(--fg-color);
  position: absolute;
  top: 50%;
  left: 0.9rem;
  pointer-events: none;
  transform: translateY(-50%);
}

input:focus ~ i {
  color: var(--fg-color-2);
}
</style>
