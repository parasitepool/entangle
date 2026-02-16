pub mod api;
pub mod server;
pub mod swap;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, author, about = "Swaps made easy")]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Start the REST API server.
    Api {
        /// Address to bind the API server to.
        #[arg(long, default_value = "0.0.0.0:3000", env = "ENTANGLE_API_BIND")]
        bind: String,
    },
    /// Start the frontend web server.
    Server {
        /// Address to bind the frontend server to.
        #[arg(long, default_value = "0.0.0.0:8080", env = "ENTANGLE_SERVER_BIND")]
        bind: String,
        /// Also serve the API routes under /api.
        #[arg(long)]
        with_api: bool,
    },
}

pub async fn main() {
    let args = Args::parse();

    match args.command {
        Command::Api { bind } => {
            api::serve(&bind).await;
        }
        Command::Server { bind, with_api } => {
            server::serve(&bind, with_api).await;
        }
    }
}
