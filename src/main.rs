mod state;
mod order;
mod routes;
mod cli;

use axum::Router;
use std::sync::Arc;
use cli::CLIArgs;
use state::AppState;
use log::info;
use std::net::SocketAddr;
use clap::Parser;

///
/// The main function.
/// Initializes logging, parses CLI arguments and start the Axum server
/// 
#[tokio::main]
async fn main() {
    // Initialize logging
    init_logging();

    // Parse command-line arguments
    let args = CLIArgs::parse();
    let socket_addr: SocketAddr = args.socket_addr.parse()
        .expect("Invalid socket address");

    // Create the app state 
    let state = Arc::new(
        AppState::new(
            args.cache_size,   // max capacity of runtime queue
            &args.host_name,            // db hostname (localhost)
            &args.user_name,  // db username 
            &args.db_name,     // db name 
            &args.password            // db password
        )
        .await
    );

    // Setup the Axum app
    let app = Router::new()
        .merge(routes::handle_order())
        .with_state(state);

    info!("Listening on {}", socket_addr);

    // Bind the Axum server
    axum_server::bind(socket_addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

/// 
/// Initializes logging via config file
/// 
fn init_logging() {
    log4rs::init_file("src/resources/logging/log_cfg.yaml",
        Default::default()).unwrap();
}
