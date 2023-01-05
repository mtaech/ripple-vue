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

<script>
import remixiconUrl from 'remixicon/fonts/remixicon.symbol.svg'
import bootstrapUrl from 'bootstrap-icons/bootstrap-icons.svg';
import 'bootstrap-icons/font/bootstrap-icons.css';
import 'bootstrap-icons/font/fonts/bootstrap-icons.woff';
import 'bootstrap-icons/font/fonts/bootstrap-icons.woff2';
export default {
  props: {
    icon: {
      type: String,
      required: true,
    },

    title: {
      type: String,
      required: true,
    },

    action: {
      type: Function,
      required: true,
    },

    isActive: {
      type: Function,
      default: null,
    },
  },

  data() {
    return {
      remixiconUrl,
      bootstrapUrl
    }
  },
}
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