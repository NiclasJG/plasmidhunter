use serde::{Deserialize, Serialize};

use uuid::Uuid;

use chrono::DateTime;
use chrono::Utc;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Job{
    pub id: Uuid,
    pub secret: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JobRequest {
    pub plasmid_name: String,
    pub dna_sequenz: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JobResponse{
    pub job: Job,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ListRequest {
    pub jobs: Vec<Job>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ListRespone {
    pub jobs: Vec<JobStatus>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JobStatus {
    pub id: Uuid,
    pub status: String,
    pub started: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ResultResponse {
    // pub id: Uuid,
    // pub started: DateTime<Utc>,
    // pub updated: DateTime<Utc>,
    pub name: String,
    pub data: String,
}