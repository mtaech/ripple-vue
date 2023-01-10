<script lang="ts" setup>
import {ref} from "vue";
import {BookModel} from "../../types/model";
import {nanoid} from "nanoid";
import {FormInst} from "naive-ui";
import {request} from "../../api/request";

const formRef = ref<FormInst | null>()
let book:BookModel =  {id:nanoid(10),name:'',description:'',create_time:'',modified_time:''};
let bookValueRef = ref<BookModel>(book)
const props = defineProps<{ refreshFunc: Function,showStatus:Boolean,closeFunc:Function }>();
const saveBook = () => {
  console.log(formRef.value)
  formRef.value?.validate((errors)=>{
    if (errors){
      console.log("meifatijao")
    }else {
      console.log("save book")
      request<BookModel>("save_book", {book: bookValueRef.value}).then((book) => {
        console.log("book",book)
        props.refreshFunc();
        props.closeFunc();
      })
    }
  })
  console.log("ednd")
}

defineExpose({saveBook})

const rules = {
  name:{
    required:true,
    trigger: 'blur',
  }
}

</script>

<template>
    <n-form ref="formRef" :model="bookValueRef" :rules="rules">
      <n-form-item label="书名" path="name" >
        <n-input v-model:value="bookValueRef.id" style="display: none"/>
        <n-input v-model:value="bookValueRef.name" placeholder="输入书名" />
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