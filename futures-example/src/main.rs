extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use futures::executor::block_on;
use futures_example::AppConfig;
use std::io;
use std::pin::Pin;
use std::time::Instant; // 1. Import Instant for high-precision timing

fn check_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn check_prime_boxed(
    n: u64,
) -> Pin<Box<dyn std::future::Future<Output = Result<bool, io::Error>>>> {
    Box::pin(futures::future::ok(check_prime(n)))
}

fn check_prime_impl_trait(n: u64) -> impl std::future::Future<Output = Result<bool, io::Error>> {
    futures::future::ok(check_prime(n))
}

fn main() {
    pretty_env_logger::init_timed();
    dotenvy::dotenv().ok();

    let appconfig = AppConfig::from_env();
    info!("Launching futures-example version: {}", appconfig.version);

    let input: u64 = 58466453;

    // -----------------------------------------------------------------
    // Timing the Boxed Future (Eagerly executed on Main Thread)
    // -----------------------------------------------------------------
    info!("Right before first call");
    let start_one = Instant::now();
    let res_one = check_prime_boxed(input);
    let duration_one = start_one.elapsed();
    info!("Called check_prime_boxed. Took: {:?}", duration_one);

    // -----------------------------------------------------------------
    // Timing the Impl Trait Future (Eagerly executed on Main Thread)
    // -----------------------------------------------------------------
    let start_two = Instant::now();
    let res_two = check_prime_impl_trait(input);
    let duration_two = start_two.elapsed();
    info!("Called check_prime_impl_trait. Took: {:?}", duration_two);

    info!(
        "Results are check_prime_boxed: {} and check_prime_impl_trait: {}",
        block_on(res_one).unwrap(),
        block_on(res_two).unwrap()
    );

    // -----------------------------------------------------------------
    // Timing the Async Worker Thread (Tokio spawn_blocking)
    // -----------------------------------------------------------------
    let rt = tokio::runtime::Runtime::new().unwrap();

    // We start a timer inside the thread closure to measure the actual background thread time
    let res_three = rt.spawn_blocking(move || {
        let thread_start = Instant::now();
        let is_prime = check_prime(input);
        let thread_duration = thread_start.elapsed();

        // Log directly from the worker thread context
        info!(
            "[Thread {:?}] Prime check finished inside background thread. Duration: {:?}",
            std::thread::current().id(),
            thread_duration
        );

        // Return both the calculation result and the duration tuple
        Ok::<_, std::io::Error>((is_prime, thread_duration))
    });

    info!("Called check_prime in another thread");

    // Main thread blocks waiting for the background thread to finish its work
    let (result, background_duration) = rt.block_on(res_three).unwrap().unwrap();

    info!(
        "Result from the last call: {} (Thread confirmed calculation took: {:?})",
        result, background_duration
    );
}
