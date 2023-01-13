<script setup lang="ts">
import Home from "./components/home/Index.vue";
import {GlobalThemeOverrides} from "naive-ui";
import { zhCN, dateZhCN } from 'naive-ui'

const themeOverrides: GlobalThemeOverrides = {
  common: {
    primaryColor: '#FF0000'
  },
  Button: {
    textColor: '#FF0000'
  }
}
</script>

<template>
  <a-config-provider>
    <div class="container">
      <Home/>
    </div>
  </a-config-provider>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
