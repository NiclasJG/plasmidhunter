<template>
    <div
        class="timeline-container"
        ref="timelineContainer"
        @mousedown="startDragging"
        @mouseup="stopDragging"
        @mouseleave="stopDragging"
        @mousemove="drag"
        @wheel="zoom"
    >
        <div
            class="timeline"
            :style="{ transform: `translateX(${position.valueOf()}px) scale(${zoomLevel.valueOf()})` }"
        >
            <!-- Horizontaler Strich -->
            <div class="line" :style="{ width: `${lineWidth.valueOf()}px` }"></div>

            <!-- Datenpunkte -->
            <div
                class="point"
                v-for="(point, index) in processedPoints.valueOf()"
                :key="index"
                :style="{
                    left: `${point.scaledX}px`,
                }"
            >
                <!-- Name oberhalb -->
                <div
                    class="point-label"
                    :style="{
                        fontSize: `${baseFontSize.valueOf() / zoomLevel.valueOf()}px`,
                        top: `${-40 - point.level * verticalOffset.valueOf()}px`,
                    }"
                >
                    {{ point.label }}
                </div>
                <!-- Punkt -->
                <div
                    class="point-circle"
                    :style="{
                        width: `${basePointSize.valueOf() / zoomLevel.valueOf()}px`,
                        height: `${basePointSize.valueOf() / zoomLevel.valueOf()}px`,
                    }"
                ></div>
                <!-- Datum unterhalb -->
                <div
                    class="point-date"
                    :style="{
                        fontSize: `${baseFontSize.valueOf() / zoomLevel.valueOf()}px`,
                        top: `${30 + point.level * verticalOffset.valueOf()}px`,
                    }"
                >
                    {{ point.date }}
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, computed, onBeforeMount } from 'vue'
// import { resultData } from '@/helpers/ResultInterface'

const props = defineProps({
    data: Object,
})
const points = ref(Array)

onBeforeMount(() => {
    points.value = processDates()
    console.log(points.valueOf())
})

// Reactive references
/* const points = ref([
    { date: '2024-01-01', label: 'Start' },
    { date: '2024-01-15', label: 'Phase1' },
    { date: '2024-01-20', label: 'EventA' },
    { date: '2024-03-01', label: 'EventB' },
    { date: '2024-04-15', label: 'Phase2' },
    { date: '2024-07-01', label: 'End' },
]) */
const zoomLevel = ref(1)
const position = ref(0)
const isDragging = ref(false)
const startX = ref(0)

const baseFontSize = ref(18) // Basisgröße für Text
const basePointSize = ref(12) // Basisgröße für Punkte
const minSpacing = ref(150) // Mindestabstand in Pixeln
const verticalOffset = ref(35) // Versatz für jede Ebene

function processDates() {
    // props.data.hits.sort((a, b) => {
    //     return Number(new Date(a['collection-date'])) - Number(new Date(b['collection-date']))
    // })

    const dates = []
    props.data.hits.forEach((e) => {
        const date = new Date(e['collection-date']).toLocaleDateString()

        // console.log(date)
        dates.push({
            date: date,
            label: e.accession,
        })
    })
    return dates
}

// Berechnung der skalierten Positionen und dynamischen Abstände
const processedPoints = computed(() => {
    const baseDate = new Date(points.value[0].date).getTime()
    const maxDate = new Date(points.value[points.value.length - 1].date).getTime()
    const dateRange = maxDate - baseDate

    const scaledPoints = points.value.map((point) => {
        const date = new Date(point.date).getTime()
        const relativePosition = (date - baseDate) / dateRange
        return {
            ...point,
            scaledX: relativePosition * 1000 * zoomLevel.value, // 1000px für die gesamte Breite
            level: 0, // Standardebene (keine Verschiebung)
        }
    })

    // Überlappungen beheben durch dynamische Ebenenzuweisung
    for (let i = 0; i < scaledPoints.length; i++) {
        for (let j = i + 1; j < scaledPoints.length; j++) {
            const currentPoint = scaledPoints[i]
            const nextPoint = scaledPoints[j]

            const distance = nextPoint.scaledX - currentPoint.scaledX

            // Wenn Punkte überlappen, erhöhe die Ebene des späteren Punktes
            if (distance < minSpacing.value) {
                nextPoint.level = Math.max(nextPoint.level, currentPoint.level + 1)
            }
        }
    }

    return scaledPoints
})

const lineWidth = computed(() => {
    return Math.max(...processedPoints.value.map((point) => point.scaledX))
})

// Dragging Methoden
const startDragging = (event) => {
    isDragging.value = true
    startX.value = event.clientX - position.value
}

const stopDragging = () => {
    isDragging.value = false
}

const drag = (event) => {
    if (isDragging.value) {
        position.value = event.clientX - startX.value
    }
}

// Zooming Methode
const zoom = (event) => {
    event.preventDefault()

    const zoomFactor = 0.1
    const container = event.target.closest('.timeline-container')
    const containerRect = container.getBoundingClientRect()

    const mouseX = event.clientX - containerRect.left
    const zoomCenter = (mouseX - position.value) / zoomLevel.value

    // Zoom anpassen
    if (event.deltaY < 0) {
        zoomLevel.value = Math.min(zoomLevel.value + zoomFactor, 3)
    } else {
        zoomLevel.value = Math.max(zoomLevel.value - zoomFactor, 0.25)
    }

    // Zoom auf die Mausposition zentrieren
    position.value = mouseX - zoomCenter * zoomLevel.value
}
</script>

<style scoped>
.timeline-container {
    width: 100%;
    height: 300px;
    overflow: hidden;
    position: relative;
    border: 1px solid #ccc;
    cursor: grab;
}

.timeline-container:active {
    cursor: grabbing;
}

.timeline {
    position: relative;
    top: 50%;
    transform: translateY(-50%);
}

.line {
    position: absolute;
    top: 50%;
    left: 0;
    height: 4px;
    background-color: black;
    transform: translateY(-50%);
}

.point {
    position: absolute;
    top: 50%;
    transform: translate(-50%, -50%);
    text-align: center;
}

.point-label {
    position: absolute;
    top: -30px; /* Standardposition oberhalb */
}

.point-circle {
    background-color: red;
    border-radius: 50%;
    margin: 0 auto;
}

.point-date {
    position: absolute;
    top: 30px; /* Standardposition unterhalb */
    color: gray;
}
</style>
