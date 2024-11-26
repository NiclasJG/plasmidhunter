import { job } from "./Interface.js"


export function saveJob(job) {
    let jobs = getJobs()
    jobs.push(job)
    localStorage.setItem("plasmid-jobs", JSON.stringify(jobs))
}

export function getJobs() {
    let jobs: Array<{id: string, secret: string}> = JSON.parse(localStorage.getItem("plasmid-jobs"))
    if (jobs === null) {
        jobs = []
    }
    
    return jobs
}

export function getSingleJob(jobid: String) {
    let jobs = getJobs()
    let job:job 
    jobs.forEach(element => {
        if (element.id === jobid) {
            job = element
        }
    });
    return job
}