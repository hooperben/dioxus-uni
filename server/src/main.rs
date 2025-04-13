use axum;
use clap::Parser;
use tower_http::cors::{Any, CorsLayer};

mod handlers;
mod helpers;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Port to listen on
    #[arg(long, default_value_t = 1337)]
    port: u16,
}

#[tokio::main]
async fn main() {
    // Parse command-line arguments
    let args = Args::parse();

    // Configure CORS middleware to allow all origins
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build our application with a single route and CORS middleware
    let app = axum::Router::new()
        .route("/estimate", axum::routing::get(handlers::estimate_handler))
        .layer(cors);

    // Run our application as a hyper server on the specified port
    let addr = format!("0.0.0.0:{}", args.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
