<script setup lang="ts">
import {NButton, NCard, NList} from "naive-ui";
import {onMounted, ref} from "vue";
import {BookModel, Page} from "../../types/model";
import {request} from "../../api/request";
import BookEdit from "./BookEdit.vue";
import BookItem from "./BookItem.vue";


const bookListRef = ref<BookModel[]>([] as BookModel[]);
const editShowRef = ref<boolean>(false);
const bookEditRef = ref()
const bookItemRef = ref()
onMounted(() => {
  getBookList();
})

function getBookList() {
  request<BookModel[]>("get_book_list").then((bookList) => {
    console.log("book list",bookList)
    bookListRef.value = bookList
  })
}

function openModal() {
  editShowRef.value = true;
}

function closeModal() {
  editShowRef.value = false;
}

function saveBook() {
  bookEditRef.value.saveBook();
}



</script>
<template>
  <div class="default-content">
    <a-page-header :show-back="false">
      <template #title>
        <span style="font-size: 1.6rem;">&nbsp;作品管理</span>
      </template>
      <template #extra>
            <a-button style="margin-right: 16px;" @click="openModal">新建作品</a-button>
      </template>
    </a-page-header>
    <a-card :bordered="false" >
      <a-list :bordered="false" style="padding: 0 16px" >
        <book-item  ref="bookItemRef" :book-list="bookListRef" :refresh-fun="getBookList"/>
      </a-list>
    </a-card>
    <div>
      <a-drawer :visible="editShowRef" :width="'30%'"  title="新建小说" @ok="saveBook" @cancel="closeModal">
          <book-edit :close-func="closeModal" :refresh-func="getBookList" :show-status="editShowRef" ref="bookEditRef"/>
      </a-drawer>
    </div>
  </div>
</template>

<script lang="ts">

export default {
  name: "Book",
}
</script>

<style scoped>
.book-header {
  display: flex;
  font-weight: bold;
  line-height: 2rem;
  font-size: 1.2rem;
  align-items: center;
}

.book-header-right {
  display: flex;
  justify-content: flex-end;
  margin-left: auto;
  cursor: pointer;
}
</style>