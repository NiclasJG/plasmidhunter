export interface responseCreateJob {
    job: job
}

export interface responseJobList {
    jobs: Array<jobStatus>
}

export interface jobStatus {
    id: string,
    status: string,
    started: Date,
    updated: Date,
    name: string,
}

export interface job {
    id: string,
    secret: string
}

export interface resultResponse {
    name: string,
    data: string
}

export interface configFile {
    api_url: string,
    key: string
}