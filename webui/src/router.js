import { createRouter, createWebHistory } from 'vue-router'
import HomeView from './pages/HomeView.vue'
import AboutView from './pages/AboutView.vue'
import JobsView from './pages/JobsView.vue'
import JobView from './pages/JobView.vue'

import NotFound from './pages/NotFound.vue'

const routes = [
    {
        path: '/',
        name: 'home',
        component: HomeView,
    },
    {
        path: '/jobs',
        name: 'jobs',
        component: JobsView,
    },
    {
        path: '/job/:id',
        name: 'job',
        component: JobView,
    },
    {
        path: '/about',
        name: 'about',
        component: AboutView,
    },
    {
        path: '/:catchAll(.*)*',
        component: NotFound,
    },
]

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes,
})

export default router
