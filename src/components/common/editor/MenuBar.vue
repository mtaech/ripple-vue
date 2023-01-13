<script setup lang="ts">

import {Editor} from "@tiptap/vue-3";
import {Item} from '@/types/model'

const {editor} = defineProps<{editor:Editor}>();


const items:Item[] =  [
  {
    icon: 'type-bold',
    title: '粗体',
    action: () => {
      editor.chain().focus().toggleBold().run()
    },
    isActive: () => editor.isActive('bold'),
  },
  {
    icon: 'type-italic',
    title: '斜体',
    action: () => editor.chain().focus().toggleItalic().run(),
    isActive: () => editor.isActive('italic'),
  },
  {
    icon: 'type-strikethrough',
    title: '删除线',
    action: () => editor.chain().focus().toggleStrike().run(),
    isActive: () => editor.isActive('strike'),
  },
  {
    icon: 'pen',
    title: '高亮',
    action: () => editor.chain().focus().toggleHighlight().run(),
    isActive: () => editor.isActive('highlight'),
  },
  {
    type: 'divider',
  },
  {
    icon: 'type-h1',
    title: '一级标题',
    action: () => editor.chain().focus().toggleHeading({ level: 1 }).run(),
    isActive: () => editor.isActive('heading', { level: 1 }),
  },
  {
    icon: 'type-h2',
    title: '二级标题',
    action: () => editor.chain().focus().toggleHeading({ level: 2 }).run(),
    isActive: () => editor.isActive('heading', { level: 2 }),
  },
  {
    icon: 'text-paragraph',
    title: '段落',
    action: () => editor.chain().focus().setParagraph().run(),
    isActive: () => editor.isActive('paragraph'),
  },
  {
    icon: 'list-ul',
    title: '无序列表',
    action: () => editor.chain().focus().toggleBulletList().run(),
    isActive: () => editor.isActive('bulletList'),
  },
  {
    icon: 'list-ol',
    title: '有序列表',
    action: () => editor.chain().focus().toggleOrderedList().run(),
    isActive: () => editor.isActive('orderedList'),
  },
  {
    icon: 'check2-square',
    title: '待办列表',
    action: () => editor.chain().focus().toggleTaskList().run(),
    isActive: () => editor.isActive('taskList'),
  },
  {
    type: 'divider',
  },
  {
    icon: 'quote',
    title: '引用',
    action: () => editor.chain().focus().toggleBlockquote().run(),
    isActive: () => editor.isActive('blockquote'),
  },
  {
    icon: 'hr',
    title: '分割线',
    action: () => editor.chain().focus().setHorizontalRule().run(),
  },
  {
    type: 'divider',
  },
  {
    icon: 'text-wrap',
    title: '换行',
    action: () => editor.chain().focus().setHardBreak().run(),
  },
  {
    icon: 'eraser',
    title: '清除格式',
    action: () => editor.chain()
        .focus()
        .clearNodes()
        .unsetAllMarks()
        .run(),
  },
  {
    type: 'divider',
  },
  {
    icon: 'arrow-counterclockwise',
    title: '撤销',
    action: () => editor.chain().focus().undo().run(),
  },
  {
    icon: 'arrow-clockwise',
    title: '重做',
    action: () => editor.chain().focus().redo().run(),
  }
];
</script>

<template>
  <div>
    <template v-for="(item, index) in items">
      <div class="divider" v-if="item.type === 'divider'" :key="`divider${index}`" />
      <menu-item v-else :key="index"
                 :action="item.action" :icon="item.icon" :title="item.title"
                 :is-active="item.isActive"/>
    </template>
  </div>
</template>

<script lang="ts">
import '@icon-park/vue-next/styles/index.css';
import MenuItem from './MenuItem.vue'
import {Home} from '@icon-park/vue-next';
import { defineComponent } from "vue";


export default defineComponent({
  components: {
    MenuItem,
    Home
  }
})
</script>

<style >
.divider {
  width: 2px;
  height: 1.25rem;
  background-color: rgba(0, 0, 0, 0.1);
  margin-left: 0.5rem;
  margin-right: 0.75rem;
}
</style>