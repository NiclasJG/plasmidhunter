
import { saveJob } from "./storagehelper.js"


let responseData: {
    job: {
        id: string
        secret: string
    }
}

export async function postJob(plasmidName:string, plasmidSequenz:string) {
    const requestCreateJob = {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', Accept: 'application/json' },
        body: JSON.stringify({ plasmid_name: plasmidName, dna_sequenz: plasmidSequenz }),
    }

    await fetch('http://127.0.0.1:1239/', requestCreateJob)
        .then((response) => response.json())
        .then((data) => (responseData = data))
        .catch((error) => {
            console.error('Test:', error)
        })

    saveJob(responseData.job)
}