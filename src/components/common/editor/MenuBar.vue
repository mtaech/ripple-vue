<template>
  <div>
    <template v-for="(item, index) in items">
      <div class="divider" v-if="item.type === 'divider'" :key="`divider${index}`" />
      <menu-item v-else :key="index" v-bind="item" />
    </template>
  </div>
</template>

<script>
import '@icon-park/vue-next/styles/index.css';
import MenuItem from './MenuItem.vue'
import {invoke} from "@tauri-apps/api";
import {nanoid} from "nanoid";
import {Home} from '@icon-park/vue-next';
export default {
  components: {
    MenuItem,
    Home
  },

  props: {
    editor: {
      type: Object,
      required: true,
    },
  },

  data() {
    return {
      items: [
        {
          icon: 'type-bold',
          title: '粗体',
          action: () => this.editor.chain().focus().toggleBold().run(),
          isActive: () => this.editor.isActive('bold'),
        },
        {
          icon: 'type-italic',
          title: '斜体',
          action: () => this.editor.chain().focus().toggleItalic().run(),
          isActive: () => this.editor.isActive('italic'),
        },
        {
          icon: 'type-strikethrough',
          title: '删除线',
          action: () => this.editor.chain().focus().toggleStrike().run(),
          isActive: () => this.editor.isActive('strike'),
        },
        {
          icon: 'code-slash',
          title: '代码',
          action: () => this.editor.chain().focus().toggleCode().run(),
          isActive: () => this.editor.isActive('code'),
        },
        {
          icon: 'pen',
          title: '高亮',
          action: () => this.editor.chain().focus().toggleHighlight().run(),
          isActive: () => this.editor.isActive('highlight'),
        },
        {
          type: 'divider',
        },
        {
          icon: 'type-h1',
          title: '一级标题',
          action: () => this.editor.chain().focus().toggleHeading({ level: 1 }).run(),
          isActive: () => this.editor.isActive('heading', { level: 1 }),
        },
        {
          icon: 'type-h1',
          title: '二级标题',
          action: () => this.editor.chain().focus().toggleHeading({ level: 2 }).run(),
          isActive: () => this.editor.isActive('heading', { level: 2 }),
        },
        {
          icon: 'text-paragraph',
          title: '段落',
          action: () => this.editor.chain().focus().setParagraph().run(),
          isActive: () => this.editor.isActive('paragraph'),
        },
        {
          icon: 'list-ul',
          title: '无序列表',
          action: () => this.editor.chain().focus().toggleBulletList().run(),
          isActive: () => this.editor.isActive('bulletList'),
        },
        {
          icon: 'list-ol',
          title: '有序列表',
          action: () => this.editor.chain().focus().toggleOrderedList().run(),
          isActive: () => this.editor.isActive('orderedList'),
        },
        {
          icon: 'check2-square',
          title: '待办列表',
          action: () => this.editor.chain().focus().toggleTaskList().run(),
          isActive: () => this.editor.isActive('taskList'),
        },
        {
          icon: 'code-square',
          title: '代码块',
          action: () => this.editor.chain().focus().toggleCodeBlock().run(),
          isActive: () => this.editor.isActive('codeBlock'),
        },
        {
          type: 'divider',
        },
        {
          icon: 'quote',
          title: '引用',
          action: () => this.editor.chain().focus().toggleBlockquote().run(),
          isActive: () => this.editor.isActive('blockquote'),
        },
        {
          icon: 'hr',
          title: '分割线',
          action: () => this.editor.chain().focus().setHorizontalRule().run(),
        },
        {
          type: 'divider',
        },
        {
          icon: 'text-wrap',
          title: '换行',
          action: () => this.editor.chain().focus().setHardBreak().run(),
        },
        {
          icon: 'eraser',
          title: '清除格式',
          action: () => this.editor.chain()
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
          action: () => this.editor.chain().focus().undo().run(),
        },
        {
          icon: 'arrow-clockwise',
          title: '重做',
          action: () => this.editor.chain().focus().redo().run(),
        }
      ],
    }
  },
  methods:{
    saveContent(){
      let html = this.editor.getHTML();
      let chapter = {
        id:nanoid(10),
        content:html
      }
      console.log("chapter",chapter)
      invoke("save_chapter",{chapter:chapter}).then((message)=>{
        console.log("msg",message)
      })
    }
  }
}
</script>

<style lang="scss">
.divider {
  width: 2px;
  height: 1.25rem;
  background-color: rgba(#000, 0.1);
  margin-left: 0.5rem;
  margin-right: 0.75rem;
}
</style>