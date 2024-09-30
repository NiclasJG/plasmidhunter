
use std::sync::Arc;
use axum::{
    extract::State, http::StatusCode, response::IntoResponse, Json
};

use serde_json::json;

use crate::api_structs::*;
use crate::api_model::*;


pub async fn create_job(
    State(state): State<Arc<PlasmidHunterModel>>,
    Json(job_request): Json<JobRequest>
)-> impl IntoResponse {
    let (id, secret) = state.state_handler.create_job(job_request.plasmid_name, job_request.dna_sequenz).await;
    
    ( StatusCode::INTERNAL_SERVER_ERROR,
    json!({"success": false, "message": "test"}).to_string()).into_response();


    (StatusCode::OK, Json(JobResponse{
        job: Job{id, secret},
    })).into_response()
    
}

pub async fn get_job_list(
    State(state): State<Arc<PlasmidHunterModel>>,
    Json(list_request): Json<ListRequest>,
) -> impl IntoResponse {
    Json(state.state_handler.get_jobs(list_request.jobs).await)
}