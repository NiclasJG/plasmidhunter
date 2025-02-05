<template>
    <div class="container-fluid text-center">
        <div v-if="loaded">
            <div>
                <h2>{{ fetchData.annotation.sequences[0].orig_description }}</h2>
            </div>
            <div class="row" style="height: 400px">
                <div class="col" style="width: 50%">
                    <h3>Locations</h3>
                    <Map ref="MapRef" :data="fetchData" />
                </div>
                <div class="col" style="width: 50%">
                    <h3>Histogram</h3>
                    <histogramcomp :data="fetchData" />
                </div>
            </div>
            <div class="row">
                <table class="table gy-5">
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
                            <td>{{ item.metadata.accession }}</td>
                            <td
                                v-if="
                                    fetchData.hits[index].metadata.latitude === null &&
                                    fetchData.hits[index].metadata.longitude === null
                                "
                            >
                                Not available
                            </td>
                            <td v-else-if="item.metadata['geo-loc-name'] !== null">
                                <button
                                    @click="
                                        MapRef.centerMarker(
                                            fetchData.hits[index].metadata.latitude,
                                            fetchData.hits[index].metadata.longitude,
                                        )
                                    "
                                >
                                    {{ item.metadata['geo-loc-name'] }}
                                    <!-- sample-metadata: "geographic location (country and/or sea,region)" -->
                                </button>
                            </td>
                            <td v-else>
                                <button
                                    @click="
                                        MapRef.centerMarker(
                                            fetchData.hits[index].metadata.latitude,
                                            fetchData.hits[index].metadata.longitude,
                                        )
                                    "
                                >
                                    Show
                                    <!-- sample-metadata: "geographic location (country and/or sea,region)" -->
                                </button>
                            </td>
                            <td>{{ item.metadata['collection-date'] }}</td>

                            <td>{{ item.metadata['sample-desc'] }}</td>
                            <!--       <td>{{ item.seqMethod }}</td> -->
                            <td>{{ item.metadata['environment-biome'] }}</td>
                            <td>
                                <a
                                    :href="'https://www.ebi.ac.uk/biosamples/samples/' + item.metadata.biosample"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    >BioSample</a
                                >
                            </td>
                            <td>
                                <a
                                    :href="'https://www.ebi.ac.uk/metagenomics/samples/' + item.metadata.accession"
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
                <h3>Genome Viewer</h3>
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
