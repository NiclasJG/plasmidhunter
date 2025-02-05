use anyhow::{Ok, Result};

use reqwest::Client;

use crate::argo::urls as urls;
use crate::argo::structs::*;

pub struct ArgoClient {
    pub token: String,
    pub url: String,
    pub namespace: String,
    pub client: Client,
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

    pub async fn start_workflow(&self, jobid: &String, name: &String, secret: &String) -> Result<WorkflowStartData> {
        
        let jobid_param = format!("jobid={jobid}");
        // let name_param = format!("parameter={name}");
        let secret_label = format!("jobsecret={secret}");

        let parameters_list = Vec::from([jobid_param]);

        let submit_options = SubmitOptions{
            parameters: parameters_list,
            labels: secret_label
        };

        let json_data = WorkflowInit{
            namespace: self.namespace.to_string(),
            resourceKind: "WorkflowTemplate".to_string(),
            resourceName: "plasmidhunter-0.4.0".to_string(),
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
                                    
        Ok(response)
    }

    pub async fn delete_workflow(&self, workflowname: String) -> Result<()> {
        let delete_workflow_url = urls::get_delete_url(self.url.to_string(), self.namespace.to_string(), workflowname);

        let response = self.client
                                .delete(delete_workflow_url)
                                .header("Authorization", &self.token)
                                .send()
                                .await;
        
        Ok(())
    }
}