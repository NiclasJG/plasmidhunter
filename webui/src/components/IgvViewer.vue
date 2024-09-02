<template>
    <div>
        <div ref="root"></div>
    </div>
</template>

<script setup lang="ts">
//@ts-ignore
import { Obj, reference } from '@popperjs/core'
import igv from 'igv'

import { ref, onMounted } from 'vue'

const root = ref(null)

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

// Receive data from parent component (JobView.vue)
const props = defineProps<{
    data: resultData
}>()
// console.log(props.data.hits[0].contigs[0])

//Load Plasmid and create URL
// const plasmid_fasta = await fetch('./plasmid.fna').then((response) => response.text())
const plasmid_fasta = props.data['plasmid_name'] + '\n' + props.data['plasmid_seq']
const fastablob = new Blob([plasmid_fasta], { type: 'text/plain' })
const fastaUrl = URL.createObjectURL(fastablob)

//test chromosome
// let chrom: String = 'NC_002119.1'
let chrom: String
// Extract chromosome name from plasmid name (first word without ">")
if (typeof props.data !== 'undefined') {
    chrom = props.data.plasmid_name.split(' ')[0].substr(1)
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

const options = {
    reference: {
        //id: 'test234', //not necessary
        fastaURL: fastaUrl,
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
</script>
