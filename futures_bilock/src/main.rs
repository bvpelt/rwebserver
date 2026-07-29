use futures_bilock::AppConfig;
use log::{debug, error, info};
use rand::seq::SliceRandom;
use std::fmt::Debug;
use std::sync::Arc; // Needed to share Mutex across tasks
use tokio::sync::{mpsc, Mutex};

async fn sender(send: &Arc<Mutex<u64>>) -> &'static str {
    {
        // Acquire exclusive mutable lock
        let mut lock = send.lock().await;
        *lock += 1;
    } // Guard is dropped here

    let options = ["ping", "pong"];
    *options.choose(&mut rand::thread_rng()).unwrap()
}

async fn receiver<T: Debug>(mut recv: mpsc::Receiver<T>, recv_lock: Arc<Mutex<u64>>) {
    {
        let lock = recv_lock.lock().await;
        info!("Value of lock: {}", *lock);
    }

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

    // Wrap Mutex in an Arc for safe multi-task ownership
    let counter = Arc::new(Mutex::new(0u64));
    let (tx, rx) = mpsc::channel(100);

    // Clone the Arc for the sender task
    let counter_sender = Arc::clone(&counter);
    let h1 = tokio::spawn(async move {
        let msg = sender(&counter_sender).await;
        if let Err(e) = tx.send(msg).await {
            error!("Failed to send message: {}", e);
        }
        // tx is dropped here, signaling the receiver channel to close
        debug!("Sender ready, signal to receiver to close the channel");
    });

    // Clone (or move) the Arc for the receiver task
    let counter_receiver = Arc::clone(&counter);
    let h2 = tokio::spawn(async move {
        debug!("Start receiver");
        receiver(rx, counter_receiver).await;
        debug!("Stop  receiver");
    });

    let _ = tokio::join!(h1, h2);
    info!("Ready");
}
