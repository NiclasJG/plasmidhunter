pub fn get_submit_url (url: String, namespace: String) -> String {
   
        format!("{url}/api/v1/workflows/{namespace}/submit")
}

pub fn get_workflows_url (url: String, namespace: String) -> String {
        format!("{url}/api/v1/workflows/{namespace}?fields=items.metadata.name,items.metadata.labels,items.status.startedAt,items.status.finishedAt,items.status.phase,items.spec.arguments,items.status.estimatedDuration")
}

pub fn get_delete_url (url: String, namespace: String, name: String) -> String {
        format!("{url}/api/v1/workflows/{namespace}/{name}")
}