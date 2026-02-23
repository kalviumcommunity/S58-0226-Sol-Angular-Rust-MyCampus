use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ApiMessage {
    message: String,
}

#[derive(Deserialize)]
struct CreateProductRequest {
    name: String,
    quantity: i32,
}

#[derive(Serialize)]
struct CreateProductResponse {
    id: i64,
    name: String,
    quantity: i32,
    status: String,
}

async fn health() -> Json<ApiMessage> {
    Json(ApiMessage {
        message: "OK".to_string(),
    })
}

async fn create_product(Json(payload): Json<CreateProductRequest>) -> Json<CreateProductResponse> {
    // Concept 2: simulate DB insert (real Postgres comes in later concepts)
    Json(CreateProductResponse {
        id: 1,
        name: payload.name,
        quantity: payload.quantity,
        status: "created".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/products", post(create_product));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind port 3000");

    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.expect("Server error");
}