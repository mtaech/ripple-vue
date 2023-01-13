<script lang="ts" setup>
import {ref} from "vue";
import {BookModel} from "../../types/model";
import {nanoid} from "nanoid";
import {request} from "../../api/request";
import {dialog, path} from "@tauri-apps/api";
import {convertFileSrc} from "@tauri-apps/api/tauri";
import {useEnvStore} from "../../Store";

const props = defineProps<{ refreshFunc: Function, showStatus: Boolean, closeFunc: Function }>();
const formRef = ref()
let book: BookModel = {
  id: nanoid(10),
  name: '',
  description: '',
  create_time: '',
  modified_time: '',
  cover_id: '',
  cover_path: '',
  text_count: 0
};
let bookValueRef = ref<BookModel>(book)

const envStore = useEnvStore();

onBeforeMount(()=>{
  path.configDir().then(configPath=>{
    path.join(configPath,"ripple","cover","cover_dark.png").then(cover_path=>{
      coverRef.value = cover_path;
    })
  })
  envStore.getEnv()
})
const coverRef = ref<string>("");

const saveBook = () => {
  console.log(formRef.value)
  formRef.value?.validate((errors: any) => {
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


  <a-form ref="formRef" :model="bookValueRef" layout="vertical" :rules="rules">
   <a-form-item label="封面图片" >
     <div style="margin-bottom: 25px;display: flex;flex-direction: row">
       <a-image :src="convertSrc(coverRef)" width="150" height="200" style="border-radius: 5px;margin-bottom: -5px;"/>
       <a-button @click="uploadCover" style="margin-left: 10px;align-self: flex-end">上传封面</a-button>
     </div>
   </a-form-item>
    <a-form-item label="书名" field="name" >
      <a-input v-model="bookValueRef.id" style="display: none"/>
      <a-input v-model="bookValueRef.name" size="large" placeholder="输入书名"/>
    </a-form-item>
    <a-form-item label="简介" field="description" >
      <a-textarea :allow-clear="true" style="height: 300px" v-model="bookValueRef.description" placeholder="输入简介"/>
    </a-form-item>
  </a-form>
</template>

<script lang="ts">
export default {
  name: "BookEdit",
}
</script>

<style scoped>

</style>