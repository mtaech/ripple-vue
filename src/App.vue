<script setup lang="ts">
import Home from "./components/home/Index.vue";
import {GlobalThemeOverrides} from "naive-ui";

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

  <n-config-provider locale="zhCN" date-locale="dateZhCN">
    <n-message-provider>
      <div class="container">
        <Home/>
      </div>
    </n-message-provider>
  </n-config-provider>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
