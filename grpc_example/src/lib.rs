use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
pub use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};
use tonic::{Request, Response, Status};

// Include the generated code from your local src directory
pub mod foobar;

use foobar::foo_bar_service_server::FooBarService;
use foobar::{
    Cab, CabLocationRequest, CabLocationResponse, GetCabRequest, GetCabResponse, Location,
    LoginRequest, LoginResponse, StopServerRequest, StopServerResponse,
};

pub const JWT_SECRET: &[u8] = b"super_secret_routing_key_123456789";

/// -----------------------------------------------------------------------
/// Security Domain & JWT Payload Structures
/// -----------------------------------------------------------------------
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Role {
    User,
    Admin,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: Role,
    pub exp: u64,
}

#[derive(Clone, Debug)]
pub struct UserClaims {
    pub user_id: String,
    pub role: Role,
}

/// -----------------------------------------------------------------------
/// Application Configuration Central Hierarchy
/// -----------------------------------------------------------------------
pub struct AppConfig {
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = std::env::var("SERVER_PORT")
            .unwrap_or_else(|_| "50051".to_string())
            .parse::<u16>()
            .unwrap_or(50051);

        Self { host, port }
    }

    /// Transforms config into the SocketAddr structure required by tonic's Server builder
    pub fn server_addr(&self) -> Result<SocketAddr, std::net::AddrParseError> {
        format!("{}:{}", self.host, self.port).parse()
    }

    /// Formats the host and port into a valid HTTP URL string for the gRPC Channel client
    pub fn client_url(&self) -> String {
        // Automatically wrap bare IPv6 addresses (containing ':') in brackets
        if self.host.contains(':') && !self.host.starts_with('[') {
            format!("http://[{}]:{}", self.host, self.port)
        } else {
            format!("http://{}:{}", self.host, self.port)
        }
    }
}

/// -----------------------------------------------------------------------
/// Global Authentication Interceptor (Token Validation)
/// -----------------------------------------------------------------------
pub fn auth_interceptor(mut req: Request<()>) -> Result<Request<()>, Status> {
    if let Some(auth_header) = req.metadata().get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            println!("received auth_str: {:?}", auth_str);

            let token = match auth_str.strip_prefix("Bearer ") {
                Some(t) => t.trim(),
                None => {
                    return Err(Status::unauthenticated(
                        "Malformed authorization header format",
                    ))
                }
            };

            let validation = Validation::new(Algorithm::HS256);
            match decode::<Claims>(token, &DecodingKey::from_secret(JWT_SECRET), &validation) {
                Ok(token_data) => {
                    println!("token_data: {:?}", token_data);
                    req.extensions_mut().insert(UserClaims {
                        user_id: token_data.claims.sub,
                        role: token_data.claims.role,
                    });
                }
                Err(err) => {
                    let err_msg = format!("Token verification failed: {}", err);
                    return Err(Status::unauthenticated(err_msg));
                }
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
/// Service Implementation
/// -----------------------------------------------------------------------
#[derive(Debug)]
pub struct MyFooBarService {
    pub shutdown_tx: tokio::sync::mpsc::Sender<()>,
}

#[tonic::async_trait]
impl FooBarService for MyFooBarService {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();

        let determined_role = if req.username == "driver" && req.password == "password123" {
            Role::User
        } else if req.username == "admin" && req.password == "admin123" {
            Role::Admin
        } else {
            return Err(Status::unauthenticated("Invalid username or password"));
        };

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let expires_at_timestamp = now + 3600;

        let my_claims = Claims {
            sub: if determined_role == Role::Admin {
                "dispatcher_hq".to_string()
            } else {
                "cab_driver_42".to_string()
            },
            role: determined_role,
            exp: expires_at_timestamp,
        };

        let token_string = encode(
            &Header::default(),
            &my_claims,
            &EncodingKey::from_secret(JWT_SECRET),
        )
        .map_err(|_| Status::internal("Token generation sequence failed"))?;

        Ok(Response::new(LoginResponse {
            token: format!("Bearer {}", token_string),
            expires_at: expires_at_timestamp as i64,
        }))
    }

    async fn record_cab_location(
        &self,
        request: Request<CabLocationRequest>,
    ) -> Result<Response<CabLocationResponse>, Status> {
        let claims = request.extensions().get::<UserClaims>().ok_or_else(|| {
            Status::unauthenticated("Authentication token required for this route")
        })?;

        println!(
            "[LOG] Location recorded by verified user '{}' ({:?})",
            claims.user_id, claims.role
        );
        Ok(Response::new(CabLocationResponse { accepted: true }))
    }

    async fn get_cabs(
        &self,
        request: Request<GetCabRequest>,
    ) -> Result<Response<GetCabResponse>, Status> {
        let _claims = request.extensions().get::<UserClaims>().ok_or_else(|| {
            Status::unauthenticated("Authentication token required for this route")
        })?;

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

    async fn stop_server(
        &self,
        request: Request<StopServerRequest>,
    ) -> Result<Response<StopServerResponse>, Status> {
        let claims = request.extensions().get::<UserClaims>().ok_or_else(|| {
            Status::unauthenticated("Authentication token required for this route")
        })?;

        if claims.role != Role::Admin {
            return Err(Status::permission_denied(
                "Administrative authorization required",
            ));
        }

        let admin_id = claims.user_id.clone();
        let inner_msg = request.into_inner();

        println!(
            "[WARN] Shutdown initiated by verified administrator: {}. Reason: {}",
            admin_id, inner_msg.reason
        );

        let tx = self.shutdown_tx.clone();
        tokio::spawn(async move {
            let _ = tx.send(()).await;
        });

        Ok(Response::new(StopServerResponse::default()))
    }
}
