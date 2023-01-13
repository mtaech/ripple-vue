import {createApp} from 'vue'
import App from './App.vue';
import 'reset-css'
import {createPinia} from "pinia";
import router from "./Routers";
import './style.css'
import './assets/font/jxzk.ttf'

import 'vuetify/styles'
import {createVuetify} from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'

const vuetify = createVuetify({
    components,
    directives,
})

const pinia = createPinia()
const app = createApp(App);
app.use(pinia)
app.use(router)
app.use(vuetify)

app.mount('#app');
