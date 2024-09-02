<template>
    <div class="container-md text-center">
        <div class="row">
            <p>Insert Plasmid here.</p>
        </div>
        <div class="row">
            <textarea
                v-model="input"
                class="form-control"
                placeholder="Insert Plasmid here."
                name="DNA_Input"
                id="sequenzInput"
                style="height: 20vh"
            ></textarea>
        </div>
        <div class="row">
            <div class="col align-self-left">
                <input
                    class="form-control"
                    type="file"
                    id="formFile"
                    @change="uploadFile"
                    accept=".fas, .fna, .fasta, .fna.gz, .fas.gz, .fasta.gz"
                />
            </div>
            <div class="col align-self-right">
                <button @click="submit" type="button" class="btn btn-secondary" style="width: 20vh">Submit</button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const input = ref()
// const filename = ref()
const file = ref()

const plasmidData = {
    name: String,
    sequenz: String,
}
let fileUpload = false

function uploadFile(event) {
    file.value = event.target.files[0]
    const reader = new FileReader()
    reader.onload = (e) => (file.value = e.target.result)
    reader.readAsText(file.value)
    fileUpload = true
}

function submit() {
    if (!fileUpload && typeof input.value !== 'undefined') {
        plasmidData.name = input.value.split('\n')[0]
        plasmidData.sequenz = input.value.substring(input.value.indexOf('\n') + 1).replace(/\n/g, '')
        // plasmidData.sequenz = plasmidData.sequenz.trim().replace(/\n/g, '')
        console.log(plasmidData)
    } else if (fileUpload && typeof input.value === 'undefined') {
        plasmidData.name = file.value.split('\n')[0]
        plasmidData.sequenz = file.value.substring(file.value.indexOf('\n') + 1).replace(/\n/g, '')
        // plasmidData.sequenz = plasmidData.sequenz.trim().replace(/\n/g, '')
        console.log(plasmidData)
    }
}
</script>
