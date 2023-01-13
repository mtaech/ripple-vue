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
    <n-page-header style="padding-left: 24px;">
      <template #title>
        <span style="font-size: 1.6rem;">我的作品</span>
      </template>
      <template #extra>
        <div class="book-header">
          <div class="book-header-right">
            <n-button @click="openModal">新建作品</n-button>
          </div>
        </div>
      </template>
    </n-page-header>
    <n-card class="default-content" :bordered="false">
      <n-list clickable >
        <book-item ref="bookItemRef" :book-list="bookListRef" :refresh-fun="getBookList"/>
      </n-list>
    </n-card>
    <div>
      <n-drawer v-model:show="editShowRef" :width="'40%'">
        <n-drawer-content title="新建小说">
          <book-edit :close-func="closeModal" :refresh-func="getBookList" :show-status="editShowRef" ref="bookEditRef"/>
          <template #footer>
            <n-button-group size="medium">
              <n-button type="tertiary">取消</n-button>
              <n-button type="primary" @click="saveBook">确定</n-button>
            </n-button-group>
          </template>
        </n-drawer-content>
      </n-drawer>
    </div>
  </div>
</template>

<script lang="ts">

export default {
  name: "Book",
}
</script>

<style scoped>
.book-content {
  width: 60vw;
  margin: 2vh auto;
}

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