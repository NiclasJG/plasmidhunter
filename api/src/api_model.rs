use std::{collections::HashMap, sync::Arc};

use chrono::Utc;
use tokio::sync::RwLock;
use reqwest::header::CONTENT_TYPE;
use anyhow::{Ok, Result, anyhow};

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

impl StateHandler{
    pub async fn update_jobs(&self) -> Result<()> {
        

        let job_list = self.argo_client.get_workflows().await.unwrap();
        let mut job_state_list = self.job_state.write().await;
        

        for job in job_list.items {
            // Option für bei variablen
            let mut jobid = Uuid::new_v4();
            let mut name = String::new();
            for param in job.spec.arguments.parameters {
                if param.name == "jobid" {
                    // crashes if value cant be parsed
                    jobid = Uuid::parse_str(&param.value).unwrap();
                } 
                if param.name == "parameter" {
                    name = String::from(param.value)
                }
            }
            
            //let else; Some() else
            let job_full_state = job_state_list.get(&jobid);

            if job_full_state.is_some() {
                let full_state = job_full_state.unwrap().clone();
                let api_status = full_state.api_status.unwrap().clone();

                // just update status and updated?
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
                } else {
                    //just update updated?
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
                }
            } else {
                let secret: &String;
                let string = &String::from("Unknown");

                if job.metadata.labels.get(&String::from("jobsecret")).is_some() {
                    secret = job.metadata.labels.get(&String::from("jobsecret")).unwrap();
                } else {
                    secret = string;
                }

                let full_job_state = FullJobState {
                    api_status: Some(JobStatus {
                        id: jobid.clone(),
                        status: job.status.phase,
                        started: job.status.startedAt.unwrap(),
                        updated: Utc::now(),
                        name: name.clone(),
                    }), 
                    workflowname: Some(job.metadata.name.clone()), 
                    secret: secret.to_string(),
                    name: name.clone(),
                };
                job_state_list.insert(jobid.clone(), full_job_state);
            }
        }
        drop(job_state_list);

        Ok(())
    } 

    pub async fn create_job(&self, name: &String) -> (Uuid, String) {
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

    pub async fn start_job(&self, name: &String, sequenz: &String, s3_post_url: &String, jobid: &Uuid, secret: &String ) -> Result<()> {
   
        let client = &self.argo_client.client;
        let s3_response = client.put(s3_post_url)
                        .header(CONTENT_TYPE, "text/plain")
                        // .danger_accept_invalid_hostnames()
                        .body(format!("{name}\n{sequenz}").to_string())
                        .send()
                        .await;

        let argo_response = self.argo_client.start_workflow(&jobid.to_string(), name, secret).await?;

        let job_status = JobStatus{
          id: jobid.clone(),
          status: String::from("Started"),
        //   started: argo_response.metadata.creationTimestamp.unwrap(),
          started: Utc::now(),
          updated: Utc::now(),
          name: name.clone()
        };

        let mut write_state = self.job_state.write().await;

        let full_job_state = write_state.get_mut(jobid).unwrap();

        full_job_state.api_status = Some(job_status);
        full_job_state.workflowname = Some(argo_response.metadata.name);

        drop(write_state);

        Ok(())
    }

    pub async fn get_jobs(&self, job_list: Vec<Job>) -> Result<ListRespone> {
        
        let mut jobs = vec![]; 

        let read_state = self.job_state.read().await;

        for job in job_list {
            
            let job_api_status = read_state.get(&job.id);
            if job_api_status.is_some() {
                if job_api_status.unwrap().secret == job.secret {
                    jobs.push(job_api_status.unwrap().api_status.as_ref().unwrap().clone())
                }
            }
            
        };
        
        Ok(ListRespone{jobs})
    }

    pub async fn get_job_result(&self, job: Job, result_url: String) -> Result<ResultResponse> {
        
        let read_state = self.job_state.read().await;

        let job_api_status = read_state.get(&job.id);
        //secret check
        if job_api_status.is_some() {
            if job_api_status.unwrap().secret == job.secret{
                return Ok( ResultResponse {
                name: job_api_status.unwrap().name.clone(),
                data: result_url
            })
            }
        }

        Err(anyhow!("Job not found"))
    }

    pub async fn delete_job(&self, job: Job) -> Result<()>{
        let read_state = self.job_state.read().await;

        let job_api_status = read_state.get(&job.id);

        // delete job from state_handler hashmap?

        if job_api_status.is_some() {
            if job_api_status.unwrap().secret == job.secret {
                let response = self.argo_client
                .delete_workflow(job_api_status.unwrap().workflowname.as_ref().unwrap().to_string())
                .await;
            }
        }

        Ok(())
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

        let s3_handler = S3Handler::new().await;

        let init_state = state_handler.update_jobs().await;

        PlasmidHunterModel
        {
            s3_handler,
            state_handler,
        }
    }
}