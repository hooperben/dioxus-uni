use axum;

mod handlers;
mod helpers;

#[tokio::main]
async fn main() {
    // Build our application with a single route.
    let app =
        axum::Router::new().route("/estimate", axum::routing::get(handlers::estimate_handler));

    // Run our application as a hyper server on http://localhost:1337.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:1337").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
