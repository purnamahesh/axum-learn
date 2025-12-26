use crate::routes::create_routes;
use std::error::Error;

mod routes;

pub async fn run() -> Result<(), Box<dyn Error>> {
    let app = create_routes().await?;

    let listner = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listner, app).await?;

    Ok(())
}
