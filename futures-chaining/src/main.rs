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
    let timeout = tokio::time::sleep(Duration::from_secs(3));

    // Define the tasks as standard async blocks, NOT spawned threads.
    let one = async {
        let d = rand::thread_rng().gen_range(1..5);
        tokio::time::sleep(Duration::from_secs(d)).await; // True async sleep
        ("player_one", start.elapsed())
    };

    let two = async {
        let d = rand::thread_rng().gen_range(1..5);
        tokio::time::sleep(Duration::from_secs(d)).await; // True async sleep
        ("player_two", start.elapsed())
    };

    // FIX: Add 'biased;' and put the timeout first.
    // Tokio will now check the timeout condition BEFORE checking the players.
    tokio::select! {
        biased; // <--- The magic keyword

        // 1. Check timeout first
        _ = timeout => {
            warn!("Timed out! Nobody finished in under 3 seconds.");
        },

        // 2. Check player one second
        (p1, d1) = one => {
            info!("{} won! (took: {:?})", p1, d1);
        },

        // 3. Check player two third
        (p2, d2) = two => {
            info!("{} won! (took: {:?})", p2, d2);
        },
    }
}
