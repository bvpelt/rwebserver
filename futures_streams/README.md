# Streams

A common pattern for steams is synchronisation
- components need to communicate with one another
- channel implementation in the standard library is not asynchronous
- futures has its own channel implementation
- provides all the guarantees

## Running

```bash
$ RUST_LOG=debug cargo run
   Compiling futures_streams v0.1.0 (/home/bvpelt/Develop/rwebserver/futures_streams)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `/home/bvpelt/Develop/rwebserver/target/debug/futures_streams`
 2026-07-29T14:41:23.823Z INFO  futures_streams > Launching futures-example version: 0.1
 2026-07-29T14:41:23.823Z DEBUG futures_streams > delay: Some(Sleep { deadline: Instant { tv_sec: 4898, tv_nsec: 353695473 }, driver: MultiThread(multi_thread::Handle { ... }), inner: Inner, timer: None })
 2026-07-29T14:41:23.823Z DEBUG futures_streams > pending
 2026-07-29T14:41:26.825Z INFO  futures_streams > current: 10
 2026-07-29T14:41:26.825Z INFO  futures_streams > 5
 2026-07-29T14:41:26.825Z DEBUG futures_streams > delay: Some(Sleep { deadline: Instant { tv_sec: 4899, tv_nsec: 355322540 }, driver: MultiThread(multi_thread::Handle { ... }), inner: Inner, timer: None })
 2026-07-29T14:41:26.825Z DEBUG futures_streams > pending
 2026-07-29T14:41:27.826Z INFO  futures_streams > current: 5
 2026-07-29T14:41:27.826Z INFO  futures_streams > 16
 2026-07-29T14:41:27.826Z DEBUG futures_streams > delay: Some(Sleep { deadline: Instant { tv_sec: 4901, tv_nsec: 356762177 }, driver: MultiThread(multi_thread::Handle { ... }), inner: Inner, timer: None })
 2026-07-29T14:41:27.826Z DEBUG futures_streams > pending
 2026-07-29T14:41:29.827Z INFO  futures_streams > current: 16
 2026-07-29T14:41:29.827Z INFO  futures_streams > 8
 2026-07-29T14:41:29.827Z DEBUG futures_streams > delay: Some(Sleep { deadline: Instant { tv_sec: 4903, tv_nsec: 357397674 }, driver: MultiThread(multi_thread::Handle { ... }), inner: Inner, timer: None })
 2026-07-29T14:41:29.827Z DEBUG futures_streams > pending
 2026-07-29T14:41:31.828Z INFO  futures_streams > current: 8
 2026-07-29T14:41:31.829Z INFO  futures_streams > 4
 2026-07-29T14:41:31.829Z DEBUG futures_streams > delay: Some(Sleep { deadline: Instant { tv_sec: 4906, tv_nsec: 358942043 }, driver: MultiThread(multi_thread::Handle { ... }), inner: Inner, timer: None })
 2026-07-29T14:41:31.829Z DEBUG futures_streams > pending
 2026-07-29T14:41:34.830Z INFO  futures_streams > current: 4
 2026-07-29T14:41:34.830Z INFO  futures_streams > 2
 2026-07-29T14:41:34.830Z DEBUG futures_streams > delay: Some(Sleep { deadline: Instant { tv_sec: 4910, tv_nsec: 360719025 }, driver: MultiThread(multi_thread::Handle { ... }), inner: Inner, timer: None })
 2026-07-29T14:41:34.830Z DEBUG futures_streams > pending
 2026-07-29T14:41:38.831Z INFO  futures_streams > current: 2
```


## Production implementation
The code contains an example how to manually implement the streams.

Currently most production software uses the approach below.

```toml
[dependencies]
async-stream = "0.3"
```

```rust
use async_stream::stream;
use futures::stream::{Stream, StreamExt};
use futures_streams::AppConfig;
use log::info;
use rand::{thread_rng, Rng};
use std::time::Duration;

fn produce_collatz(mut current: u64) -> impl Stream<Item = u64> {
    // The stream! macro automatically converts async/yield blocks into a valid Stream
    stream! {
        let end = 1;
        while current != end {
            let d = thread_rng().gen_range(1..5);
            
            // Non-blocking async sleep - Tokio can process other tasks while waiting!
            tokio::time::sleep(Duration::from_secs(d)).await;

            if current % 2 == 0 {
                current /= 2;
            } else {
                current = 3 * current + 1;
            }

            yield current;
        }
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init_timed();
    dotenvy::dotenv().ok();

    let appconfig = AppConfig::from_env();
    info!("Launching futures-example version: {}", appconfig.version);

    let stream = produce_collatz(10);

    stream
        .for_each(|num| async move {
            info!("{}", num);
        })
        .await;
}
```