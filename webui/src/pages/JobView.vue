<template>
    <div class="container fluid text-center">
        <div><h1>Single Job</h1></div>
        <div v-if="loaded">
            <div class="row" style="height: 400px">
                <div class="col">
                    <h3>Locations</h3>
                    <Map ref="MapRef" :data="fetchData" />
                </div>
                <div class="col">
                    <histogramcomp :data="fetchData" />
                </div>
            </div>
            <div class="row">
                <h3>Data Table</h3>
                <table class="table">
                    <thead>
                        <tr>
                            <!-- <th>ID</th> -->
                            <th>Accession</th>
                            <th>Location</th>
                            <th>Collection Date</th>
                            <th>Description</th>
                            <th>Sample Origin</th>
                            <th>BioSample</th>
                            <th>Mgnify</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr v-for="(item, index) in fetchData.hits">
                            <td>{{ item.accession }}</td>
                            <td>
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
                            </td>
                            <td>{{ item['collection-date'] }}</td>

                            <td>{{ item['sample-desc'] }}</td>
                            <!--       <td>{{ item.seqMethod }}</td> -->
                            <td>{{ item['environment-biome'] }}</td>
                            <td>
                                <a
                                    :href="'https://www.ebi.ac.uk/biosamples/samples/' + item.biosample"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    >BioSample</a
                                >
                            </td>
                            <td>
                                <a
                                    :href="'https://www.ebi.ac.uk/metagenomics/samples/' + item.accession"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    >MGnify</a
                                >
                            </td>
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
import histogramcomp from '@/components/HistogramComponent.vue'

import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { resultData } from '@/helpers/ResultInterface'
import { getJobResult } from '@/helpers/fetch_helper'
import { getSingleJob } from '@/helpers/storagehelper'

const MapRef = ref<InstanceType<typeof Map | null>>(null)
const route = useRoute()
const loaded = ref(false)

const fetchData = ref<resultData>()

// await fetch('http://localhost:5173/result.json')
//     .then((response) => response.json())
//     .then((data) => (fetchData.value = data))
//     .catch((error) => console.log(error))
// loaded.value = true

onMounted(async () => {
    let job = getSingleJob(route.params.id.toLocaleString())
    // console.log(job)
    fetchData.value = await getJobResult(job)
    loaded.value = true
    // console.log(fetchData.value)
})

// console.log(new Date('Oct-13-2010'))

// fetchData.value.hits.sort((a, b) => {
//     return Number(new Date(a['collection-date'])) - Number(new Date(b['collection-date']))
// })
</script>

<style></style>
