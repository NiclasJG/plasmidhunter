use std::{collections::HashMap, fs::Metadata};
use anyhow::{Ok, Result};

use axum::response;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use chrono::Utc;
use serde_json::json;

use crate::argo::urls as urls;

pub struct ArgoClient {
    pub token: String,
    pub url: String,
    pub namespace: String,
    pub client: Client,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WorkflowInit {
    pub namespace: String,
    pub resourceKind: String,
    pub resourceName: String,
    pub submitOptions: SubmitOptions,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SubmitOptions {
    pub parameters: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WorkflowsList {
    pub items: Vec<WorkflowData>,
    pub metadata: Option<MetaData>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MetaData {
    pub resourceVersion: Option<String>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WorkflowData {
    pub metadata: WorkflowMetaData,
    pub status: WorkflowStatus,
    pub spec: Spec
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Spec {
    pub arguments: Arguments
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Arguments {
    pub parameters: Vec<Parameters>,
    pub workflowTemplateRef: Option<WorkflowTemplateRef>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WorkflowTemplateRef {
    pub name: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Parameters {
    pub name: String,
    pub value: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WorkflowMetaData {
    pub labels: HashMap<String,String>,
    pub name: String,
    pub creationTimestamp: Option<chrono::DateTime<Utc>> 
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WorkflowStatus {
    pub finishedAt: Option<chrono::DateTime<Utc>>,
    pub startedAt: Option<chrono::DateTime<Utc>>,
    pub phase: String,
    pub estimatedDuration: Option<String>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WorkflowStartData {
    pub metadata: WorkflowMetaData,
    pub status: WorkflowStartStatus,
    pub spec: Spec
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WorkflowStartStatus {
    pub finishedAt: Option<String>,
    pub startedAt: Option<String>
}

impl ArgoClient {
    pub fn new(token: String, url: String, namespace: String) -> ArgoClient {
        ArgoClient {
            token,
            url,
            namespace,
            // reqwst::Client::new(), is enough if invalid_certs is not required (insecure https)
            client: reqwest::Client::builder().danger_accept_invalid_certs(true).build().unwrap()
        }
    }

    pub async fn start_workflow(&self, jobid: &String, name: &String) -> Result<WorkflowStartData> {
        
        let jobid_param = format!("jobid={jobid}");
        let name_param = format!("parameter={name}");

        let parameters_list = Vec::from([name_param, jobid_param]);

        let submit_options = SubmitOptions{
            parameters: parameters_list,
        };

        let json_data = WorkflowInit{
            namespace: self.namespace.to_string(),
            resourceKind: "WorkflowTemplate".to_string(),
            resourceName: "testworkflow".to_string(),
            submitOptions: submit_options,
        };

        let submit_url = urls::get_submit_url(self.url.to_string(), self.namespace.to_string());

        

        let response = self.client
                                            .post(submit_url)
                                            .header("Authorization", &self.token)
                                            .json(&json_data)
                                            .send()
                                            .await?
                                            .json::<WorkflowStartData>()
                                            .await?;

        // println!("{:?}", response.metadata.name);
        Ok(response)
    }

    pub async fn get_workflows(&self) -> Result<WorkflowsList> {
        let get_workflows_url = urls::get_workflows_url(self.url.to_string(), self.namespace.to_string());

        let response = self.client
                                    .get(get_workflows_url)
                                    .header("Authorization", &self.token)
                                    .send()
                                    .await?
                                    .json::<WorkflowsList>()
                                    .await?;
                                    
        println!("First Result: {:?}", json!(response));
        Ok(response)
        // Ok(())
    }

    pub async fn delete_workflow(&self, name: String) {
        let delete_workflow_url = urls::get_delete_url(self.url.to_string(), self.namespace.to_string(), name);

        let response = self.client
                                .delete(delete_workflow_url)
                                .header("Authorization", &self.token)
                                .send()
                                .await;
    }
}