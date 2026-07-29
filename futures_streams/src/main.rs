use futures::stream::{Stream, StreamExt};
use futures_streams::AppConfig;
use log::{debug, info};
use rand::{Rng, thread_rng};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::time::{Sleep, sleep};

struct CollatzStream {
    current: u64,
    end: u64,
    // Store an active timer future directly inside the stream struct
    delay: Option<Pin<Box<Sleep>>>,
}

impl CollatzStream {
    fn new(start: u64) -> CollatzStream {
        CollatzStream {
            current: start,
            end: 1,
            delay: None,
        }
    }
}

impl Stream for CollatzStream {
    type Item = u64;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // 1. If we don't have an active delay timer, create one
        if self.delay.is_none() {
            let d = thread_rng().gen_range(1..5);
            self.delay = Some(Box::pin(sleep(Duration::from_secs(d))));
            debug!("delay: {:?}", self.delay);
        }

        // 2. Poll the inner delay timer asynchronously!
        if let Some(delay) = &mut self.delay {
            // Asynchronously poll the Tokio sleep timer
            if delay.as_mut().poll(cx).is_pending() {
                debug!("pending");
                // Timer is still counting down; yield execution back to Tokio worker
                return Poll::Pending;
            }
        }

        // 3. Timer has finished! Reset delay so a new duration is picked next iteration
        self.delay = None;

        info!("current: {}", self.current);

        // 4. Calculate the Collatz step
        if self.current % 2 == 0 {
            self.current /= 2;
        } else {
            self.current = 3 * self.current + 1;
        }

        if self.current == self.end {
            Poll::Ready(None)
        } else {
            Poll::Ready(Some(self.current))
        }
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init_timed();
    dotenvy::dotenv().ok();

    let appconfig = AppConfig::from_env();
    info!("Launching futures-example version: {}", appconfig.version);

    let stream = CollatzStream::new(10);

    stream
        .for_each(|num| async move {
            info!("{}", num);
        })
        .await;
}
