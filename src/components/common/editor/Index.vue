<script setup lang="ts">
import {EditorContent, useEditor} from "@tiptap/vue-3";
import {StarterKit} from "@tiptap/starter-kit";
import {Highlight} from "@tiptap/extension-highlight";
import {TaskList} from "@tiptap/extension-task-list";
import {TaskItem} from "@tiptap/extension-task-item";
import {CharacterCount} from "@tiptap/extension-character-count";
import MenuBar from "./MenuBar.vue";
import EditorFooter from "./EditorFooter.vue";
import "../../../assets/css/editor.css"


const editor = useEditor({
  autofocus: "start",
  extensions: [
    StarterKit,
    Highlight,
    TaskList,
    TaskItem,
    CharacterCount.configure({
      limit: null,
    }),
  ],
});
const getEditorInfo = () => {
  console.log("start inner get ")
  let html = editor.value?.getHTML();
  console.log("get html end")
  let text = editor.value?.getText();
  let textCount: number = editor.value?.storage.characterCount.characters();
  let wordCount = editor.value?.storage.characterCount.words();
  let info = {html: html, text: text, text_count: textCount, word_count: wordCount}
  console.log("info", info)
  return info;
}

const setEditorContent = (content: string) => {
  editor.value?.commands.setContent(content)
}
defineExpose({getEditorInfo, setEditorContent})
</script>
<template>
  <div class="editor" v-if="editor">
    <menu-bar class="editor_header" :editor="editor"/>
    <editor-content class="editor_content" :editor="editor"/>
    <editor-footer :editor="editor"/>
  </div>
</template>

<script lang="ts">
import {defineComponent} from "vue";

export default defineComponent({
  name: "Editor",
})
</script>
