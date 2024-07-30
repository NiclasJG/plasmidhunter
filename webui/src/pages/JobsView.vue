<template>
    <div>
        <h1>Jobs</h1>
        <table class="table">
            <thead>
                <tr>
                    <th>ID</th>
                    <th>Jobname</th>
                    <th>Link</th>
                </tr>
            </thead>
            <tbody>
                <tr v-for="item in jobs">
                    <th>{{ item.jobID }}</th>
                    <th>{{ item.jobName }}</th>
                    <th>
                        <router-link :to="{ name: 'job', params: { id: item.jobKey } }"> Link </router-link>
                    </th>
                </tr>
            </tbody>
        </table>
    </div>
</template>

<script setup>
const jobs = [
    {
        jobID: 1,
        jobName: 'test1',
        jobKey: 1,
    },
    {
        jobID: 2,
        jobName: 'test2',
        jobKey: 2,
    },
    {
        jobID: 3,
        jobName: 'test3',
        jobKey: 3,
    },
]
</script>
