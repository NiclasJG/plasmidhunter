<script setup>
import { RouterView } from 'vue-router'
import Navi from './components/Navi.vue'
import Footer from './components/Footer.vue'
</script>

<template>
    <Suspense>
        <div class="d-flex flex-column min-vh-100">
            <Navi />
            <RouterView />
            <Footer />
        </div>
    </Suspense>
</template>
