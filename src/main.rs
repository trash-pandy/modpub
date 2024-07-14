use anyhow::Result;
use axum::{routing, Router};
use clap::Parser;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = 3000)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let Cli { port } = Cli::parse();

    let app = Router::new().route("/", routing::get(root));

    let listener = tokio::net::TcpListener::bind(("127.0.0.1", port)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Modpub says hello!"
}
