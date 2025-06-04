import 'bootstrap/dist/css/bootstrap.css'

import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { loadConfig } from './configloader.ts'

loadConfig()
    .then(() => {
        const app = createApp(App)
        app.use(router)
        app.mount('#app')
    })
    .catch((err) => {
        console.error('Failed to load config:', err)
    })

import 'bootstrap/dist/js/bootstrap.js'
