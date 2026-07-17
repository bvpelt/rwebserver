use std::env;
use tonic::{transport::Server, Request, Response, Status};

// 1. Include the generated code
pub mod foobar;

// 2. Import the generated traits and structs
use foobar::foo_bar_service_client::FooBarServiceClient;
use foobar::foo_bar_service_server::{FooBarService, FooBarServiceServer};
use foobar::{
    Cab, CabLocationRequest, CabLocationResponse, GetCabRequest, GetCabResponse, Location,
};

// 3. Define your Server struct
#[derive(Debug, Default)]
pub struct MyFooBarService {}

// 4. Implement the generated server trait
#[tonic::async_trait]
impl FooBarService for MyFooBarService {
    async fn record_cab_location(
        &self,
        request: Request<CabLocationRequest>,
    ) -> Result<Response<CabLocationResponse>, Status> {
        println!("Server received cab location: {:?}", request.into_inner());

        // Return a dummy successful response
        let reply = CabLocationResponse { accepted: true };
        Ok(Response::new(reply))
    }

    async fn get_cabs(
        &self,
        request: Request<GetCabRequest>,
    ) -> Result<Response<GetCabResponse>, Status> {
        println!(
            "Server received get_cabs request: {:?}",
            request.into_inner()
        );

        // Return some dummy data
        let reply = GetCabResponse {
            cabs: vec![Cab {
                name: "Yellow Cab #42".into(),
                location: Some(Location {
                    lattitude: 52.02,
                    longituede: 5.55,
                }),
            }],
        };
        Ok(Response::new(reply))
    }
}

// 5. The Server Runner
async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = MyFooBarService::default();

    println!("gRPC Server listening on {}", addr);

    Server::builder()
        .add_service(FooBarServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}

// 6. The Client Runner
async fn run_client() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to server...");
    let mut client = FooBarServiceClient::connect("http://[::1]:50051").await?;

    let request = Request::new(CabLocationRequest {
        name: "Taxi 1".into(),
        location: Some(Location {
            lattitude: 52.37,
            longituede: 4.89,
        }),
    });

    println!("Sending request...");
    let response = client.record_cab_location(request).await?;

    println!("Got response: {:?}", response.into_inner());
    Ok(())
}

// 7. Entry Point
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"--server".to_string()) {
        run_server().await?;
    } else if args.contains(&"--client".to_string()) {
        run_client().await?;
    } else {
        println!("Please specify either --server or --client");
    }

    Ok(())
}
