<script setup lang="ts">

import {BookModel} from "../../types/model";
import {convertFileSrc} from "@tauri-apps/api/tauri";
import {useEnvStore} from "../../Store";
import {request} from "../../api/request";
import routers from "../../Routers";


const {bookList} = defineProps<{bookList:BookModel[]}>();
const envStore = useEnvStore();
const textCountRef = ref()


function getCoverSrc(coverId: string):string {
  console.log("cover di",coverId)
  if (coverId){
    return convertFileSrc(coverId);
  }
  return convertFileSrc(envStore.env.cover_dir_path+"/cover_dark.png");
}

function getBookTextCount() {
  request('get_book_text_count').then(res=>{
    textCountRef.value = res
  })
}

function toChapterList(bookId: string, bookName: string) {
  routers.push({name: 'bookInfo', params: {bookId: bookId, bookName: bookName}})
}

onBeforeMount(()=>{
  envStore.getEnv();
  getBookTextCount();
})
</script>
<template>
  <div>
    <n-list-item v-if="bookList.length> 0" v-for="book in bookList" @click="toChapterList(book.id,book.name)">
      <div class="book-item-content">
        <div class="book-cover">
          <n-image
              :src="getCoverSrc(book.cover_path)"
              object-fit="fill"
              style="width: 75px;height: 100px;border-radius: 5px"></n-image>
        </div>
        <div class="book-info-content">
          <div class="book-title">
            <span>{{book.name}}</span>
          </div>
          <div class="book-white-content"></div>

          <div class="book-desc-content">
            <label class="book-desc-item">总字数:
              <span>{{ textCountRef[book.id] ? textCountRef[book.id] : 0 }}</span>
            </label>
            <label class="book-desc-item">更新时间:
              <span>{{book.create_time}}</span>
            </label>
          </div>
        </div>
      </div>
    </n-list-item>
    <n-empty class="empty-list-content" size="large" v-else></n-empty>
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
  flex-direction: column;
  flex-flow: row wrap;
}
.book-cover{
  display: flex;
  margin-right: 10px;
}
.book-title{
  font-size: 1.5rem;
  font-weight: 500;
}
.book-info-content{
  width: 100px;
  display: flex;
  flex-grow: 1;
  flex-flow: column wrap ;
  padding-top: 5px;
}
.book-white-content{
  flex-grow: 1;
}
.book-desc-content{
  display: flex;
  font-size: 1rem;
  color: darkgray;
  align-self: flex-start;
}
.book-desc-item:not(:last-child){
  margin-right: 5px;
}
.book-desc-item{
  justify-content: flex-end;
}

</style>