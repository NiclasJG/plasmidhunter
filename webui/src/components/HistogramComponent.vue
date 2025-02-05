<template>
    <div ref="vegalite" style="width: 100%"></div>
</template>

<script setup>
import { ref, onMounted } from 'vue'

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
        const date = new Date(e.metadata['collection-date']).valueOf()
        if (date !== 0) {
            // console.log(date)
            dates.push({
                date: date,
            })
        }
    })

    return dates
}

function mountGraph() {
    const values = processDates()

    const spec = {
        $schema: 'https://vega.github.io/schema/vega-lite/v5.json',
        data: {
            values,
        },
        width: 'container', //500
        height: 300, // 300
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
                title: 'Date',
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
