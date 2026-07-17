use grpc_example::foobar::foo_bar_service_client::FooBarServiceClient;
use grpc_example::foobar::{CabLocationRequest, Location, LoginRequest, StopServerRequest};
use grpc_example::AppConfig;
use tonic::metadata::MetadataValue;
use tonic::Request; // Added AppConfig

pub async fn run_client(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    // Generate target destination endpoint dynamically
    let config = AppConfig::from_env();
    let target_url = config.client_url();

    println!("[Client] Connecting to target gateway: {}", target_url);

    // Fix: Swap from_static for from_shared to accept a standard runtime String
    let channel = tonic::transport::Channel::from_shared(target_url)?
        .connect()
        .await?;

    let mut public_client = FooBarServiceClient::new(channel.clone());

    // -----------------------------------------------------------------
    // Mode A: Administrative Stop Flow
    // -----------------------------------------------------------------
    if args.contains(&"stop".to_string()) {
        println!("[Client] Attempting admin login exchange...");
        let admin_login = public_client
            .login(LoginRequest {
                username: "admin".to_string(),
                password: "admin123".to_string(),
            })
            .await?;

        let token_received = admin_login.into_inner().token;
        let token_value = token_received.parse::<MetadataValue<tonic::metadata::Ascii>>()?;

        let mut secure_admin_client =
            FooBarServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
                req.metadata_mut()
                    .insert("authorization", token_value.clone());
                Ok(req)
            });

        println!("[Client] Sending cryptographically authorized stop command...");
        let stop_res = secure_admin_client
            .stop_server(StopServerRequest {
                reason: "Remote maintenance shutdown".to_string(),
            })
            .await?;

        println!("[Client] Server response: {:?}", stop_res.into_inner());
        return Ok(());
    }

    // -----------------------------------------------------------------
    // Mode B: Regular Driver Location Update Flow
    // -----------------------------------------------------------------
    println!("[Client] Attempting driver login exchange...");
    let login_res = public_client
        .login(LoginRequest {
            username: "driver".to_string(),
            password: "password123".to_string(),
        })
        .await?;

    let token_received = login_res.into_inner().token;
    println!("[Client] JWT Token acquired.");

    let token_value = token_received.parse::<MetadataValue<tonic::metadata::Ascii>>()?;
    let mut secure_driver_client =
        FooBarServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
            req.metadata_mut()
                .insert("authorization", token_value.clone());
            Ok(req)
        });

    println!("[Client] Sending protected update payload using JWT headers...");
    let reply = secure_driver_client
        .record_cab_location(CabLocationRequest {
            name: "Taxi 1".to_string(),
            location: Some(Location {
                lattitude: 52.3702,
                longituede: 4.8952,
            }),
        })
        .await?;

    println!("[Client] Response received: {:?}", reply.into_inner());
    Ok(())
}
