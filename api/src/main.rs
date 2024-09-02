use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{get}, Json, Router
};

use serde::{Deserialize, Serialize};

use tokio::net::TcpListener;
use tokio::sync::RwLock;

use uuid::Uuid;
use rand::distributions::{Alphanumeric, DistString};
use chrono::Utc;
use chrono::DateTime;


#[tokio::main]
async fn main() {
    // let server_address = "http://localhost:5170";
    let server_address = "127.0.0.1:1235";
    let db = ph_db().await;

    let listener = TcpListener::bind(server_address)
    .await
    .expect("Couldnt create TCP listener");

    // let plasmidhunter_handler = Arc::new(PlasmidHunterHandler::new().await);

    println!("listening on {}", listener.local_addr().unwrap());

    let app = Router::new().route("/", get(|| async {"Helllo"}).post(create_job))
        .route("/jobs", get(|| async {"Helllo"}))
        .route("/job/:id", get(|| async {"Helllo"}))
        .route("/about", get(|| async {"Helllo"}))
        .with_state(db);

    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}
#[derive(Debug)]
pub struct StateHandler {
    pub job_state: RwLock<HashMap<Uuid, FullJobState>>,
    //pub argo_client: Arc<ArgoClient>,
}
#[derive(Debug)]
pub struct FullJobState {
    pub api_status: Option<JobStatus>,
    // pub workflowname: Option<String>,
    pub secret: String,
    pub name: String,
}

pub struct PlasmidHunterHandler{
    //pub s3_handler: S3Handler,
    pub state_handler: Arc<StateHandler>, 
}

pub type DB = Arc<PlasmidHunterHandler>;

pub async  fn ph_db() -> DB {
    Arc::new(PlasmidHunterHandler::new().await)
}

impl StateHandler{
    pub async fn create_job(&self, name: String, dna_sequenz: String) -> (Uuid, String) {
        // add name trim?

        let job_id = Uuid::new_v4();
        let secret = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);

        self.job_state.write().await.insert(
            job_id,
            FullJobState {
                api_status: None,
                secret: secret.clone(),
                name: name.to_string(),
            },
        );
        
       
        println!("{:?}", self.job_state);

        (job_id, secret)
    }
}

impl PlasmidHunterHandler {
    pub async fn new(

    ) -> Self {
        let state_handler = Arc::new(StateHandler {
            job_state: RwLock::new(HashMap::new())
        });

        // let state_handler_clone =state_handler.clone();
        // state_handler_clone.run().await;

        PlasmidHunterHandler{
            state_handler,
        }
    }
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Job{
    pub id: Uuid,
    pub secret: String

}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JobRequest {
    plasmid_name: String,
    dna_sequenz: String
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
    pub id: Uuid,
    pub started: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub name: String,
    pub data: String,
}



async fn create_job(
    State(state): State<DB>,
    Json(job_request): Json<JobRequest>
)-> impl IntoResponse {
    let (id, secrect) = state.state_handler.create_job(job_request.plasmid_name, job_request.dna_sequenz).await;


}

