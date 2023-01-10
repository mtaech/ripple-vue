<script setup lang="ts">
import {NButton, NCard, NList} from "naive-ui";
import {onMounted, ref} from "vue";
import {BookModel, Page} from "../../types/model";
import {request} from "../../api/request";
import BookEdit from "./BookEdit.vue";
import BookItem from "./BookItem.vue";
import routers from "../../Routers";


const bookPageRef = ref<Page<BookModel>>({pages: 0, page_no: 0, page_size: 10, datas: [], total: 0});
const editShowRef = ref<boolean>(false);
onMounted(() => {
  getBookList(1);
})

function getBookList(page: number) {
  let page_no = page ? page : 1;
  console.log("page num", page)
  request<Page<BookModel>>("get_book_list", {req: {page_no: page_no, page_size: 10}}).then((bookList) => {
    bookPageRef.value = bookList
  })
}

function openModal() {
  editShowRef.value = true;
}

function closeModal() {
  editShowRef.value = false;
}

const bookEditRef = ref<any>()

function saveBook() {
  bookEditRef.value.saveBook();
}

function toChapterList(bookId: string, bookName: string) {
  routers.push({name: 'bookInfo', params: {bookId: bookId, bookName: bookName}})
}

</script>
<template>
  <div>
    <n-card class="default-content">
      <template #header>
        <div>我的作品</div>
      </template>
      <template #header-extra>
        <div class="book-header">
          <div class="book-header-right">
            <n-button @click="openModal">新建作品</n-button>
          </div>
        </div>
      </template>
      <n-list clickable >
        <n-list-item v-if="bookPageRef.total> 0"
            :key="book.id" v-for="(book,index) in bookPageRef.datas"
                     @click="toChapterList(book.id,book.name)">
          <book-item :book="book"/>
          <template #suffix>
            <n-button text>
              编辑
            </n-button>
          </template>
        </n-list-item>
        <n-empty class="empty-list-content" size="large" v-else></n-empty>
      </n-list>
      <n-pagination
          style="padding-top: 24px;"
          :on-update:page="getBookList"
          :page="bookPageRef.page_no"
          :page-size="bookPageRef.page_size"
          :page-count="bookPageRef.pages"
          :default-page="0"
          :default-page-size="10"/>
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