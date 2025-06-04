
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

    let (id, secret) = state.state_handler.create_job(&job_request.plasmid_name).await;
    
    let post_plasmid_url = state.s3_handler.presigned_post_url(id.to_string()).await.unwrap();

    let start_response = state.state_handler.start_job(&job_request.plasmid_name, &job_request.dna_sequenz, &post_plasmid_url, &id, &secret).await;

    ( StatusCode::INTERNAL_SERVER_ERROR,
    json!({"success": false, "message": "start_response"}).to_string()).into_response();


    (StatusCode::OK, Json(JobResponse{
        job: Job{id, secret},
    })).into_response()
    
}

pub async fn get_job_list(
    State(state): State<Arc<PlasmidHunterModel>>,
    Json(list_request): Json<ListRequest>,
) -> impl IntoResponse {
    let state_handler_clone: Arc<StateHandler> = state.state_handler.clone();
    let state_updater = state_handler_clone.update_jobs().await;

    let response= Json(state.state_handler.get_jobs(list_request.jobs).await.unwrap());

    response.into_response()

}

pub async fn get_job_result(
    State(state): State<Arc<PlasmidHunterModel>>,
    Json(result_request): Json<Job>
) -> impl IntoResponse {
    let get_result_url = state.s3_handler.presigned_get_url(result_request.id.to_string()).await.unwrap();

    let response = Json(state.state_handler.get_job_result(result_request, get_result_url).await.unwrap());
    
    response.into_response()
}

pub async fn delete_job(
    State(state): State<Arc<PlasmidHunterModel>>,
    Json(job): Json<Job>
) -> impl IntoResponse {
    let response = state.state_handler.delete_job(job).await;
}