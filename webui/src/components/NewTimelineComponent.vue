<template>
    <div>
        <Timeline
            :items="items"
            :groups="[{ id: 'group1' }]"
            :viewportMin="0"
            :viewportMax="1603049600000"
            :initialViewportStart="start"
            :initialViewportEnd="end"
        >
            <!-- <template #item="{ item }">
                <div
                    style="inset: 0; position: absolute; padding: 0.2em 1em; color: white; font-weight: bold"
                    :data-tippy-content="item.tooltip"
                >
                    ⚑ {{ item.name }}
                </div>
            </template> -->
        </Timeline>
    </div>
</template>

<script setup lang="ts">
import { Timeline } from 'vue-timeline-chart'
import 'vue-timeline-chart/style.css'
import { resultData } from '@/helpers/ResultInterface'
import { onBeforeMount, ref } from 'vue'

const props = defineProps<{
    data: resultData
}>()

interface item {
    id: string
    group: string
    name: string
    type: string
    start: Date
    cssVariables: { string: string }
}

let items: Array<item>
let start: number = new Date(props.data.hits[0]['collection-date']).valueOf()
let end: number = new Date(props.data.hits[props.data.hits.length - 1]['collection-date']).valueOf() + 1

onBeforeMount(() => {
    items = processDates()
})

function processDates() {
    const dates = []
    props.data.hits.forEach((e) => {
        const date = new Date(e['collection-date']).valueOf()

        // const date = new Date('2014-06-18')
        // const clonedate = new Date(date.valueOf())
        console.log(date.valueOf())
        dates.push({
            id: '1',
            group: 'group1',
            name: e['sample-alias'],
            type: 'point',
            start: date,
            cssVariables: { '--item-background': 'var(--color-4)' },
            // end: date.setDate(date.getDate() + 1),
            // end: new Date(clonedate.setDate(clonedate.getDate() + 1)),
        })
    })
    console.log(dates)
    return dates
}

// const items = [
//     {
//         id: 1,
//         tooltip: 'Tooltip 1',
//         name: 'Hover me!',
//         group: 'group1',
//         type: 'range',
//         start: 1707135072000,
//         end: 1708431072000,
//         cssVariables: { '--item-background': 'var(--color-4)' },
//     },
//     {
//         id: 2,
//         tooltip: 'Tooltip 2',
//         name: 'Hover me!',
//         group: 'group1',
//         type: 'range',
//         start: 1708517472000,
//         end: 1709813472000,
//         cssVariables: { '--item-background': 'var(--color-2)' },
//     },
//     {
//         id: 3,
//         tooltip: 'Tooltip 3',
//         name: 'Hover me!',
//         group: 'group1',
//         type: 'range',
//         start: 1709903872000,
//         end: 1711199872000,
//         cssVariables: { '--item-background': 'var(--color-3)' },
//     },
// ]
</script>
