<template>
    <div class="container fluid text-center">
        <div><h1>Single Job</h1></div>
        <div v-if="loaded">
            <div class="row" style="height: 30vh">
                <div class="col">
                    <h3>Locations</h3>
                    <Map ref="MapRef" :data="fetchData" />
                </div>
                <div class="col">
                    <h3>Timeline</h3>
                    <TimelineComponent :dateList="fetchData" />
                </div>
                <!-- <div class="col">
                    <h3>Timeline</h3>
                    <NewTimelineComponent :data="fetchData" />
                </div> -->
            </div>
            <div class="row">
                <h3>Data Table</h3>
                <table class="table">
                    <thead>
                        <tr>
                            <!-- <th>ID</th> -->
                            <th>Name</th>
                            <th>Location</th>
                            <th>Method</th>
                            <th>Sample Origin</th>
                            <th>Collection Date</th>
                            <th>BioSample</th>
                            <th>Mgnify</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr v-for="(item, index) in fetchData.hits">
                            <th>{{ item['sample-alias'] }}</th>
                            <th>
                                <button
                                    @click="
                                        MapRef.centerMarker(
                                            fetchData.hits[index].latitude,
                                            fetchData.hits[index].longitude,
                                        )
                                    "
                                >
                                    {{ item['geo-loc-name'] }}
                                    <!-- sample-metadata: "geographic location (country and/or sea,region)" -->
                                </button>
                            </th>
                            <!--     <th>{{ item.seqMethod }}</th> -->
                            <th>{{ item['environment-biome'] }}</th>
                            <th>{{ item['collection-date'] }}</th>
                            <th>
                                <a
                                    :href="'https://www.ebi.ac.uk/biosamples/samples/' + item.biosample"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    >BioSample</a
                                >
                            </th>
                            <th>
                                <a
                                    :href="'https://www.ebi.ac.uk/metagenomics/samples/' + item.accession"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    >MGnify</a
                                >
                            </th>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="row">
                <h3>Plasmid Viewer</h3>
                <IgvViewer :data="fetchData" />
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import IgvViewer from '@/components/IgvViewer.vue'
import Map from '@/components/MapComponent.vue'
import TimelineComponent from '@/components/TimelineComponent.vue'
import NewTimelineComponent from '@/components/NewTimelineComponent.vue'

import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { resultData } from '@/helpers/ResultInterface'
import { getJobResult } from '@/helpers/fetch_helper'
import { getSingleJob } from '@/helpers/storagehelper'

const MapRef = ref<InstanceType<typeof Map | null>>(null)
const route = useRoute()
const loaded = ref(false)

const fetchData = ref<resultData>()

// let fetchData: resultData
// fetch local file (./metagenome.. loads 'http://localhost:5173/job/metagenome_testset.json' )
// await fetch('http://localhost:5173/result.json')
//     .then((response) => response.json())
//     .then((data) => (fetchData.value = data))
//     .catch((error) => console.log(error))
// // })
// loaded.value = true
// beforeRouteEnter (to, from, next) {

onMounted(async () => {
    let job = getSingleJob(route.params.id.toLocaleString())
    console.log(job)
    fetchData.value = await getJobResult(job)
    loaded.value = true
    console.log(fetchData.value)
})

// fetchData.value.hits.sort((a, b) => {
//     return Number(new Date(a['collection-date'])) - Number(new Date(b['collection-date']))
// })
// //
</script>

<style></style>
