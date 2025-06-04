
import { saveJob, getJobs } from "./storagehelper.js"
import { resultData } from "./ResultInterface.js"
import { responseCreateJob, responseJobList, resultResponse, job, configFile } from "./Interface.js"
import { getConfig } from "@/configloader.js"


export async function postJob(plasmidName:string, plasmidSequenz:string) {
    let configFile : configFile = getConfig()
    let apiURL : string = configFile.api_url + 'api/v1/job/submit'
    let jobResponse: responseCreateJob

    const requestCreateJob = {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', Accept: 'application/json' },
        body: JSON.stringify({ plasmid_name: plasmidName, dna_sequenz: plasmidSequenz }),
    }

    await fetch(apiURL, requestCreateJob)
        .then((response) => response.json())
        .then((data) => (jobResponse = data))
        .catch((error) => {
            console.error('Test:', error)
        })

    saveJob(jobResponse.job)
}

export async function getJobList() {
    let configFile : configFile = getConfig()
    let apiURL : string = configFile.api_url + 'api/v1/job/list'
    let listResponse: responseJobList

    const jobsObject = {
        jobs: getJobs()
    }
    const requestJobList = {
        method: "POST",
        headers: { 'Content-Type': 'application/json', Accept: 'application/json' },
        body: JSON.stringify(jobsObject),
    }

    await fetch(apiURL, requestJobList)
        .then((response) => response.json())
        .then((data) => (listResponse = data))
        .catch((error) => {
            console.error('Test:', error)
        })
        
    return listResponse
}

export async function getJobResult(job: job) {
    let configFile : configFile = getConfig()
    let apiURL : string = configFile.api_url + 'api/v1/job/result'
    let resultResponse: resultResponse
    let results: resultData
    
    const requestJobResult = {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', Accept: 'application/json' },
        body: JSON.stringify(job ),
    }

    await fetch(apiURL, requestJobResult)
        .then((response) => response.json())
        .then((data) => (resultResponse = data))
        .catch((error) => {
            console.error('Test:', error)
        })

    let s3JobResultUrl = resultResponse.data
    // console.log(resultResponse.data)
    await fetch(s3JobResultUrl)
        .then((response) => response.json())
        .then((data) => (results = data))
        .catch((error) => {
            console.error('Test:', error)
        })
    // console.log(resultResponse.data)

    return results
}
    
