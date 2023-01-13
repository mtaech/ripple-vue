import {createApp} from 'vue'
import App from './App.vue';
import 'reset-css'
import {createPinia} from "pinia";
import router from "./Routers";
import './style.css'
import './assets/font/jxzk.ttf'
import ArcoVue from '@arco-design/web-vue';
import ArcoVueIcon from '@arco-design/web-vue/es/icon';
import '@arco-design/web-vue/dist/arco.css';


const pinia = createPinia()
const app = createApp(App);
app.use(pinia)
app.use(router)
app.use(ArcoVue);
app.use(ArcoVueIcon);

app.mount('#app');
