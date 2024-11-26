
use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Ok;
use axum::{
    extract::State, http::StatusCode, response::IntoResponse, Json
};
use reqwest;
use serde_json::json;

use crate::api_structs::*;
use crate::api_model::*;


pub async fn create_job(
    State(state): State<Arc<PlasmidHunterModel>>,
    Json(job_request): Json<JobRequest>
)-> impl IntoResponse {

    let (id, secret) = state.state_handler.create_job(&job_request.plasmid_name).await;
    
    let post_plasmid_url = state.s3_handler.presigned_post_url(id.to_string()).await.unwrap();

    let test = state.state_handler.start_job(&job_request.plasmid_name, &job_request.dna_sequenz, &post_plasmid_url, &id).await;

    // let mut map = HashMap::new();
    // map.insert("name:", &job_request.plasmid_name);
    // map.insert("dna_sequenz", &job_request.dna_sequenz);
    

    // let pname = job_request.plasmid_name;
    // let dsequenz = job_request.dna_sequenz;

    // let client = reqwest::Client::new();
    // let res = client.put(post_plasmid_url.unwrap())
    //                                         .header(CONTENT_TYPE, "text/plain")
    //                                         .body(format!("{pname}\n{dsequenz}").to_string())
    //                                         // .json(&map)
    //                                         .send().await;

    // println!("Create Job:{:?}",state.state_handler.argo_client.token);



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
    let state_handler: Arc<StateHandler> = state.state_handler.clone();
    let test2 = state_handler.update_jobs().await;

    println!("{:?}", test2);
    let response= Json(state.state_handler.get_jobs(list_request.jobs).await);

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