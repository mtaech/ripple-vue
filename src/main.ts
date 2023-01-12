import {createApp} from 'vue'
import App from './App.vue';
import 'reset-css'
import {createPinia} from "pinia";
import  router from "./Routers";
import './style.css'
import  './assets/font/jxzk.ttf'
import 'vfonts/IBMPlexSans.css'

const pinia = createPinia()
const app = createApp(App);
app.use(pinia)


app.use(router)

app.mount('#app');
