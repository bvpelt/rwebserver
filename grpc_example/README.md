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