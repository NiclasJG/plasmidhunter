<template>
    <div>
        <h1>About!!!</h1>
        <!-- <TimelineComponent style="height: 499px; width: 500px" /> -->
        <div id="app">
            <!-- <VueTimeline :data="events"></VueTimeline> -->
            <TimelineComponent :data="events" />
        </div>
    </div>
</template>

<script>
import TimelineComponent from '@/components/TimelineComponent.vue'
// import VueTimeline from 'vue-timeline-component'

export default {
    name: 'App',
    components: {
        TimelineComponent,
    },
    data() {
        return {
            events: [
                {
                    name: 'event 1',
                    start: new Date(2010, 1, 1),
                    end: new Date(2010, 1, 1),
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
