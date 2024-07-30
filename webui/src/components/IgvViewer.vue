<template>
    <div>
        <div ref="root"></div>
        <button @click="refreshIgv">Test</button>
    </div>
</template>

<script setup>
//@ts-ignore
import { reference } from '@popperjs/core'
import igv from 'igv'

import { ref, onMounted, toRaw, watchEffect, watch } from 'vue'

const root = ref(null)

// Receive data from parent component (JobView.vue)
const props = defineProps({
    data: Object,
})

const dataset = props.data
// console.log(dataset.plasmid_name)

//Load Plasmid and create URL
// const plasmid_fasta = await fetch('./plasmid.fna').then((response) => response.text())
const plasmid_fasta = dataset['plasmid_name'] + '\n' + dataset['plasmid_seq']
const fastablob = new Blob([plasmid_fasta], { type: 'text/plain' })
const fastaUrl = URL.createObjectURL(fastablob)

//test chromosome
// const chrom = 'NC_002119.1'
// Extract chromosome name from plasmid name (first word without ">")
const chrom = dataset.plasmid_name.split(' ')[0].substr(1)

// Create Tracks and features out of results
function createTracks(results, chromosome) {
    const tracks = []
    results.forEach((element) => {
        const features = createFeatures(element['contigs'], chromosome)
        tracks.push({
            name: element.dataset,
            type: 'annotation',
            features: features,
        })
    })
    return tracks
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

const options = {
    reference: {
        //id: 'test234', //not necessary
        fastaURL: fastaUrl,
        indexed: false,
        tracks: createTracks(dataset['hits'], chrom),
        wholeGenomeView: false,
    },

    loadDefaultGenomes: false,
}

// Create IGV-Viewer at start
onMounted(() => {
    setupIgv(options)
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
</script>
