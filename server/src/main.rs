use axum::{extract::Query, response::Json};
use serde::{Deserialize, Serialize};

// Define a struct for our query parameters
#[derive(Deserialize)]
struct Params {
    pool: Option<String>,
    src: Option<String>,
    dst: Option<String>,
    src_amount: Option<String>,
}

// Define a struct for our JSON response
#[derive(Serialize)]
struct Response {
    pool: Option<String>,
    src: Option<String>,
    dst: Option<String>,
    src_amount: Option<String>,
}

#[tokio::main]
async fn main() {
    // Build our application with a single route.
    let app = axum::Router::new().route("/estimate", axum::routing::get(handler));

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Handler that extracts query parameters and returns JSON
async fn handler(Query(params): Query<Params>) -> Json<Response> {
    let response = Response {
        pool: params.pool,
        src: params.src,
        dst: params.dst,
        src_amount: params.src_amount,
    };

    // Return as JSON
    Json(response)
}
