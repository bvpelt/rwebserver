use std::env;

mod client;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from the .env file if present
    dotenvy::dotenv().ok();

    let args: Vec<String> = env::args().collect();

    if args.contains(&"--server".to_string()) {
        server::run_server().await?;
    } else if args.contains(&"--client".to_string()) {
        client::run_client(&args).await?;
    } else {
        eprintln!("Usage Error: Use 'cargo run -- --server' or 'cargo run -- --client'");
    }

    Ok(())
}
