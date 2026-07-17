use grpc_example::foobar::foo_bar_service_server::FooBarServiceServer;
use grpc_example::{auth_interceptor, AppConfig, MyFooBarService};
use tonic::transport::Server; // Added AppConfig

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    // Dynamically retrieve configured address properties
    let config = AppConfig::from_env();
    let addr = config.server_addr()?;

    let (shutdown_tx, mut shutdown_rx) = tokio::sync::mpsc::channel::<()>(1);

    let my_service = MyFooBarService { shutdown_tx };
    let secured_svc = FooBarServiceServer::with_interceptor(my_service, auth_interceptor);

    println!("🛡️ Secure gRPC server listening on {}", addr);
    Server::builder()
        .add_service(secured_svc)
        .serve_with_shutdown(addr, async move {
            shutdown_rx.recv().await;
            println!("[Server] Shutdown token received. Draining connections and stopping...");
        })
        .await?;

    println!("[Server] Offline.");
    Ok(())
}
