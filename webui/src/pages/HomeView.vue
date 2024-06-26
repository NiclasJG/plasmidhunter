<template>
    <div class="container-md text-center">
        <div class="row">
            <p>Insert Plasmid here!</p>
        </div>
        <div class="row">
            <textarea
                v-model="input"
                class="form-control"
                placeholder="Insert Plasmid here!"
                name="DNA_Input"
                id="sequenzInput"
            ></textarea>
        </div>
        <div class="row">
            <div class="col align-self-left">
                <input class="form-control" type="file" id="formFile" />
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

function printTest() {
    console.log(input.value)
}
</script>
