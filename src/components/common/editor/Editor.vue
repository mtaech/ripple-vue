<template>
  <div class="editor" v-if="editor">
    <menu-bar class="editor_header" :editor="editor" />
    <editor-content class="editor_content" :editor="editor" />
    <editor-footer :editor="editor"/>
  </div>
</template>
<script setup>
import {useEditor} from "@tiptap/vue-3";
import {StarterKit} from "@tiptap/starter-kit";
import MenuBar from "./MenuBar.vue";
import {Highlight} from "@tiptap/extension-highlight";
import {TaskList} from "@tiptap/extension-task-list";
import {TaskItem} from "@tiptap/extension-task-item";
import {CharacterCount} from "@tiptap/extension-character-count";
import EditorFooter from "./EditorFooter.vue";

const editor = useEditor({
  extensions: [
    StarterKit.configure({
      history: false,
    }),
    Highlight,
    TaskList,
    TaskItem,
    History,
    CharacterCount.configure({
      limit: 100000,
    }),
  ],
})
</script>
<script>
import {EditorContent} from "@tiptap/vue-3";
import {StarterKit} from "@tiptap/starter-kit";
import {CharacterCount} from "@tiptap/extension-character-count";
import {Highlight} from "@tiptap/extension-highlight";
import {TaskList} from "@tiptap/extension-task-list";
import {TaskItem} from "@tiptap/extension-task-item";
export default {
  name: "Editor",
  components: {EditorContent},
  data() {
    return {
      editor: null,
    }
  },
  beforeUnmount() {
    this.editor.destroy()
  },
}

</script>


<style lang="scss">
.editor {
  display: flex;
  flex-direction: column;
  max-height: 500vh;
  height: 80vh;
  width:60%;
  margin: 50px auto;
  color: #0d0d0d;
  background-color: #fff;
  border: 3px solid #0d0d0d;
  border-radius: 0.75rem;
  font-size: 1.5rem;
  /* Some information about the status */
}
.editor_header {
  display: flex;
  align-items: center;
  flex: 0 0 auto;
  flex-wrap: wrap;
  padding: 0.25rem;
  border-bottom: 3px solid #0d0d0d;
}
.editor_content {
  padding: 0.5rem 0.5rem;
  flex: 1 1 auto;
  overflow-x: hidden;
  overflow-y: auto;
  -webkit-overflow-scrolling: touch;
}

.ProseMirror {
  height: 95%;
  padding: 1rem 1.25rem;
}
.ProseMirror:focus{
  //outline: none;
}
.ProseMirror ul, .ProseMirror ol {
  padding: 0 1rem;
}
.ProseMirror h1, .ProseMirror h2, .ProseMirror h3, .ProseMirror h4, .ProseMirror h5, .ProseMirror h6 {
  line-height: 1.1;
}
.ProseMirror code {
  background-color: rgba(97, 97, 97, 0.1);
  color: #616161;
}
.ProseMirror pre {
  background: #0d0d0d;
  color: #fff;
  font-family: 'JetBrainsMono', monospace;
  padding: 0.75rem 1rem;
  border-radius: 0.5rem;
}
.ProseMirror pre code {
  color: inherit;
  padding: 0;
  background: none;
  font-size: 0.8rem;
}
.ProseMirror mark {
  background-color: #faf594;
}
.ProseMirror img {
  max-width: 100%;
  height: auto;
}
.ProseMirror hr {
  margin: 1rem 0;
}
.ProseMirror blockquote {
  padding-left: 1rem;
  border-left: 2px solid rgba(13, 13, 13, 0.1);
}
.ProseMirror hr {
  border: none;
  border-top: 2px solid rgba(13, 13, 13, 0.1);
  margin: 2rem 0;
}
.ProseMirror ul {
  margin-top: 10px;
  margin-bottom: 10px;
}
.ProseMirror ul[data-type="taskList"] {
  list-style: none;
  padding: 0;
}
.ProseMirror ul[data-type="taskList"] li {
  display: flex;
  align-items: center;
}
.ProseMirror ul[data-type="taskList"] li > label {
  flex: 0 0 auto;
  margin-right: 0.5rem;
  user-select: none;
}
.ProseMirror ul[data-type="taskList"] li > div {
  flex: 1 1 auto;
}
.ProseMirror  p {
  margin-top: 5px;
  margin-bottom: 5px;
}
.ProseMirror > p {
  text-indent: 2em;
}

.ProseMirror ol,.ProseMirror ul {
  padding: 0 2rem;
}
</style>