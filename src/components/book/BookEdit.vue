<script lang="ts" setup>
import {ref} from "vue";
import {BookModel} from "../../types/model";
import {nanoid} from "nanoid";
import {request} from "../../api/request";
import {FormInst, useMessage} from 'naive-ui'
import {dialog} from "@tauri-apps/api";
import {convertFileSrc} from "@tauri-apps/api/tauri";
import {useEnvStore} from "../../Store";

const props = defineProps<{ refreshFunc: Function, showStatus: Boolean, closeFunc: Function }>();
const message = useMessage()
const formRef = ref<FormInst | null>()
let book: BookModel = {
  id: nanoid(10),
  name: '',
  description: '',
  create_time: '',
  modified_time: '',
  cover_id: '',
  cover_path: ''
};
let bookValueRef = ref<BookModel>(book)

const envStore = useEnvStore();

let src = envStore.env.cover_dir_path + "/cover_dark.png";
console.log("src", src)
const coverRef = ref<string>(src);

const saveBook = () => {
  console.log(formRef.value)
  formRef.value?.validate((errors) => {
    if (errors) {
      console.log("meifatijao")
    } else {
      console.log("save book")
      request<BookModel>("save_book", {book: bookValueRef.value,coverUrl:coverRef.value}).then((book) => {
        console.log("book", book)
        props.refreshFunc();
        props.closeFunc();
      })
    }
  })
  console.log("ednd")
}

function uploadCover() {
  dialog.open({
    multiple: false,
    filters: [{
      name: 'Image',
      extensions: ['png', 'jpeg', "jpg"]
    }]
  }).then(filePath => {
    if (filePath !== null && typeof filePath === 'string') {
      coverRef.value = filePath;
      console.log("open file", filePath)
    }
  })
}

function convertSrc(path:string) {
  return convertFileSrc(path)
}

defineExpose({saveBook})

const rules = {
  name: {
    required: true,
    trigger: 'blur',
  }
}

</script>

<template>
  <div style="margin-bottom: 25px;">
    <n-image :src="convertSrc(coverRef)" width="150" height="200" style="border-radius: 5px;margin-bottom: -5px;"/>
    <n-button @click="uploadCover" style="margin-left: 10px;bottom: 15px;">上传封面</n-button>
  </div>

  <n-form ref="formRef" :model="bookValueRef" :rules="rules">
    <n-form-item label="书名" path="name">
      <n-input v-model:value="bookValueRef.id" style="display: none"/>
      <n-input v-model:value="bookValueRef.name" placeholder="输入书名"/>
    </n-form-item>
    <n-form-item label="简介" path="description">
      <n-input rows="15" type="textarea" v-model:value="bookValueRef.description" placeholder="输入简介"/>
    </n-form-item>
  </n-form>
</template>

<script lang="ts">
export default {
  name: "BookEdit",
}
</script>

<style scoped>

</style>