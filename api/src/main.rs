use std::sync::Arc;

use api_model::PlasmidHunterModel;
use axum::{
    extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{get, post}, Error, Json, Router
};

use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

// use crate::api_model::ph_db;
mod api_structs;
mod api_handler;
mod api_model;
mod argo {
    pub mod client;
    mod urls;
}
mod s3_helper;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    //load .env for s3 variables
    dotenvy::dotenv()?;

    let server_address = "127.0.0.1:1239";
    
    let listener = TcpListener::bind(server_address)
    .await
    .expect("Couldnt create TCP listener");

    let plasmid_hunter_model = Arc::new(PlasmidHunterModel::new(
        dotenvy::var("ARGO_TOKEN")?,
        dotenvy::var("ARGO_URL")?,
        dotenvy::var("ARGO_NAMESPACE")?,
        dotenvy::var("PLASMIDHUNTER_VERSION")?,
        dotenvy::var("DATABASE_VERSION")?,
        dotenvy::var("BACKEND_VERSION")?,
    ).await,);

    println!("listening on {}", listener.local_addr().unwrap());

    let app = Router::new().route("/", get(|| async {"Helllo"}).post(api_handler::create_job))
        // .route("/api/v1/job/init", get(||async{"Hello"}))
        .route("/api/v1/job/list", post(api_handler::get_job_list))
        .route("/api/v1/job/result", post(api_handler::get_job_result))
        // .route("/about", get(|| async {"Helllo"}))
        .with_state(plasmid_hunter_model)
        .layer(CorsLayer::very_permissive())
        .layer(TraceLayer::new_for_http()
            .on_response(())
            .on_body_chunk(())
            .on_eos(()));

    axum::serve(listener, app)
        .await
        .expect("Error serving application");
    Ok(())
}











