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
import igv from 'igv'

import { ref, onMounted } from 'vue'

const root = ref(null)

// Receive data from parent component (JobView.vue)
const plasmids = defineProps(['data'])

const options = {
    reference: {
        id: 'hg38',
        name: 'Human (GRCh38/hg38)',
        fastaURL: 'https://s3.amazonaws.com/igv.broadinstitute.org/genomes/seq/hg38/hg38.fa',
        indexURL: 'https://s3.amazonaws.com/igv.broadinstitute.org/genomes/seq/hg38/hg38.fa.fai',
        cytobandURL: 'https://s3.amazonaws.com/igv.broadinstitute.org/annotations/hg38/cytoBandIdeo.txt',
        tracks: [
            {
                name: 'Refseq Genes',
                url: 'https://s3.amazonaws.com/igv.org.genomes/hg38/refGene.txt.gz',
                order: 1000000,
                indexed: false,
            },
        ],
    },
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
