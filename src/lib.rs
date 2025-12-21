use crate::routes::create_routes;

mod routes;

pub async fn run() {
    let app = create_routes();

    let listner = tokio::net::TcpListener::bind("0.0.0.0:5065").await.unwrap();

    axum::serve(listner, app).await.unwrap();
}
