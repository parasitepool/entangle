pub mod app;

#[cfg(feature = "ssr")]
pub mod api;
#[cfg(feature = "ssr")]
pub mod server;
#[cfg(feature = "ssr")]
pub mod swap;

#[cfg(feature = "ssr")]
use clap::{Parser, Subcommand};

#[cfg(feature = "ssr")]
#[derive(Parser)]
#[command(version, author, about = "Swaps made easy")]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[cfg(feature = "ssr")]
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

#[cfg(feature = "ssr")]
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

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(app::App);
}
