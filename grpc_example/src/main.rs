use std::env;
use tonic::metadata::MetadataValue;
use tonic::{transport::Server, Request, Response, Status};

// 1. Include the generated code from your local src directory (Option 2)
pub mod foobar;

// 2. Import your EXACT generated traits and structs
use foobar::foo_bar_service_client::FooBarServiceClient;
use foobar::foo_bar_service_server::{FooBarService, FooBarServiceServer};
use foobar::{
    Cab, CabLocationRequest, CabLocationResponse, GetCabRequest, GetCabResponse, Location,
    LoginRequest, LoginResponse, StopServerRequest, StopServerResponse,
};

/// -----------------------------------------------------------------------
/// Security Domain Types
/// -----------------------------------------------------------------------
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Role {
    User,
    Admin,
}

/// The claims object that lives inside the Request Extensions if authenticated
#[derive(Clone, Debug)]
pub struct UserClaims {
    pub user_id: String,
    pub role: Role,
}

/// -----------------------------------------------------------------------
/// Global Authentication Interceptor
/// -----------------------------------------------------------------------
fn auth_interceptor(mut req: Request<()>) -> Result<Request<()>, Status> {
    if let Some(auth_header) = req.metadata().get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            // Mock validation matching incoming tokens
            if auth_str == "Bearer valid-user-token" {
                req.extensions_mut().insert(UserClaims {
                    user_id: "cab_driver_42".to_string(),
                    role: Role::User,
                });
            } else if auth_str == "Bearer valid-admin-token" {
                req.extensions_mut().insert(UserClaims {
                    user_id: "dispatcher_hq".to_string(),
                    role: Role::Admin,
                });
            } else {
                return Err(Status::unauthenticated("Invalid or expired access token"));
            }
        } else {
            return Err(Status::unauthenticated(
                "Authorization header contains invalid text",
            ));
        }
    }
    Ok(req)
}

/// -----------------------------------------------------------------------
/// Service Implementation with Authorization Guards
/// -----------------------------------------------------------------------
#[derive(Debug)]
pub struct MyFooBarService {
    // Add a channel sender to signal the main server loop
    pub shutdown_tx: tokio::sync::mpsc::Sender<()>,
}

#[tonic::async_trait]
impl FooBarService for MyFooBarService {
    // Public Endpoint: No claims required
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();

        // Match the expires_at field required by your proto specification
        if req.username == "driver" && req.password == "password123" {
            Ok(Response::new(LoginResponse {
                token: "Bearer valid-user-token".to_string(),
                expires_at: 1767225600, // Example future epoch timestamp
            }))
        } else if req.username == "admin" && req.password == "admin123" {
            Ok(Response::new(LoginResponse {
                token: "Bearer valid-admin-token".to_string(),
                expires_at: 1767225600,
            }))
        } else {
            Err(Status::unauthenticated("Invalid username or password"))
        }
    }

    // Protected Endpoint: Requires user authentication
    async fn record_cab_location(
        &self,
        request: Request<CabLocationRequest>,
    ) -> Result<Response<CabLocationResponse>, Status> {
        let claims = request.extensions().get::<UserClaims>().ok_or_else(|| {
            Status::unauthenticated("Authentication token required for this route")
        })?;

        println!(
            "[LOG] Location recorded by user '{}' ({:?})",
            claims.user_id, claims.role
        );

        Ok(Response::new(CabLocationResponse { accepted: true }))
    }

    // Protected Endpoint: Requires user authentication
    async fn get_cabs(
        &self,
        request: Request<GetCabRequest>,
    ) -> Result<Response<GetCabResponse>, Status> {
        let _claims = request.extensions().get::<UserClaims>().ok_or_else(|| {
            Status::unauthenticated("Authentication token required for this route")
        })?;

        // Return a response containing your Cab list matching the generated code fields
        Ok(Response::new(GetCabResponse {
            cabs: vec![Cab {
                name: "Yellow Cab #42".into(),
                location: Some(Location {
                    lattitude: 52.02,
                    longituede: 5.55,
                }),
            }],
        }))
    }

    // Administrative Endpoint: Requires specific Admin privileges
    // Administrative Endpoint: Requires specific Admin privileges
    async fn stop_server(
        &self,
        request: Request<StopServerRequest>,
    ) -> Result<Response<StopServerResponse>, Status> {
        // 1. Guard: Verify user is authenticated
        let claims = request.extensions().get::<UserClaims>().ok_or_else(|| {
            Status::unauthenticated("Authentication token required for this route")
        })?;

        // 2. Guard: Verify authorization level
        if claims.role != Role::Admin {
            return Err(Status::permission_denied(
                "Administrative authorization required",
            ));
        }

        let admin_id = claims.user_id.clone();
        let inner_msg = request.into_inner();

        println!(
            "[WARN] Shutdown initiated by administrator: {}. Reason: {}",
            admin_id, inner_msg.reason
        );

        // 3. Emit the shutdown token asynchronously
        let tx = self.shutdown_tx.clone();
        tokio::spawn(async move {
            let _ = tx.send(()).await;
        });

        // 4. Safely return the success message back to the client
        Ok(Response::new(StopServerResponse::default()))
    }
}

/// -----------------------------------------------------------------------
/// Entrypoint Runtime Control
/// -----------------------------------------------------------------------
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"--server".to_string()) {
        let addr = "[::1]:50051".parse()?;

        // Create a bounded multi-producer, single-consumer channel
        let (shutdown_tx, mut shutdown_rx) = tokio::sync::mpsc::channel::<()>(1);

        // Instantiate your service with the channel handle
        let my_service = MyFooBarService { shutdown_tx };
        let secured_svc = FooBarServiceServer::with_interceptor(my_service, auth_interceptor);

        println!("🛡️ Secure gRPC server listening on {}", addr);
        // Use serve_with_shutdown to handle clean request termination
        Server::builder()
            .add_service(secured_svc)
            .serve_with_shutdown(addr, async move {
                shutdown_rx.recv().await;
                println!("[Server] Shutdown token received. Draining connections and stopping...");
            })
            .await?;

        println!("[Server] Offline.");
    } else if args.contains(&"--client".to_string()) {
        let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
            .connect()
            .await?;

        // Create the base client for authentication
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

            let token_value = admin_login
                .into_inner()
                .token
                .parse::<MetadataValue<tonic::metadata::Ascii>>()?;
            let mut secure_admin_client =
                FooBarServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
                    req.metadata_mut()
                        .insert("authorization", token_value.clone());
                    Ok(req)
                });

            println!("[Client] Sending authorized stop command...");
            let stop_res = secure_admin_client
                .stop_server(StopServerRequest {
                    reason: "Remote maintenance shutdown".to_string(),
                })
                .await?;

            println!("[Client] Server response: {:?}", stop_res.into_inner());
            return Ok(()); // Early exit so we don't drop down into the driver flow
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
        println!("[Client] Login token acquired.");

        // Build intercepted client for driver metadata
        let token_value = token_received.parse::<MetadataValue<tonic::metadata::Ascii>>()?;
        let mut secure_driver_client =
            FooBarServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
                req.metadata_mut()
                    .insert("authorization", token_value.clone());
                Ok(req)
            });

        // Test sending a real structured message payload
        println!("[Client] Sending protected update payload...");
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
    } else {
        eprintln!("Usage Error: Use 'cargo run -- --server' or 'cargo run -- --client'");
    }

    Ok(())
}
