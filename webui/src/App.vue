<script setup>
import { RouterView } from 'vue-router'
import Navi from './components/Navi.vue'
</script>

<template>
    <Suspense>
        <div>
            <Navi />
            <RouterView />
        </div>
    </Suspense>
</template>
