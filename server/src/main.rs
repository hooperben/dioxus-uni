use axum;
use clap::Parser;

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

    // Build our application with a single route.
    let app =
        axum::Router::new().route("/estimate", axum::routing::get(handlers::estimate_handler));

    // Run our application as a hyper server on the specified port
    let addr = format!("0.0.0.0:{}", args.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
