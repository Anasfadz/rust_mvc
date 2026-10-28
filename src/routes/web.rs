use axum::{routing::get, Router};
use crate::controller::base_controller;
use crate::controller::test_controller;

pub fn routes() -> Router {
    Router::new()
    .route("/", get(base_controller::index))
    .route("/test", get(test_controller::index))
    .route("/create-student", get(test_controller::create_student))

    .route("/test-anas", get(base_controller::test_anas))
} 