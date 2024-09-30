<template>
    <div class="container fluid text-center">
        <div><h1>Single Job</h1></div>

        <div class="row">
            <div class="col">
                <h3>Locations</h3>
                <Map ref="MapRef" :data="fetchData" />
            </div>
            <div class="col">
                <h3>Histogram</h3>
                <Histogram :dataChart="fetchData" />
            </div>
            <!-- <div class="col">
                <h3>Timeline</h3>
                <TimelineComponent />
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
                        <th>{{ item.dataset }}</th>
                        <th>
                            <button
                                @click="
                                    MapRef.centerMarker(
                                        fetchData.hits[index].geolocation.latitude,
                                        fetchData.hits[index].geolocation.longitude,
                                    )
                                "
                            >
                                {{ item.geolocation.location }}
                            </button>
                        </th>
                        <th>{{ item.seqMethod }}</th>
                        <th>{{ item.sampleOrigin }}</th>
                        <th>{{ item.collectionDate }}</th>
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
                                :href="'https://www.ebi.ac.uk/metagenomics/samples/' + item.mgnifySample"
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
</template>

<script setup lang="ts">
import IgvViewer from '@/components/IgvViewer.vue'
import Map from '@/components/MapComponent.vue'
import Histogram from '@/components/HistogramComponent.vue'
import { ref } from 'vue'
import TimelineComponent from '@/components/TimelineComponent.vue'
import { resultData } from '@/assets/ResultInterface'
// import { resultData } from '@/assets/ResultInterface'

const MapRef = ref<InstanceType<typeof Map | null>>(null)

const fetchData = ref<resultData>()
//fetch local file (./metagenome.. loads 'http://localhost:5173/job/metagenome_testset.json' )
await fetch('http://localhost:5173/metagenome_testset.json')
    .then((response) => response.json())
    .then((data) => (fetchData.value = data))
    .catch((error) => console.log(error))
// })

fetchData.value.hits.sort((a, b) => {
    return Number(new Date(a.collectionDate)) - Number(new Date(b.collectionDate))
})
</script>

<style></style>
