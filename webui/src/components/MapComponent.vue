<template>
    <div ref="mapContainer" style="height: 85%; width: 100%"></div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import L from 'leaflet'

const map = ref()
const mapContainer = ref()

onMounted(() => {
    createMap()

    setInitialMarker()
})

const props = defineProps<{
    data: resultData
}>()

function createMap() {
    map.value = L.map(mapContainer.value, {
        maxBounds: [
            [-90, -200],
            [90, 200],
        ],
        maxBoundsViscosity: 0.9,
    }).setView([51, -0.09], 2)
    L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
        minZoom: 1,
        maxZoom: 19,
        attribution: '&copy; <a href="http://www.openstreetmap.org/copyright">OpenStreetMap</a>',
    }).addTo(map.value)
}

function centerMarker(lat, long) {
    map.value.flyTo([lat, long], 5)
}
// Needed to call function from ParentComponent
defineExpose({
    centerMarker,
})

function setInitialMarker() {
    if (typeof props.data !== 'undefined') {
        props.data.hits.forEach((element) => {
            // console.log(element.geolocation.location)
            try {
                var marker = L.marker([
                    parseFloat(element.geolocation.latitude),
                    parseFloat(element.geolocation.longitude),
                ]).addTo(map.value)
            } catch {
                console.log('Location not available or found.')
            }
        })
    }

    // var marker = L.marker([51.5, -0.09]).addTo(map.value)
}

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
</script>
