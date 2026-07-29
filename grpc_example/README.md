# GRPC example


## Example
- a service like Uber
- Central server - clients (cabs) record names and locations
- When a user request a cab, the server sends a list of cabs near that user
- In an ideal scenario:
    - Server should have two kind of clients

## Work actions

- download https://github.com/protocolbuffers/protobuf/releases/download/v35.1/protoc-35.1-linux-x86_64.zip using
curl -LO https://github.com/protocolbuffers/protobuf/releases/download/v35.1/protoc-35.1-linux-x86_64.zip 
- unzip protoc-35.1-linux-x86_64.zip -d protoc3
- sudo mv protoc3/bin/* /usr/local/bin

## Running
In one terminal session start the server

```bash
$ cargo run -- --server
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `/home/bvpelt/Develop/rwebserver/target/debug/grpc_example --server`
🛡️ Secure gRPC server listening on 127.0.0.1:50051
```

In another terminal session start the client

```bash
$ cargo run -- --client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `/home/bvpelt/Develop/rwebserver/target/debug/grpc_example --client`
[Client] Connecting to target gateway: http://127.0.0.1:50051
[Client] Attempting driver login exchange...
[Client] JWT Token acquired.
[Client] Sending protected update payload using JWT headers...
[Client] Response received: CabLocationResponse { accepted: true }
```

The server session gives loggin

```bash
received auth_str: "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJjYWJfZHJpdmVyXzQyIiwicm9sZSI6IlVzZXIiLCJleHAiOjE3ODQ5MjI1NjF9.DENWaCcig7HBtnDfB5DAC5QfECrPXeXGUl6nfhC4axo"
token_data: TokenData { header: Header { typ: Some("JWT"), alg: HS256, cty: None, jku: None, jwk: None, kid: None, x5u: None, x5c: None, x5t: None, x5t_s256: None, crit: None, enc: None, zip: None, url: None, nonce: None, extras: {} }, claims: Claims { sub: "cab_driver_42", role: User, exp: 1784922561 } }
[LOG] Location recorded by verified user 'cab_driver_42' (User)
```