pub fn get_status_url (url: String, namespace: String) -> String {
   
        format!("{url}/api/v1/workflows/{namespace}?field=items.status.finishedAt,items.status.startedAt,items.metadata.name,items.status.phase,items.metadata.labels")
    
}

