mod app_settings;
mod azure;
mod cli;
mod git;
mod app;

#[tokio::main]
async fn main() {
    if let Err(e) = app::run().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
