<template>
    <div ref="vegalite"></div>
</template>

<script setup>
import { ref, onMounted, toRaw, onBeforeMount } from 'vue'

const vegalite = ref(null)

const props = defineProps({
    data: Object,
})
const points = ref(Array)

onMounted(() => {
    mountGraph()
})

function processDates() {
    // props.data.hits.sort((a, b) => {
    //     return Number(new Date(a['collection-date'])) - Number(new Date(b['collection-date']))
    // })
    const dates = []
    props.data.hits.forEach((e) => {
        const date = new Date(e['collection-date']).valueOf()
        dates.push({
            date: date,
        })
    })

    return dates
}

function mountGraph() {
    const values = processDates()

    console.log(values)
    const spec = {
        $schema: 'https://vega.github.io/schema/vega-lite/v5.json',
        data: {
            values,
        },
        width: 500,
        height: 300,
        selection: {
            brush: {
                type: 'interval',
                encodings: ['x'],
                bind: 'scales',
            },
        },
        mark: 'bar',
        encoding: {
            x: {
                field: 'date',
                type: 'temporal',
                bin: {
                    maxbins: 40,
                    extent: {
                        selection: 'brush',
                    },
                },
                axis: {
                    format: '%Y-%m-%d',
                    labelAngle: -45,
                    labelOverlap: false,
                },
            },
            y: {
                aggregate: 'count',
            },
        },
    }

    vegaEmbed(vegalite.value, spec)
}
</script>
