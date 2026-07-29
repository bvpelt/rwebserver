use futures_streams::AppConfig;
use log::info;

// 1. Bring in StreamExt to get access to `.for_each()`
use futures::stream::{Stream, StreamExt};
use rand::{Rng, thread_rng};
use std::pin::Pin;
use std::task::{Context, Poll}; // 2. Use the standard library's Poll and Context
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct CollatzStream {
    current: u64,
    end: u64,
}

impl CollatzStream {
    fn new(start: u64) -> CollatzStream {
        CollatzStream {
            current: start,
            end: 1,
        }
    }
}

impl Stream for CollatzStream {
    type Item = u64; // 3. The Error type is removed in modern Stream

    // 4. Update the signature to use Pin and Context
    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // 5. Update to rand 0.8 range syntax
        let d = thread_rng().gen_range(1..5);

        // Note: std::thread::sleep blocks the async executor thread.
        // In a real high-performance app, you would use async timers,
        // but this works perfectly for learning manual Stream implementation.
        thread::sleep(Duration::from_secs(d));

        if self.current % 2 == 0 {
            self.current = self.current / 2;
        } else {
            self.current = 3 * self.current + 1;
        }

        if self.current == self.end {
            Poll::Ready(None) // stream is finished when it reaches 1
        } else {
            Poll::Ready(Some(self.current))
        }
    }
}

// 6. Use the Tokio runtime and async main
#[tokio::main]
async fn main() {
    pretty_env_logger::init_timed();
    dotenvy::dotenv().ok();

    let appconfig = AppConfig::from_env();
    info!("Launching futures-example version: {}", appconfig.version);

    let stream = CollatzStream::new(10);

    // 7. Modern for_each takes an async closure that returns ()
    stream
        .for_each(|num| async move {
            info!("{}", num);
        })
        .await; // 8. Replace .wait() with .await
}
