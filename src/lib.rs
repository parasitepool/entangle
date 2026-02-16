#[cfg(feature = "api")]
pub mod api;
#[cfg(any(feature = "server", feature = "hydrate"))]
pub mod app;
#[cfg(feature = "server")]
pub mod server;
#[cfg(feature = "api")]
pub mod swap;

#[cfg(feature = "api")]
use clap::{Parser, Subcommand};

#[cfg(feature = "api")]
#[derive(Parser)]
#[command(version, author, about = "Swaps made easy")]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[cfg(feature = "api")]
#[derive(Subcommand)]
pub enum Command {
    /// Start the REST API server.
    Api {
        /// Address to bind the API server to.
        #[arg(long, default_value = "0.0.0.0:3000", env = "ENTANGLE_API_BIND")]
        bind: String,
    },
    /// Start the frontend web server.
    #[cfg(feature = "server")]
    Server {
        /// Address to bind the frontend server to.
        #[arg(long, default_value = "0.0.0.0:8080", env = "ENTANGLE_SERVER_BIND")]
        bind: String,
        /// Also serve the API routes under /api.
        #[arg(long)]
        with_api: bool,
    },
}

#[cfg(feature = "api")]
pub async fn main() {
    let args = Args::parse();

    match args.command {
        Command::Api { bind } => {
            api::serve(&bind).await;
        }
        #[cfg(feature = "server")]
        Command::Server { bind, with_api } => {
            server::serve(&bind, with_api).await;
        }
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(app::App);
}
