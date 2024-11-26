use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Utc};
use serde::de::value;
use tokio::sync::RwLock;
use reqwest::header::CONTENT_TYPE;
use anyhow::{Ok, Result};

use uuid::{uuid, Uuid};
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
    pub async fn update_jobs(&self) -> Result<()> {
        

        let job_list = self.argo_client.get_workflows().await.unwrap();
        let mut job_state_list = self.job_state.write().await;
        println!("test1");
        // let job_list_iter = job_list.items.iter();
        for job in job_list.items {
            let mut jobid = Uuid::new_v4();
            let mut name = String::new();
            for param in job.spec.arguments.parameters {
                if param.name == "jobid" {
                    // crashes if value cant be parsed
                    println!("{:?}", param.value);
                    jobid = Uuid::parse_str(&param.value).unwrap();
                    println!("{:?}{:?}", jobid, param.value);
                } 
                if param.name == "parameter" {
                    name = String::from(param.value)
                }
            }
            

            let job_full_state = job_state_list.get(&jobid);

            if job_full_state.is_some() {
                let full_state = job_full_state.unwrap().clone();
                let api_status = full_state.api_status.unwrap().clone();
                // if job.status.phase == "Succeeded" {
                //     let mut api_status = full_state.api_status.unwrap();
                //     api_status.status = job.status.phase;
                //     api_status.updated = job.status.finishedAt.unwrap();
                // }
                println!("test4.1");
                if job.status.phase == "Succeeded" {
                    let full_job_state = FullJobState {
                        api_status: Some(JobStatus {
                            id: api_status.id,
                            status: job.status.phase,
                            started: api_status.started,
                            updated: job.status.finishedAt.unwrap(),
                            name: api_status.name,
                        }), 
                        workflowname: full_state.workflowname, 
                        secret: full_state.secret,
                        name: full_state.name,
                    };

                    job_state_list.insert(jobid.clone(), full_job_state);
                    println!("test5.1");
                } else {
                    let full_job_state = FullJobState {
                        api_status: Some(JobStatus {
                            id: api_status.id,
                            status: api_status.status,
                            started: api_status.started,
                            updated: Utc::now(),
                            name: api_status.name,
                        }), 
                        workflowname: full_state.workflowname, 
                        secret: full_state.secret,
                        name: full_state.name,
                    };

                    job_state_list.insert(jobid.clone(), full_job_state);
                    println!("test6.1");
                }
            } else {
                let full_job_state = FullJobState {
                    api_status: Some(JobStatus {
                        id: jobid.clone(),
                        status: job.status.phase,
                        started: job.status.startedAt.unwrap(),
                        updated: Utc::now(),
                        name: name.clone(),
                    }), 
                    workflowname: Some(job.metadata.name.clone()), 
                    secret: job.metadata.name.clone(),
                    name: name.clone(),
                };
                job_state_list.insert(jobid.clone(), full_job_state);
                // let full_job_state2 = test.get_mut(&jobid).unwrap();
                // full_job_state2 = full_job_state;
                // println!("test7.122");
                // full_job_state2.api_status = full_job_state.api_status;
                // full_job_state2.workflowname = full_job_state.workflowname;
                // full_job_state2.secret = full_job_state.secret;
                // full_job_state2.name = full_job_state.name;
                println!("test7.2");
            }
        }
        Ok(())
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

        drop(write_state);

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
        // state_handler_clone.update_jobs().await;

        let s3_handler = S3Handler::new().await;

        PlasmidHunterModel
        {
            s3_handler,
            state_handler,
        }
    }
}