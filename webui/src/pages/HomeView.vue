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
                <button @click="printTest" type="button" class="btn btn-secondary">Submit</button>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref } from 'vue'

const input = ref('')
const filename = ref()

//manage file
const file = ref()

function uploadFile(event) {
    file.value = event.target.files[0]
    const reader = new FileReader()
    reader.onload = (e) => (filename.value = e.target.result)
    reader.readAsText(file.value)
}

function printTest() {
    console.log(filename.value)
}
</script>
