<template>
    <div class="container fluid text-center">
        <div class="row">
            <h2>Insert Plasmid here.</h2>
        </div>
        <div class="row ">
            <textarea
                v-model="input"
                class="form-control"
                placeholder="Insert Plasmid in Format below or Upload Fasta-File.&#10>NC_002119.1 Escherichia coli plasmid CloDF13, complete sequence&#10AACGTAAAATGTTCAGCGAAAAACCGACATGGTTCACCTATCCTGATAATTGATCGTCAGGCAATAGAAAGACGTAATCAGGGGACAATATCCCACATCAGCGC.... "
                name="DNA_Input"
                id="sequenzInput"
                style="height: 40vh"
            ></textarea>
        </div>
        <div class="row mt-3">
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
        <div class="row">
            <div class="alert alert-success" role="alert" v-if="success">
                Successfully uploaded the Plasmid.
            </div>
            <div class="alert alert-danger" role="alert" v-if="failed">
                Something went wrong. Please check formating.
            </div>

        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { postJob } from '@/helpers/fetch_helper'

const input = ref()
// const filename = ref()
const file = ref()

const success = ref(false)
const failed = ref(false)

interface plasmidInterface {
    name: string
    sequenz: string
}

let fileUpload = false

function uploadFile(event) {
    file.value = event.target.files[0]
    const reader = new FileReader()
    reader.onload = (e) => (file.value = e.target.result)
    reader.readAsText(file.value)
    fileUpload = true
}

async function submit() {
    success.value = false
    failed.value = false
    let plasmidName: string
    let plasmidSequenz: string
    let validation = false

    if (!fileUpload && typeof input.value !== 'undefined') {

            plasmidName = input.value.split('\n')[0]
            plasmidSequenz = input.value.substring(input.value.indexOf('\n') + 1).replace(/\n/g, '')
        if (plasmidName.startsWith(">")) {
            validation = true
            input.value = ""
            success.value = true
        }else {
            failed.value = true
        }
    } else if (fileUpload && typeof input.value === 'undefined') {
        plasmidName = file.value.split('\n')[0]
        plasmidSequenz = file.value.substring(file.value.indexOf('\n') + 1).replace(/\n/g, '')
        if (plasmidName.startsWith(">")) {
        validation = true
        success.value = true
        // console.log(plasmidData)
        }else {
            failed.value = true
        }
    }
    if (validation === true) {    
        let plasmidData: plasmidInterface = {name: plasmidName, sequenz:plasmidSequenz}
        postJob(plasmidData.name, plasmidData.sequenz)
    }

}


</script>
