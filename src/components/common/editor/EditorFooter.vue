<template>
  <div class="editor_footer">
    <span>
      字数统计：{{editor.storage.characterCount.characters()}}
    </span>
  </div>
</template>

<script>
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
  border-top: 3px solid #0d0d0d;
  font-size: 1rem;
  font-weight: 600;
  color: #0d0d0d;
  padding: 0.5rem 0.75rem;
}
</style>