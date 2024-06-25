import { createRouter, createWebHistory } from "vue-router";
import HomeView from "./pages/HomeView.vue";
import AboutView from "./pages/AboutView.vue";

const routes = [
    {
        path: "/",
        name: "home",
        component: HomeView
    },
    {
        path: "/about",
        name: "about",
        component: AboutView
    }
]

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes
})

export default router