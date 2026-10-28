mod config;
mod routes;
mod controller;
mod model;

use routes::web;

#[tokio::main]
async fn main() {
    config::db::connect().await.unwrap();
    println!("Database connected");

    let app = web::routes();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8090")
        .await
        .unwrap();
    println!("Server running on http://127.0.0.1:8090");
    axum::serve(listener, app).await.unwrap();

}