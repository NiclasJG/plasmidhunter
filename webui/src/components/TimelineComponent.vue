<template>
    <div id="app">
        <timelinevue :data="events" />
    </div>
</template>

<script lang="ts">
import timelinevue from '@/graph/timelinevue.vue'
import { PropType, ref } from 'vue'
import { resultData } from '@/helpers/ResultInterface'

/* const events = ref([
    {
        name: 'event 1',
        start: new Date(2020, 1, 1),
        end: new Date(2020, 1, 4),
    },
    {
        name: 'event 2',
        start: new Date(2020, 1, 2),
        end: new Date(2020, 1, 5),
    },
    {
        name: 'event 3',
        start: new Date(2020, 1, 3),
        end: new Date(2020, 1, 10),
    },
]) */

export default {
    props: {
        dateList: Object as PropType<resultData>,
    },

    name: 'App',
    components: {
        timelinevue,
    },
    data() {
        return {
            events: () => {
                const dates = []
                this.dateList.hits.forEach((e) => {
                    const date = new Date(e['collection-date'])
                    // const date = new Date('2014-06-18')
                    const clonedate = new Date(date.valueOf())
                    // console./* log */(e)
                    dates.push({
                        name: e['sample-alias'],
                        start: date,
                        // end: date.setDate(date.getDate() + 1),
                        end: new Date(clonedate.setDate(clonedate.getDate() + 1)),
                    })
                })
                return dates
            },
            // events: [
            //     {
            //         name: 'event 1',
            //         start: new Date(2020, 1, 1),
            //     },
            //     {
            //         name: 'event 2',
            //         start: new Date(2020, 1, 9),
            //         end: new Date(2020, 1, 10),
            //     },
            //     {
            //         name: 'event 3',
            //         start: new Date(2020, 5, 2),
            //     },
            // ],
        }
    },
    mounted() {
        // this.extractDate()
    },
    methods: {
        // extractDate() {
        // this.dateList.hits.forEach((e) => {
        //     const date = new Date(e.collectionDate)
        //     this.events.push({
        //         name: e.dataset,
        //         start: date,
        //         end: date,
        //     })
        // })
        // console.log(this.events)
        // },
    },
}
</script>
