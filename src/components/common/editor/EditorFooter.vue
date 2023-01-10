<template>
  <div class="editor_footer">
    <span>
      字数统计：{{editor.storage.characterCount.characters()}}
    </span>
  </div>
</template>

<script lang="ts">
export default {
  name: "EditorFooter",
  props: {
    editor: {
      type: Object,
      required: true,
    },
  },
}
</script>

<style scoped>
.editor_footer {
  display: flex;
  flex: 0 0 auto;
  align-items: center;
  justify-content: space-between;
  flex-wrap: wrap;
  white-space: nowrap;
  font-size: 0.8rem;
  font-weight: 400;
  color: #383737;
  padding: 0.5rem 0.75rem;
}
</style>