<template>
    <div>
        <div ref="root"></div>
    </div>
</template>

<script setup lang="ts">
import igv from 'igv'
import { resultData } from '@/helpers/ResultInterface'
import { ref, onMounted } from 'vue'

// Receive data from parent component (JobView.vue)
const props = defineProps<{
    data: resultData
}>()

const root = ref(null)

let chrom: String

// Extract chromosome name from plasmid name
if (typeof props.data !== 'undefined') {
    chrom = props.data.annotation.sequences[0].orig_description.split(' ')[0]
}

const options = {
    // genome: chrom,
    reference: {
        fastaURL: createFastaUrl(),
        indexed: false,
        // Add error function with empty tracks
        tracks: createTracks(props.data.annotation, props.data.hits, chrom),
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
    console.log(chrom)
}

// Update IGV-Viewer
function refreshIgv() {
    const newgenome = {
        genome: 'hg19',
    }
    igv.browser.loadGenome(newgenome)
}

function createFastaUrl() {
    //Load Plasmid from annotation data and create URL
    const plasmid_fasta =
        '>' + props.data.annotation.sequences[0].orig_description + '\n' + props.data.annotation.sequences[0].nt
    const fastablob = new Blob([plasmid_fasta], { type: 'text/plain' })
    const fastaUrl = URL.createObjectURL(fastablob)
    return fastaUrl
}

// Create Tracks and features out of results
function createTracks(annotation, samples, chromosome) {
    const tracksArray = []
    // console.log(samples)
    const annotationTrack = {
        type: 'annotation',
        features: [],
    }

    annotation.features.forEach((e) => {
        annotationTrack.features.push({
            chr: chromosome,
            name: e.product,
            start: e.start,
            end: e.stop,
            strand: e.strand,
            color: 'rgb(100,0,0)',
        })
    })

    tracksArray.push(annotationTrack)

    try {
        samples.forEach((sample) => {
            const features = createFeatures(sample.runs, chromosome)
            tracksArray.push({
                name: sample.metadata.accession,
                type: 'annotation',
                displayMode: 'expanded',
                features: features,
            })
        })
    } catch (err) {
        console.log('Couldnt create Tracks!!', err)
    }

    return tracksArray
}

function createFeatures(runs, chromosome) {
    const features = []
    let rgb_value = random_color()
    console.log(runs)
    runs.forEach((run) => {
        run.contigs.forEach((contig) => {
            let contig_id = contig.contig_id

            contig.alignments.forEach((alignment) => {
                features.push({
                    //chr must be same named as shown right from igv symbol in browser
                    chr: chromosome,
                    name: contig_id,
                    start: alignment.plasmid_start,
                    end: alignment.plasmid_end,
                    color: rgb_value,
                    strand: alignment.strand,
                    row: 0,
                })
            })
        })

        // features.push({
        //     //chr must be same named as shown right from igv symbol in browser
        //     chr: chromosome,
        //     name: element.contig_id,
        //     start: element.plasmid_start,
        //     end: element.plasmid_end,
        //     color: rgb_value,
        //     strand: element.strand,
        //     row: 0,
        // })
    })

    function random_color() {
        const r = Math.floor(Math.random() * 200)
        const g = Math.floor(Math.random() * 150)
        const b = Math.floor(Math.random() * 150)

        var rgb = `rgb(${r},${g},${b})`

        return rgb
    }

    return features
}
</script>
