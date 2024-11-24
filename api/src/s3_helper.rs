use aws_sdk_s3 as s3;
use std::time::Duration;
use aws_config::{load_from_env, meta::region::RegionProviderChain};
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::presigning::PresigningConfigError;
use aws_sdk_s3::{config::Region, meta::PKG_VERSION, Client};
use dotenvy::dotenv;
// use aws_sdk_s3::{config::Region, Client};
use uuid::Uuid;
use std::error::Error;

pub struct S3Handler {
    client: aws_sdk_s3::Client,
    bucket: String,
    // access_key: String,
    // secret_key: String,
    // bucket: String, 
    // endpoint: String,
    // is_ssl: bool,
}

// #[::tokio::main]
impl S3Handler {

    pub async fn new() -> Self {

        let myconfig = load_from_env().await;

    
        let s3_client = s3::Client::new(&myconfig);

        S3Handler {
            client: s3_client,
            bucket: dotenvy::var("S3_BUCKET").unwrap(),
        }
    }

    pub async fn presigned_get_url(&self, job_id: String) -> Result<String, Box<dyn Error>> {
        let expires_in = Duration::from_secs(5555); // max 7 days (a little bit more than 600000)
        let key = format!("jobs/{job_id}/result/result.json").to_string();

        let presigned_request = &self.client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .presigned(PresigningConfig::expires_in(expires_in)?)
            .await?;
        


        // println!("Object URI: {}", presigned_request.uri());
        // let valid_until = chrono::offset::Local::now() + expires_in;
        // println!("Valid until: {valid_until}");
    
        Ok(presigned_request.uri().to_string())
    }

    pub async fn presigned_post_url(&self, job_id: String ) -> Result<String, Box<dyn Error>> {
        let expires_in = Duration::from_secs(5555);
        let key = format!("jobs/{job_id}/input/plasmid.fasta").to_string();

        let presigned_request = &self.client
            .put_object()
            .content_type("text/plain")
            .bucket(&self.bucket)
            .key(key)
            .presigned(PresigningConfig::expires_in(expires_in)?)
            .await?;
                 
        Ok(presigned_request.uri().into())
    }
        
    
}

