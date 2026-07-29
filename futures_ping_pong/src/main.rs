use futures_ping_pong::AppConfig;
use log::{debug, error, info};
use rand::seq::SliceRandom; // Required for .choose() in rand 0.8
use rand::{thread_rng, Rng};
use std::time::Duration;
use tokio::sync::mpsc; // Use Tokio's async mpsc channel

async fn sender() -> &'static str {
    // 1. ThreadRng is created and dropped immediately on this single line
    let d = thread_rng().gen_range(1..5);

    // 2. Safe to await! No ThreadRng exists in local scope anymore
    tokio::time::sleep(Duration::from_secs(d)).await;

    // 3. Re-acquire a thread_rng handle after the await point
    let options = ["ping", "pong"];
    let mut rng = thread_rng();
    *options.choose(&mut rng).unwrap()
}

async fn receiver(mut recv: mpsc::Receiver<&'static str>) {
    // Asynchronously yield items as they arrive on the channel
    while let Some(item) = recv.recv().await {
        info!("{:?}", item);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init_timed();
    dotenvy::dotenv().ok();

    let appconfig = AppConfig::from_env();
    info!("Launching futures-example version: {}", appconfig.version);

    // Bounded mpsc channel
    let (tx, rx) = mpsc::channel::<&'static str>(100);

    // Task 1: Sender task
    let h1 = tokio::spawn(async move {
        let msg = sender().await;
        if let Err(e) = tx.send(msg).await {
            error!("Failed to send message: {}", e);
        }
        // tx is dropped here, signaling the receiver channel to close
        debug!("Sender ready, signal to receiver to close the channel");
    });

    // Task 2: Receiver task
    let h2 = tokio::spawn(async move {
        debug!("Start receiver");
        receiver(rx).await;
        debug!("Stop  receiver");
    });

    // Wait for both Tokio tasks to finish concurrently
    let _ = tokio::join!(h1, h2);

    info!("Ready");
    Ok(())
}
