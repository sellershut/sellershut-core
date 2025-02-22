<div align="center">
  <img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/sellershut/sellershut-core/check.yaml?label=build">
  
 <a href="https://codecov.io/github/sellershut/sellershut-core" > 
 <img src="https://codecov.io/github/sellershut/sellershut-core/graph/badge.svg?token=cp3sFpIqlC"/> 
 </a>
</div>
<h1 align="center">sellershut-core</h1>
<p align="center">
A foundational library providing core types and entities used across the platform
<br />

## Build Dependencies
- `protoc`

## Features

`sellershut-core` provides various features organised by entity and functionalities. Enable the features you need through `cargo` feature flags

### Users Features

These features are related to the types and functionalities used by the [users-service](https://github.com/sellershut/users-service)

- **`users`**: Enables the `User` type definition
- **`rpc-client-users`**: Generates gRPC client implementation
    - Implicitly enables the `users` feature
- **`rpc-server-users`**: Generates a gRPC server implementations
    - Implicitly enables the `users` feature

### Categories Features

These features are related to the types and functionalities used by the [categories-service](https://github.com/sellershut/categories-service)

- **`categories`**: Enables the `Category` type definition and pagination utilities
- **`rpc-client-categories`**: Generates gRPC client implementation
    - Implicitly enables the `categories` feature
- **`rpc-server-categories`**: Generates a gRPC server implementations
    - Implicitly enables the `categories` feature

### Listings Features

These features are related to the types and functionalities used by the [listings-service](https://github.com/sellershut/listings-service)

- **`listings`**: Enables the `Listing` type definition and pagination utilities
- **`rpc-client-listings`**: Generates gRPC client implementation
    - Implicitly enables the `listings` feature
- **`rpc-server-listings`**: Generates a gRPC server implementations
    - Implicitly enables the `listings` feature

### Common Features

These features are used across multiple microservices or provide easy conversions between types:

- **`serde`**: Derives `serde::Deserialize` and `serde::Serialize` for types
- **`time`**: Implements `Timestamp` type conversions to and fro [time::OffsetDateTime](https://docs.rs/time/latest/time/struct.OffsetDateTime.html)  using the `From` and `TryFrom` traits

## Usage

To use `sellershut-core` in your project, add the dependency:

```toml
[dependencies]
sellershut-core = { git = "https://github.com/sellershut/sellershut-core", features = ["rpc-client-users"] }
```

## Examples

- `serde`
Serialisation and deserialisation of a `User` with serde:
```sh
cargo run --example serde
```
- `grpc_users`
A gRPC client and server implementation for querying users

```sh
cargo run --example grpc_server feature="rpc-server-users"
cargo run --example grpc_client feature="rpc-client-users"
```
