use futures_chaining::AppConfig;
use log::{info, warn};
use rand::Rng;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    pretty_env_logger::init_timed();
    dotenvy::dotenv().ok();

    let appconfig = AppConfig::from_env();
    info!("Launching futures-example version: {}", appconfig.version);

    let start = Instant::now();
    let timeout = tokio::time::sleep(Duration::from_secs(3));

    // Shared storage for results
    let results = Arc::new(Mutex::new((None, None)));

    let one = {
        let results = results.clone();
        tokio::task::spawn_blocking(move || {
            let d = rand::thread_rng().gen_range(1..5);
            std::thread::sleep(Duration::from_secs(d));
            let duration = start.elapsed();
            let mut r = results.blocking_lock();
            r.0 = Some(("player_one", duration));
            ("player_one", duration)
        })
    };

    let two = {
        let results = results.clone();
        tokio::task::spawn_blocking(move || {
            let d = rand::thread_rng().gen_range(1..5);
            std::thread::sleep(Duration::from_secs(d));
            let duration = start.elapsed();
            let mut r = results.blocking_lock();
            r.1 = Some(("player_two", duration));
            ("player_two", duration)
        })
    };

    // FIX: Race all three conditions independently
    tokio::select! {
        // Condition 1: Player One finishes first
        Ok((p1, d1)) = one => {
            info!("{} won! (took: {:?})", p1, d1);
        },

        // Condition 2: Player Two finishes first
        Ok((p2, d2)) = two => {
            info!("{} won! (took: {:?})", p2, d2);
        },

        // Condition 3: The 3-second timeout is reached before EITHER finishes
        _ = timeout => {
            let r = results.lock().await;
            warn!("Timed out! Nobody finished in under 3 seconds.");
            warn!("Current status - {}: {:?}, {}: {:?}",
                r.0.as_ref().map_or("player_one", |(p, _)| *p),
                r.0.as_ref().map_or(Duration::from_secs(0), |(_, d)| *d),
                r.1.as_ref().map_or("player_two", |(p, _)| *p),
                r.1.as_ref().map_or(Duration::from_secs(0), |(_, d)| *d));
        },
    }
}
