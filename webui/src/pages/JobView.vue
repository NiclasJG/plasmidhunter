<template>
    <div>
        <h1>Single Job</h1>
        <IgvViewer :data="toRaw(resultData)" />
    </div>
</template>

<script setup>
import IgvViewer from '@/components/IgvViewer.vue'
import { ref, reactive, toRaw, onMounted } from 'vue'

const resultData = ref()

//fetch local file (./metagenome.. loads 'http://localhost:5173/job/metagenome_testset.json' )
await fetch('http://localhost:5173/metagenome_testset.json')
    .then((response) => response.json())
    .then((data) => (resultData.value = data))
    .catch((error) => console.log(error))

// console.log(resultData.value)
</script>
