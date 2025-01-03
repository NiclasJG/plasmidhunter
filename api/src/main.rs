use std::sync::Arc;

use api_model::PlasmidHunterModel;
use axum::{routing::post, Router
};

use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
mod api_structs;
mod api_handler;
mod api_model;
mod argo {
    pub mod client;
    mod urls;
    mod structs;
}
mod s3_helper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    //load .env for s3 variables
    dotenvy::dotenv()?;

    //0.0.0.0
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

    let app = Router::new().route("/", post(api_handler::create_job))
        .route("/api/v1/job/list", post(api_handler::get_job_list))
        .route("/api/v1/job/result", post(api_handler::get_job_result))
        .route("/api/v1/job/delete", post(api_handler::delete_job))
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











