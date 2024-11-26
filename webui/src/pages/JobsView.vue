<template>
    <div>
        <h1>Jobs</h1>
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
                <tr v-for="item in jobs.jobs">
                    <th>{{ item.id }}</th>
                    <th>{{ item.name }}</th>
                    <th>{{ item.status }}</th>
                    <th v-if="item.status === 'Succeeded'">
                        <router-link :to="{ name: 'job', params: { id: item.id } }"> Results </router-link>
                    </th>
                    <th>{{ new Date(item.started).toLocaleString() }}</th>
                    <th>{{ new Date(item.updated).toLocaleString() }}</th>
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

let jobs: responseJobList

const loaded = ref(false)

onMounted(async () => {
    jobs = await getJobList()
    loaded.value = true
})

// console.log(jobs)
// function test() {
//     console.log(getJobs())
// }

// const jobs = [
//     {
//         jobID: 1,
//         jobName: 'test1',
//         jobKey: 1,
//     },
//     {
//         jobID: 2,
//         jobName: 'test2',
//         jobKey: 2,
//     },
//     {
//         jobID: 3,
//         jobName: 'test3',
//         jobKey: 3,
//     },
// ]
</script>
