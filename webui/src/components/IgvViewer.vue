<template>
    <div>
        <div ref="root"></div>
    </div>
</template>

<script setup lang="ts">
//@ts-ignore
import { Obj, reference } from '@popperjs/core'
import igv from 'igv'
import { resultData } from '@/assets/ResultInterface'

import { ref, onMounted } from 'vue'

// Receive data from parent component (JobView.vue)
const props = defineProps<{
    data: resultData
}>()

const root = ref(null)

let chrom: String
// let chrom: String = 'NC_002119.1'

// Extract chromosome name from plasmid name (first word without ">")
if (typeof props.data !== 'undefined') {
    chrom = props.data.plasmid_name.split(' ')[0].substr(1)
}

const options = {
    reference: {
        //id: 'test234', //not necessary
        fastaURL: createFastaUrl(),
        indexed: false,
        // Add error function with empty tracks
        tracks: createTracks(props.data.hits, chrom),
        wholeGenomeView: false,
    },

    loadDefaultGenomes: false,
}

// Create IGV-Viewer at start
onMounted(() => {
    if (typeof props.data !== 'undefined') {
        setupIgv(options)
    }
})

// Initialize IGV-Viewer
function setupIgv(options) {
    igv.createBrowser(root.value, options).then(function (browser) {
        console.log('Browser ready')
        igv.browser = browser
    })
}

// Update IGV-Viewer
function refreshIgv() {
    const newgenome = {
        genome: 'hg19',
    }
    igv.browser.loadGenome(newgenome)
}

function createFastaUrl() {
    //Load Plasmid and create URL
    const plasmid_fasta = props.data['plasmid_name'] + '\n' + props.data['plasmid_seq']
    const fastablob = new Blob([plasmid_fasta], { type: 'text/plain' })
    const fastaUrl = URL.createObjectURL(fastablob)

    return fastaUrl
}

// Create Tracks and features out of results
function createTracks(results, chromosome) {
    const tracksArray = []

    try {
        results.forEach((element) => {
            const features = createFeatures(element['contigs'], chromosome)
            tracksArray.push({
                //without ts element.props.data worked
                name: element.dataset,
                type: 'annotation',
                features: features,
            })
        })
    } catch (err) {
        console.log('Couldnt create Tracks!!', err)
    }
    return tracksArray
}

function createFeatures(contigs, chromosome) {
    const features = []
    contigs.forEach((element) => {
        features.push({
            chr: chromosome,
            start: element['plasmid start'],
            end: element['plasmid end'],
            color: 'rgb(100,0,0)',
        })
    })
    return features
}
</script>
