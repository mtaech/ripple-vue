<script setup lang="ts">


import {Chapter, EditorInfo} from "../../types/model";
import {getRouteStingParam, request} from "../../api/request";
import {useRoute} from "vue-router";
import {onBeforeUnmount} from "vue";
import routers from "../../Routers";

let route = useRoute();
let params = route.params;

let editorRef = ref();
const chapterRef = ref<Chapter>({} as Chapter);
const chapterNameRef = ref('');
const message = useMessage()

onMounted(() => {
  console.log("param",params)
  let chapterId = getRouteStingParam(params,"chapterId");
  let bookId = getRouteStingParam(params,"bookId");
  get_chapter(chapterId,bookId);
  addSaveListener();
})
onBeforeUnmount(() => {
  removeSaveListener();
})


function get_chapter(id: string,bookId:string) {
  request<Chapter>("find_chapter_by_id", {chapterId: id,bookId:bookId}).then((chapter) => {
    if (chapter) {
      chapterRef.value = chapter;
      chapterNameRef.value = chapter.name;
      editorRef.value.setEditorContent(chapter.html_content);
    } else {
      chapterRef.value.id = id;
      chapterRef.value.book_id = bookId;
    }
  })
}

function saveChapter() {
  console.log("get start"+new Date().toString())
  let info: EditorInfo = editorRef.value.getEditorInfo();
  console.log("get end"+new Date().toString())
  console.log("info", info)
  chapterRef.value.name = chapterNameRef.value;
  chapterRef.value.html_content = info.html;
  chapterRef.value.text_content = info.text;
  chapterRef.value.text_count = info.text_count;
  request<Chapter>("save_chapter", {chapter: chapterRef.value}).then((chapter) => {
    console.log('save_chapter',chapter)
    message.success("保存成功")
  })
}

function addSaveListener() {
  document.addEventListener("keydown", enterKey)
}

function removeSaveListener() {
  document.removeEventListener("keydown", enterKey)
}

function enterKey(event: KeyboardEvent) {
  if (event.ctrlKey && event.key === 's') {
    saveChapter();
  }
}
</script>

<template>
  <div class="default-content">
    <n-page-header title="新建"  @back="routers.back();" style="margin-bottom: 20px;">
      <template #extra>
        <n-button @click="saveChapter">
          保存
        </n-button>
      </template>
    </n-page-header>
    <n-input v-model:value="chapterNameRef" size="small" :bordered="false" class="chapter-name"
             placeholder="请输入章节号和章节名。例如 第一章：灵根育孕源流出　心性修持大道生"
    />
    <p class="dividing-line"></p>
    <editor ref="editorRef" />
  </div>
</template>

<script lang="ts">
import Editor from "../common/editor/Index.vue";

export default {
  name: "ChapterEdit",
  components: {Editor}
}
</script>

<style scoped>
.chapter-name{
  line-height: 1.2rem !important;
}
.dividing-line{
  margin-bottom: 5px;
  border-bottom: 2px solid #d8dad9;
}
</style>