<template>
    <div id="app">
        <vue-timeline :data="events"></vue-timeline>
    </div>
</template>

<script>
import VueTimeline from 'vue-timeline-component'

export default {
    name: 'App',
    component: {
        VueTimeline,
    },
    data() {
        return {
            events: [
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
            ],
        }
    },
}
</script>

<!-- 
<script setup>
import { defineComponent } from 'vue'
import VueTimeline from 'vue-timeline-component'

// defineOptions({
//     name: 'App',
//     components: {
//         VueTimeline,
//     },
// })
const events = [
    {
        name: 'event 1',
        start: new Date(2020, 1, 1),
    },
    {
        name: 'event 2',
        start: new Date(2022, 1, 2),
    },
    {
        name: 'event 3',
        start: new Date(1996, 1, 3),
    },
]
</script> -->
