<template>
    <div>
        <div ref="root">
            <p>Blank browser to demonstrate loading igv sessions and files from html links. See igv-links.html</p>
        </div>
    </div>
</template>

<script setup>
//@ts-ignore
import igv from 'igv'

import { ref, onMounted } from 'vue'

//const root = ref(null)
//onMounted(() => console.log(root.value.outerHTML))
const root = ref(null)
const test = ref()

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

onMounted(() => {
    igv.createBrowser(root.value, options).then(function (browser) {
        console.log('Browser ready')
        igv.browser = browser
    })
})

/* onMounted(() => {
    root.value.focus()
})

test.value = root.value
 */
console.log(igv)
</script>
