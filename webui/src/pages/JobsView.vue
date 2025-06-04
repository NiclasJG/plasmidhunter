<template>
    <div class="container-fluid text-center">
        <div><h1>Jobs</h1></div>
        <table class="table" v-if="loaded">
            <thead>
                <tr>
                    <th>ID</th>
                    <th>Jobname</th>
                    <th>Status</th>
                    <th>Link</th>
                    <th>Started</th>
                    <th>Updated</th>
                </tr>
            </thead>
            <tbody>
                <tr v-for="item in jobs.jobs.slice().reverse()">
                    <td>{{ item.id }}</td>
                    <td>{{ item.name }}</td>
                    <td>{{ item.status }}</td>
                    <td v-if="item.status === 'Succeeded'">
                        <router-link :to="{ name: 'job', params: { id: item.id } }"> Results </router-link>
                    </td>
                    <td v-else>None</td>
                    <td>{{ new Date(item.started).toLocaleString() }}</td>
                    <td>{{ new Date(item.updated).toLocaleString() }}</td>
                </tr>
            </tbody>
        </table>
    </div>
    <!-- <div><button v-on:click="test"></button></div> -->
</template>

<script setup lang="ts">
import { getJobs } from '@/helpers/storagehelper'

import { onMounted, ref } from 'vue'
import { getJobList } from '@/helpers/fetch_helper'
import { responseJobList } from '@/helpers/Interface'

// let jobs: responseJobList

const jobs = ref<responseJobList>()

const loaded = ref(false)

onMounted(async () => {
    initJobList()
    updateJobList()
})

// callJobList()
async function initJobList() {
    jobs.value = await getJobList()
    loaded.value = true
}

function updateJobList() {
    setInterval(async () => {
        jobs.value = await getJobList()
        loaded.value = true
    }, 20000)
}
</script>
