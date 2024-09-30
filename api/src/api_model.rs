use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use uuid::Uuid;
use rand::distributions::{Alphanumeric, DistString};

use crate::api_structs::*;
use crate::argo::client::ArgoClient;


pub struct StateHandler {
    pub job_state: RwLock<HashMap<Uuid, FullJobState>>,
    pub argo_client: Arc<ArgoClient>,
}


pub struct FullJobState {
    pub api_status: Option<JobStatus>,
    pub workflowname: Option<String>,
    pub secret: String,
    pub name: String,
}

pub struct PlasmidHunterModel{
    //pub s3_handler: S3Handler,
    pub state_handler: Arc<StateHandler>, 
}

// pub type DB = Arc<PlasmidHunterModel>;

// pub async fn ph_db() -> DB {
//     Arc::new(PlasmidHunterModel::new().await)
// }

impl StateHandler{
    async fn run(self: Arc<Self>) {

    } 

    pub async fn create_job(&self, name: String, dna_sequenz: String) -> (Uuid, String) {
        // add name trim?

        let job_id = Uuid::new_v4();
        let secret = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);

        self.job_state.write().await.insert(
            job_id,
            FullJobState {
                api_status: None,
                workflowname: None,
                secret: secret.clone(),
                name: name.to_string(),
            },
        );
        
       
        // println!("{:?}", self.job_state);

        (job_id, secret)
    }

    pub async fn get_jobs(&self, job_list: Vec<Job>) -> (ListRespone) {
        
        let mut jobs = vec![]; 
        
        ListRespone {jobs}

    }
}

impl PlasmidHunterModel {
    pub async fn new(
        argo_token: String,
        argo_url: String,
        argo_namespace: String,
        s3_access_key: String,
        s3_secret_key: String,
        bucket: String,
        endpoint: String,
        plasmidhunter_version: String,
        database_version: String,
        backend_version: String,
    ) -> Self {
        let argo_client = Arc::new(ArgoClient::new(argo_token, argo_url, argo_namespace));

        let state_handler = Arc::new(StateHandler {
            job_state: RwLock::new(HashMap::new()),
            argo_client: argo_client,
        });

        let state_handler_clone = state_handler.clone();
        state_handler_clone.run().await;

        // let state_handler_clone =state_handler.clone();
        // state_handler_clone.run().await;

        PlasmidHunterModel
        {
            state_handler,
        }
    }
}