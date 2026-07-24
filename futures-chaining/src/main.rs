use futures::future::join; // Add this import
use futures_chaining::AppConfig;
use log::info;
use rand::Rng;
use std::time::{Duration, Instant};
use tokio;

#[tokio::main]
async fn main() {
    pretty_env_logger::init_timed();
    dotenvy::dotenv().ok();

    let appconfig = AppConfig::from_env();
    info!("Launching futures-example version: {}", appconfig.version);

    let start = Instant::now();
    let timeout = tokio::time::sleep(Duration::from_secs(3));

    let one = tokio::task::spawn_blocking(move || {
        let d = rand::thread_rng().gen_range(1..5);
        std::thread::sleep(Duration::from_secs(d));
        ("player_one", start.elapsed())
    });
    let two = tokio::task::spawn_blocking(move || {
        let d = rand::thread_rng().gen_range(1..5);
        std::thread::sleep(Duration::from_secs(d));
        ("player_two", start.elapsed())
    });

    // Combine both futures into one
    let both = join(one, two);

    tokio::select! {
        _ = timeout => {
            info!("Timed out");
        },
        (result1, result2) = both => {
            let (p1, d1) = result1.unwrap();
            let (p2, d2) = result2.unwrap();
            info!("Player {:?} took {:?}", p1, d1);
            info!("Player {:?} took {:?}", p2, d2);
            if d1 < d2 {
                info!("{} won (took: {:?} vs {:?})", p1, d1, d2);
            } else {
                info!("{} won (took: {:?} vs {:?})", p2, d2, d1);
            }
        },
    }
}
