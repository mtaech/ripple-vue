<script setup lang="ts">

import {BookModel} from "@/types/model";
import {convertFileSrc} from "@tauri-apps/api/tauri";
import {useEnvStore} from "@/Store";
import {request} from "@/api/request";
import routers from "@/Routers";


const {bookList,refreshFun} = defineProps<{bookList:BookModel[],refreshFun:Function}>();
const envStore = useEnvStore();
const textCountRef = ref<Map<string,number>>()
const dialogRef = ref<boolean>(false);

function getCoverSrc(coverId: string):string {
  if (coverId){
    return convertFileSrc(coverId);
  }
  return convertFileSrc(envStore.env.cover_dir_path+"/cover_dark.png");
}

function toChapterList(bookId: string, bookName: string) {
  routers.push({name: 'bookInfo', params: {bookId: bookId, bookName: bookName}})
}
onMounted(()=>{
  envStore.getEnv();
})
onUpdated(()=>{
  envStore.getEnv();
})
function deleteBook(bookId: string) {
  request("delete_book_by_id",{bookId:bookId}).then(res=>{
    refreshFun();
    dialogRef.value = false;
  })
}
function changDialog(status: boolean) {
  dialogRef.value = status;
}
</script>
<template>
  <div>
    <a-list-item  v-for="book in bookList" >
      <div class="book-item-content" @click="toChapterList(book.id,book.name)">
        <div class="book-cover">
          <a-image
              :src="getCoverSrc(book.cover_path)"
              width="75"
              height="100"
              fit="fill"
              style="border-radius: 5px"></a-image>
        </div>
        <div class="book-info-content">
          <div class="book-title">
            <span>{{book.name}}</span>
          </div>
          <div class="book-white-content"></div>

          <div class="book-desc-content">
            <label class="book-desc-item">总字数:
              <span v-text="book.text_count"></span>
            </label>
            <label class="book-desc-item">更新时间:
              <span>{{book.create_time}}</span>
            </label>
          </div>
        </div>
      </div>
      <template #extra>
        <a-popconfirm @ok="deleteBook(book.id)" content="确认删除本地所有书籍相关数据吗？">
          <a-button variant="text" >
            删除
          </a-button>
        </a-popconfirm>
        <a-back-top variant="text" @click="changDialog(true)">
          编辑
        </a-back-top>
      </template>
    </a-list-item>
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