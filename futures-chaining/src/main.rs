use futures::future::join;
use futures_chaining::AppConfig;
use log::{info, warn};
use rand::Rng;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    pretty_env_logger::init_timed();
    dotenvy::dotenv().ok();

    let appconfig = AppConfig::from_env();
    info!("Launching futures-example version: {}", appconfig.version);

    let start = Instant::now();
    let timeout_duration = Duration::from_secs(3);

    // FIX 1: Use tokio::spawn and tokio::time::sleep for true async cancellation
    let one = tokio::spawn(async move {
        let d = rand::thread_rng().gen_range(1..5);
        tokio::time::sleep(Duration::from_secs(d)).await;
        ("player_one", start.elapsed())
    });

    let two = tokio::spawn(async move {
        let d = rand::thread_rng().gen_range(1..5);
        tokio::time::sleep(Duration::from_secs(d)).await;
        ("player_two", start.elapsed())
    });

    let both = join(one, two);

    // FIX 2: Use tokio::time::timeout instead of select! for explicit timeouts
    match tokio::time::timeout(timeout_duration, both).await {
        Ok((result1, result2)) => {
            // This block executes ONLY if both players finish before 3 seconds
            let (p1, d1) = result1.unwrap();
            let (p2, d2) = result2.unwrap();
            info!("Player {:?} took {:?}", p1, d1);
            info!("Player {:?} took {:?}", p2, d2);
            if d1 < d2 {
                info!("{} won (took: {:?} vs {:?})", p1, d1, d2);
            } else {
                info!("{} won (took: {:?} vs {:?})", p2, d2, d1);
            }
        }
        Err(_) => {
            // This block strictly executes if 3 seconds is reached.
            // Because we used tokio::spawn, the pending player tasks are safely aborted.
            warn!("Timed out! Game cancelled.");
        }
    }
}
