use clap::Parser;

/// Command-line arguments for configuring the Axum-based web application.
/// 
/// This struct uses the `clap` crate to parse various arguments passed to the application
/// and provides default values where necessary. It supports customization of the server's 
/// socket address, database connection parameters, and the size of the order cache.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CLIArgs {
    /// The socket address on which the web server listens for incoming requests.
    /// The default value is `127.0.0.1:3000`.
    #[arg(short, long, default_value_t = String::from("127.0.0.1:3000"))]
    pub socket_addr: String,

    /// The maximum size of the in-memory order cache. If the cache exceeds this limit,
    /// the application will persist the orders to the PostgreSQL database.
    /// The default value is `500`.
    #[arg(short, long, default_value_t = 500)]
    pub cache_size: usize,

    /// The hostname for the PostgreSQL database connection.
    #[arg(long)]
    pub host_name: String,

    /// The username for authenticating to the PostgreSQL database.
    #[arg(short, long)]
    pub user_name: String,

    /// The name of the PostgreSQL database to connect to.
    #[arg(short, long)]
    pub db_name: String,

    /// The password for authenticating to the PostgreSQL database.
    #[arg(short, long)]
    pub password: String
}
