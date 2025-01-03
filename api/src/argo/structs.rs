use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use chrono::Utc;

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
    pub labels: String
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
    pub estimatedDuration: Option<i128>
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