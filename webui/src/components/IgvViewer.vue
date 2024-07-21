<template>
    <div>
        <div ref="root">
            <p>Blank browser to demonstrate loading igv sessions and files from html links. See igv-links.html</p>
        </div>
        <button @click="refreshIgv">Test</button>
    </div>
</template>

<script setup>
//@ts-ignore
import { reference } from '@popperjs/core'
import igv from 'igv'

import { ref, onMounted } from 'vue'

const root = ref(null)

// const test = await fetch('./metagenome_testset.json')
// .then((response) => response.json())
// .then((json) => (dataset = json))
// const testdata = test.json
// const dataset = await test.json()
// console.log(dataset)

const fasta = await fetch('./plasmid.fna').then((response) => response.text())
//     .then((text) => (testfasta = text))
// console.log(testfasta)
const fastablob = new Blob([fasta], { type: 'text/plain' })
const fastaUrl = URL.createObjectURL(fastablob)
console.log(fastaUrl)

// Receive data from parent component (JobView.vue)
const plasmids = defineProps(['data'])

const options = {
    reference: {
        id: 'test234', //not necessary
        fastaURL: fastaUrl,
        indexed: false,
        tracks: [
            {
                name: 'Plasmid',
                type: 'annotation',
                displayMode: 'EXPANDED',
                features: [
                    {
                        chr: 'NC_002119.1',
                        start: 1,
                        end: 1000,
                        locus: 'test',
                        color: 'rgb(100,0,0)',
                    },
                    {
                        chr: 'NC_002119.1',
                        start: 1500,
                        end: 3500,
                        locus: 'test',
                        color: 'rgb(200,0,0)',
                    },
                ],
            },
            {
                name: 'Plasmid2',
                type: 'annotation',
                features: [
                    {
                        chr: 'NC_002119.1',
                        start: 500,
                        end: 2500,
                        locus: 'test',
                        color: 'rgb(100,0,0)',
                    },
                    {
                        chr: 'NC_002119.1',
                        start: 2000,
                        end: 4000,
                        locus: 'test2',
                        color: 'rgb(200,0,0)',
                    },
                    {
                        chr: 'NC_002119.1',
                        start: 6000,
                        end: 8700,
                        locus: 'test3',
                        color: 'rgb(200,0,0)',
                    },
                ],
            },
        ],
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
