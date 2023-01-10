<script setup lang="ts">

import {BookModel} from "../../types/model";

const {book} = defineProps<{book:BookModel}>();

</script>
<template>
  <div class="book-item-content">
    <div class="book-title">
      <span>{{book.name}}</span>
    </div>
    <div class="book-desc-content">
      <label class="book-desc-item">总字数:
        <span>11</span>
      </label>
      <label class="book-desc-item">更新时间:
        <span>{{book.create_time}}</span>
      </label>
    </div>
  </div>
</template>

<script lang="ts">
export default {
  name: "BookItem"
}
</script>

<style scoped>
.book-item-content{
  display: flex;
  height: 64px;
  flex-direction: column;
  flex-flow: row wrap ;
}
.book-title{
  font-size: 1.1rem;
  font-weight: 500;
}
.book-desc-content{
  width: 100%;
  display: flex;
  flex-grow: 1;
  font-size: 0.7rem;
  color: darkgray;
  align-self: flex-end;
}
.book-desc-item:not(:last-child){
  margin-right: 5px;
}
.book-desc-item{
  justify-content: flex-end;
}

</style>