#[cfg(feature = "rpc")]
tonic::include_proto!("category");

#[cfg(not(feature = "rpc"))]
include!(concat!(env!("OUT_DIR"), "/category.rs"));
