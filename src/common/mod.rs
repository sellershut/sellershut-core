#[cfg(feature = "rpc")]
tonic::include_proto!("common");

#[cfg(not(feature = "rpc"))]
include!(concat!(env!("OUT_DIR"), "/common.rs"));
