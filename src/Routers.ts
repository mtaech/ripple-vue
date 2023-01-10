import {createRouter, createWebHashHistory} from 'vue-router'

const routes = [
    {
        path: '/',
        name: 'home',
        component: () => import('./components/book/Index.vue')
    }, {
        path: '/book',
        name: 'bookIndex',
        component: () => import('./components/book/Index.vue')
    }, {
        path: '/book/:bookId?bookName=:bookName',
        name: 'bookInfo',
        component: () => import('./components/chapter/Index.vue'),
    },
    {
        path: "/chapter",
        name: "chapterIndex",
        component: () => import('./components/chapter/Index.vue'),
    }, {
        path: "/chapter/:chapterId?bookId=:bookId",
        name: "chapterEdit",
        component: () => import('./components/chapter/ChapterEdit.vue')
    }
]

export default createRouter({
    history: createWebHashHistory(),
    routes
})