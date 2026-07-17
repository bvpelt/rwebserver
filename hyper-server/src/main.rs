extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use socket2::{Domain, Socket, Type};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::thread;
use tokio::net::TcpListener;
use tokio::time::{sleep, Duration};

async fn heavy_work() -> String {
    sleep(Duration::from_millis(100)).await;
    "done".to_string()
}

async fn handle_request(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    // Grab the current thread's name/ID so we can visually track load balancing in the logs
    let thread_id = format!("{:?}", thread::current().id());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/data") => {
            let body = heavy_work().await;
            info!("[Thread {}] processed /data successfully", thread_id);
            Ok(Response::new(Full::new(Bytes::from(body))))
        }
        _ => {
            let mut response = Response::new(Full::new(Bytes::new()));
            *response.status_mut() = StatusCode::NOT_FOUND;
            warn!("[Thread {}] path {} not found", thread_id, req.uri().path());
            Ok(response)
        }
    }
}

fn start_server(addr: SocketAddr, num_cores: usize) {
    let mut thread_handles = vec![];

    for core_id in 0..num_cores {
        // Spawn a dedicated native OS thread for each CPU core
        let handle = thread::spawn(move || {
            // Create a lightweight, single-threaded async runtime dedicated to this thread
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to build local Tokio runtime");

            rt.block_on(async move {
                // Configure the reuse_port socket exactly like the previous answer
                let socket =
                    Socket::new(Domain::IPV4, Type::STREAM, None).expect("Failed to create socket");

                socket.set_reuse_address(true).unwrap();

                #[cfg(not(windows))]
                socket.set_reuse_port(true).unwrap();

                socket.bind(&addr.into()).unwrap();
                socket.listen(128).unwrap();

                let std_listener: std::net::TcpListener = socket.into();
                std_listener.set_nonblocking(true).unwrap();

                let listener = TcpListener::from_std(std_listener).unwrap();

                info!(
                    "Worker thread for Core #{} is online and listening",
                    core_id
                );

                loop {
                    let (stream, _) = match listener.accept().await {
                        Ok(conn) => conn,
                        Err(e) => {
                            error!("Failed to accept connection on core #{}: {:?}", core_id, e);
                            continue;
                        }
                    };

                    let io = TokioIo::new(stream);

                    // Process concurrently within this thread's local event loop
                    tokio::task::spawn(async move {
                        if let Err(err) = http1::Builder::new()
                            .serve_connection(io, service_fn(handle_request))
                            .await
                        {
                            error!("Error serving connection: {:?}", err);
                        }
                    });
                }
            });
        });

        thread_handles.push(handle);
    }

    // Keep the main thread alive by waiting for all worker threads to finish
    for handle in thread_handles {
        let _ = handle.join();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init_timed();

    // Dynamically query available CPU cores via the standard library
    let num_cores = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!(
        "Master Process: Spawning server across {} cores...",
        num_cores
    );
    info!(
        "Master Process: Spawning server across {} cores...",
        num_cores
    );

    start_server(addr, num_cores);

    Ok(())
}
