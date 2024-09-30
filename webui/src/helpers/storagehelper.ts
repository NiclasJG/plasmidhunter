

export function saveJob(job) {
    var jobs = getJobs()
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