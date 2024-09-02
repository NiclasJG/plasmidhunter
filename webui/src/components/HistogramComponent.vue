<template>
    <div>
        <Line :data="data" :options="options" />
    </div>
</template>

<script setup lang="ts">
import {
    Chart,
    Title,
    Tooltip,
    Legend,
    BarElement,
    CategoryScale,
    LineElement,
    LinearScale,
    TimeScale,
    PointElement,
} from 'chart.js'
import { Line } from 'vue-chartjs'
import 'chartjs-adapter-date-fns'

Chart.register(CategoryScale, LinearScale, BarElement, Title, Tooltip, Legend, TimeScale, PointElement, LineElement)

const props = defineProps<{ dataChart: resultData }>()

const labels: string[] = []

const count: number[] = []

props.dataChart.hits.forEach((item) => {
    if (labels.some((x) => x === item.collectionDate) === false) {
        labels.push(item.collectionDate)
        count.push(1)
    }
})

console.log(labels)
console.log(count)

const data = {
    // labels: ['2014-11-06', '2022-12-07 ', '2023-11-07'],
    // datasets: [
    //     {
    //         data: [
    //             {
    //                 x: '2014-11-06',
    //                 y: 1,
    //             },
    //             {
    //                 x: '2023-11-07 ',
    //                 y: 2,
    //             },
    //             {
    //                 x: '2022-12-07 ',
    //                 y: 1,
    //             },
    //         ],
    //     },
    // ],
    labels: labels,
    datasets: [{ data: count }],
}

const options = {
    // showLine: false,
    scales: {
        x: {
            min: '2000-01-01',

            type: 'time',
            // time: {
            //     unit: 'month',
            // },
        },
        y: {
            min: '0',
            max: '2',
        },
    },
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
