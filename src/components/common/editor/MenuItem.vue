<script setup lang="ts">
interface Item {
  type?: string,
  title?: string,
  icon?: string,
  action?: () => void,
  isActive?: () => boolean
}

 const {icon,title,action,isActive} =
     defineProps<Item>();
</script>
<template>
  <button
      class="menu-item"
      :class="{ 'is-active': isActive ? isActive(): null }"
      @click="action"
      :title="title"
  >

    <i :class="`bi bi-${icon}`"></i>
  </button>
</template>

<script lang="ts">
import 'bootstrap-icons/font/bootstrap-icons.css';
import 'bootstrap-icons/font/fonts/bootstrap-icons.woff';
import 'bootstrap-icons/font/fonts/bootstrap-icons.woff2';
import {defineComponent} from "vue";
export default defineComponent({
})
</script>

<style >
.menu-item {
  width: 2rem;
  height: 2rem;
  color: #0d0d0d;
  border: none;
  background-color: transparent;
  border-radius: 0.4rem;
  padding: 0.25rem;
  margin-right: 0.25rem;
}
.menu-item i {
  margin: auto auto;
}
.menu-item.is-active, .menu-item:hover {
  color: #fff;
  background-color: #0d0d0d;
}
</style>