use futures_bilock::AppConfig;
use log::{debug, error, info};

use futures::lock::BiLock; // 1. Modern BiLock from futures 0.3
use rand::seq::SliceRandom;
use std::fmt::Debug;
use tokio::sync::mpsc; // 2. Modern Tokio MPSC channel

async fn sender(send: &BiLock<u64>) -> &'static str {
    // 3. Asynchronously acquire lock guard without blocking or polling
    {
        let mut lock = send.lock().await;
        *lock += 1;
    } // Lock is dropped here so the receiver can access it

    let options = ["ping", "pong"];
    *options.choose(&mut rand::thread_rng()).unwrap()
}

async fn receiver<T: Debug>(mut recv: mpsc::Receiver<T>, recv_lock: BiLock<u64>) {
    // Acquire lock and read shared counter
    {
        let lock = recv_lock.lock().await;
        info!("Value of lock: {}", *lock);
    }

    // Modern async loop for channel items
    while let Some(item) = recv.recv().await {
        info!("{:?}", item);
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init_timed();
    dotenvy::dotenv().ok();

    let appconfig = AppConfig::from_env();
    info!("Launching futures-example version: {}", appconfig.version);

    let counter = 0;

    // 4. In futures 0.3, `BiLock::bilock` creates the pair
    let (send, recv) = BiLock::new(counter);
    let (tx, rx) = mpsc::channel(100);

    // 5. Spawn Tokio async tasks instead of native threads
    let h1 = tokio::spawn(async move {
        let msg = sender(&send).await;
        if let Err(e) = tx.send(msg).await {
            error!("Failed to send message: {}", e);
        }
        debug!("Sender ready, signal to receiver to close the channel");
    });

    let h2 = tokio::spawn(async move {
        debug!("Start receiver");
        receiver(rx, recv).await;
        debug!("Stop  receiver");
    });

    // Wait for both tasks to complete
    let _ = tokio::join!(h1, h2);
}
