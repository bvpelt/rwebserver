extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::time::{sleep, Duration};

async fn heavy_work() -> String {
    sleep(Duration::from_millis(100)).await;
    "done".to_string()
}

// Pulled the routing logic into its own function for cleaner task spawning
async fn handle_request(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/data") => {
            let body = heavy_work().await;
            info!("server working and responding");
            // In 1.x, bodies must be wrapped in http_body_util types like Full
            Ok(Response::new(Full::new(Bytes::from(body))))
        }
        _ => {
            let mut response = Response::new(Full::new(Bytes::new()));
            *response.status_mut() = StatusCode::NOT_FOUND;
            warn!("path {} not found", req.uri().path());
            Ok(response)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize the logger based on the RUST_LOG environment variable
    pretty_env_logger::init_timed();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    // 1. Manually bind the TCP listener using Tokio
    let listener = TcpListener::bind(addr).await?;
    println!("Server running on http://{}", addr);
    info!("Server running on http://{}", addr);

    // 2. Accept connections in a continuous loop
    loop {
        let (stream, _) = listener.accept().await?;

        // 3. Wrap the raw Tokio stream into a Hyper-compatible IO trait
        let io = TokioIo::new(stream);

        // 4. Spawn a concurrent Tokio task to serve this specific connection
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(handle_request))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
                error!("Error serving connection: {:?}", err);
            }
        });
    }
}
