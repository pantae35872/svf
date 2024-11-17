<template>
  <div>
    <router-view></router-view>
  </div>
</template>

<script lang="ts">
const color_vars = ['--bg-color', '--fg-color', '--bg-color-4', '--bg-color-2',
'--bg-color-3', '--fg-color-2'];
export default {
  data() {
    return {
      colorScheme: 'light',
    }
  },
  mounted() {
    const savedColorScheme = window.localStorage.getItem('colorScheme');
    if (savedColorScheme) {
      this.colorScheme = savedColorScheme;
      this.updateCssVariables();
    } else {
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
      this.colorScheme = mediaQuery.matches ? 'dark' : 'light';
      mediaQuery.addEventListener('change', (event) => {
        this.colorScheme = event.matches ? 'dark' : 'light'; 
      });
      this.updateCssVariables();
    }
  },
  watch: {
    colorScheme() {
      this.updateCssVariables();
    }
  },
  methods: {
    toggleColorScheme() {
      this.colorScheme = this.colorScheme == 'dark' ? 'light' : 'dark';
      window.localStorage.setItem('colorScheme', this.colorScheme);
    },
    updateCssVariables() {
      const root = document.documentElement;
      if (this.colorScheme === 'dark') {
        color_vars.forEach((color_var) => {
          root.style.setProperty(color_var, `var(${color_var}-dark)`);
        })
      } else {
        color_vars.forEach((color_var) => {
          root.style.setProperty(color_var, `var(${color_var}-light)`);
        })
      }
    }
  }
}
</script>
