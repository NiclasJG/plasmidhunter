
import { saveJob, getJobs } from "./storagehelper.js"
import { resultData } from "./ResultInterface.js"
import { responseCreateJob, responseJobList, resultResponse, job } from "./Interface.js"


export async function postJob(plasmidName:string, plasmidSequenz:string) {
    let jobResponse: responseCreateJob
    
    const requestCreateJob = {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', Accept: 'application/json' },
        body: JSON.stringify({ plasmid_name: plasmidName, dna_sequenz: plasmidSequenz }),
    }

    await fetch('http://127.0.0.1:1239/', requestCreateJob)
        .then((response) => response.json())
        .then((data) => (jobResponse = data))
        .catch((error) => {
            console.error('Test:', error)
        })

    saveJob(jobResponse.job)
}

export async function getJobList() {
    let listResponse: responseJobList
    const jobsObject = {
        jobs: getJobs()
    }
    // if (jobsObject.jobs.length !== 0) {
        const requestJobList = {
            method: "POST",
            headers: { 'Content-Type': 'application/json', Accept: 'application/json' },
            body: JSON.stringify(jobsObject),
        }

        await fetch('http://127.0.0.1:1239/api/v1/job/list', requestJobList)
            .then((response) => response.json())
            .then((data) => (listResponse = data))
            .catch((error) => {
                console.error('Test:', error)
            })
        
  

    // console.log(listResponse)
    return listResponse
}

export async function getJobResult(job: job) {
    let resultResponse: resultResponse

    let results: resultData
    
    const requestJobResult = {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', Accept: 'application/json' },
        body: JSON.stringify(job ),
    }

    await fetch('http://127.0.0.1:1239/api/v1/job/result', requestJobResult)
        .then((response) => response.json())
        .then((data) => (resultResponse = data))
        .catch((error) => {
            console.error('Test:', error)
        })

    let test = resultResponse.data
    console.log(resultResponse.data)
    await fetch(test)
        .then((response) => response.json())
        .then((data) => (results = data))
        .catch((error) => {
            console.error('Test:', error)
        })
    console.log(resultResponse.data)
    return results
}
    
