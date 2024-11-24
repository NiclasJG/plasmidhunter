use std::{collections::HashMap, sync::Arc};

use chrono::Utc;
use tokio::sync::RwLock;
use reqwest::header::CONTENT_TYPE;
use anyhow::{Ok, Result};

use uuid::Uuid;
use rand::distributions::{Alphanumeric, DistString};

use crate::api_structs::*;
use crate::argo::client::ArgoClient;
use crate::s3_helper::{self, S3Handler};


pub struct StateHandler {
    pub job_state: RwLock<HashMap<Uuid, FullJobState>>,
    pub argo_client: Arc<ArgoClient>,
}

#[derive(Debug, Clone)]

pub struct FullJobState {
    pub api_status: Option<JobStatus>,
    pub workflowname: Option<String>,
    pub secret: String,
    pub name: String,
}

pub struct PlasmidHunterModel{
    pub s3_handler: s3_helper::S3Handler,
    pub state_handler: Arc<StateHandler>, 
}

// pub type DB = Arc<PlasmidHunterModel>;

// pub async fn ph_db() -> DB {
//     Arc::new(PlasmidHunterModel::new().await)
// }

impl StateHandler{
    async fn run(self: Arc<Self>) {

    } 

    pub async fn create_job(&self, name: &String) -> (Uuid, String) { //, sequenz: &String, s3_post_url: &String
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
        

        // let mut map = HashMap::new();
        // map.insert("name:", &name);
        // map.insert("dna_sequenz", &sequenz);
        
    
        // let pname = name;
        // let dsequenz = sequenz;
    
        // let client = self.argo_client.client;
        // let res = client.put(s3_post_url)
        //                                         .header(CONTENT_TYPE, "text/plain")
        //                                         .body(format!("{pname}\n{dsequenz}").to_string())
        //                                         // .json(&map)
        //                                         .send().await;
        // println!("{:?}", self.job_state);

        (job_id, secret)
    }

    pub async fn start_job(&self, name: &String, sequenz: &String, s3_post_url: &String, jobid: &Uuid ) -> Result<()> {
        let mut map = HashMap::new();
        map.insert("name:", &name);
        map.insert("dna_sequenz", &sequenz);
        
    
        let pname = name;
        let dsequenz = sequenz;
    
        let client = &self.argo_client.client;
        let s3_response = client.put(s3_post_url)
                        .header(CONTENT_TYPE, "text/plain")
                        // .danger_accept_invalid_hostnames()
                        .body(format!("{pname}\n{dsequenz}").to_string())
                        // .json(&map)
                        .send()
                        .await;

        // println!("{:?}", s3_response);

        let argo_response = self.argo_client.start_workflow(&jobid.to_string(), name).await?;

        let job_status = JobStatus{
          id: jobid.clone(),
          status: String::from("Started"),
          started: argo_response.metadata.creationTimestamp.unwrap(),
          updated: Utc::now(),
          name: name.clone()
        };

        let mut write_state = self.job_state.write().await;

        let full_job_state = write_state.get_mut(jobid).unwrap();

        full_job_state.api_status = Some(job_status);
        full_job_state.workflowname = Some(argo_response.metadata.name);


        Ok(())
        // println!("{:?}", test)

    }

    pub async fn get_jobs(&self, job_list: Vec<Job>) -> ListRespone {
        
        let mut jobs = vec![]; 

        let read_state = self.job_state.read().await;

        for job in job_list {
            
            let job_api_status = read_state.get(&job.id);
            if job_api_status.is_some() {
                //secret check
                jobs.push(job_api_status.unwrap().api_status.as_ref().unwrap().clone())
            }
            
            
        };
        
        ListRespone {jobs}
    }

    pub async fn get_job_result(&self, job: Job, result_url: String) -> Result<ResultResponse> {
        let read_state = self.job_state.read().await;

        let job_api_status = read_state.get(&job.id).unwrap();
        // let Some(job_api_status) = read_state.get(&job.id) else {
        //     return Err("Job ID couldnt be found".into())
        // };

        //secret check

        let result_response = ResultResponse {
            name: job_api_status.name.clone(),
            data: result_url
        };

        Ok(result_response)
    }

}

impl PlasmidHunterModel {
    pub async fn new(
        argo_token: String,
        argo_url: String,
        argo_namespace: String,
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

        let s3_handler = S3Handler::new().await;

        PlasmidHunterModel
        {
            s3_handler,
            state_handler,
        }
    }
}