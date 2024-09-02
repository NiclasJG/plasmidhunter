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
// import { resultData } from '@/assets/ResultInterface'

const MapRef = ref<InstanceType<typeof Map | null>>(null)
// const MapRef = ref<HTMLDivElement>()

interface resultData {
    plasmid_name: string
    plasmid_seq: string
    hits: Array<{
        dataset: string
        geolocation: {
            location: string
            longitude: string
            latitude: string
        }
        seqMethod: string
        sampleOrigin: string
        collectionDate: string
        biosample: string
        mgnifySample: string
        contigs: Array<contig>
    }>
}

interface contig {
    plasmid: string
    contig: string
    contigstart: string
    contigend: string
    contiglength: string
    coverage: string
    identity: string
    alignmentlength: string
    strand: string
    plasmidstart: string
    plasmidend: string
    plasmidlength: string
}

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

<style>
/* .table-responsive {
    max-height: 20vh;
}

.column {
    float: left;
}

.left {
    float: left;
    width: 50%;
}

.right {
    float: right;
    position: center;
    width: 50%;
} */
</style>
