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
/// The main function that runs the server. 
/// 
/// This function serves as the entry point of the application, where it:
/// - Initializes logging
/// - Parses command-line arguments using the `clap` crate to configure the server
/// - Sets up the application state, including a connection to PostgreSQL
/// - Configures Axum routes and starts the Axum web server.
///
/// # Steps
/// 1. **Initialize logging**: This step configures logging using the `log4rs` crate, loading the configuration from a YAML file.
/// 2. **Parse CLI arguments**: The `clap`-generated `CLIArgs` struct is used to handle command-line parameters, such as the socket address and database credentials.
/// 3. **Initialize app state**: An `AppState` struct is created, which includes the max capacity for caching orders and database client connections.
/// 4. **Set up Axum routes**: Axum routes are defined in a separate `routes` module, and the app's routes are registered to handle HTTP requests.
/// 5. **Start the Axum server**: The server is bound to the provided socket address and starts handling incoming requests.
///
/// # Panics
/// The function will panic if:
/// - The provided socket address is invalid.
/// - The server fails to start (e.g., port already in use).
#[tokio::main]
async fn main() {
    // Initialize logging from a configuration file
    init_logging();

    // Parse command-line arguments
    let args = CLIArgs::parse();  // CLIArgs struct is generated from clap to capture user input

    // Parse and validate the socket address
    let socket_addr: SocketAddr = args.socket_addr.parse()
        .expect("Invalid socket address");  // Exit if the address is malformed

    // Create the app state, including database connection and order queue
    let state = Arc::new(
        AppState::new(
            args.cache_size,  // The maximum capacity for the runtime order queue
            &args.host_name,  // Database host (e.g., localhost)
            &args.user_name,  // Database username
            &args.db_name,    // Database name
            &args.password    // Database password
        )
        .await
    );

    // Setup the Axum application with the routes and shared application state
    let app = Router::new()
        .merge(routes::handle_order())  // Register routes from the routes module
        .with_state(state);  // Attach the shared application state

    // Log that the server is starting and display the listening address
    info!("Listening on {}", socket_addr);

    // Bind the server to the socket address and start it
    axum_server::bind(socket_addr)
        .serve(app.into_make_service())  // Serve the app with Axum
        .await
        .expect("Failed to start server");  // Exit if the server fails to bind or start
}

/// 
/// Initializes logging for the application.
///
/// This function loads the logging configuration from a YAML file located at
/// `src/resources/logging/log_cfg.yaml`. The `log4rs` crate is used to configure 
/// logging, allowing different levels of log outputs such as error, info, debug, etc.
/// 
/// # Panics
/// If the logging configuration file cannot be found or loaded correctly, this function
/// will panic and the application will not start.
fn init_logging() {
    // Load the logging configuration from a file
    log4rs::init_file("src/resources/logging/log_cfg.yaml",
        Default::default()).unwrap();
}
