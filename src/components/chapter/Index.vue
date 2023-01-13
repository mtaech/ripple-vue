<script lang="ts" setup>
import {getRouteStingParam, request} from "../../api/request";
import {Chapter} from "../../types/model";
import {ref} from "vue";
import routers from "../../Routers";
import {nanoid} from "nanoid";
import {useRoute} from "vue-router";

const chapterListRef = ref<Chapter[]>([]);
const route = useRoute();
const bookNameRef = ref("");
function getChapterList( book_id: string) {
  console.log("book_id",book_id)
  request<Chapter[]>("get_chapter_list", {
    bookId: book_id
  }).then((chapterList) => {
    console.log("chapter list",chapterList)
    chapterListRef.value = chapterList;
  })
}

function create_chapter() {
  let id = nanoid(10);
  let bookId = getRouteStingParam(route.params,'bookId');
  routers.push({name:"chapterEdit",params:{chapterId:id,bookId:bookId}});
}

function edit_chapter(chapterId:string) {
  let bookId = getRouteStingParam(route.params,'bookId');
  routers.push({name:"chapterEdit",params:{chapterId:chapterId,bookId:bookId}});
}
function back() {
  routers.back();
}

onMounted(() => {
  let params = route.params;
  let bookId = getRouteStingParam(params,'bookId');
  bookNameRef.value = getRouteStingParam(params, 'bookName');
  getChapterList(bookId);
})
</script>
<template>
  <div class="default-content">
    <a-page-header :title="bookNameRef" subtitle="章节列表" @back="back">
      <template #extra>
        <a-button @click="create_chapter"
                  style="margin-right: 24px;"
                  size="small">新建章节</a-button>
      </template>
    </a-page-header>
    <a-card class="default-content" style="min-height: 40vh" :bordered="false">
      <a-list class="chapter-list" :bordered="false" style="padding: 0 16px"  :show-divider="false">
        <a-list-item v-for="(chapter,index) in chapterListRef" v-if="chapterListRef.length > 0">
          <div class="chapter-content" @click="edit_chapter(chapter.id)">
            <span class="chapter-name">
              {{ chapter.name }}
            </span>
            <span class="gap-line"/>
            <span class="chapter-time">
              {{ chapter.create_time }}
            </span>
          </div>
        </a-list-item>
      </a-list>
    </a-card>
  </div>
</template>

<script lang="ts">
export default {
  name: "Chapter"
}
</script>

<style scoped>
.chapter-content {
  display: flex;
  flex-direction: row;
}
.chapter-list{
  display: flex;
  flex-direction: column;
}
.chapter-name {
  font-size: 1.1rem;
}
.chapter-index {
  font-size: 1.1rem;
  margin-right: 5px;
}
.chapter-time {
  font-size: 0.9rem;
  color: darkgray;
  align-self: center;
}
.gap-line{
  flex: 1;
  margin: auto 16px;
  border-top: 1px dashed #d8dad9;
  height: 1px;
}
</style>