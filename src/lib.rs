#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(
    missing_docs,
    rustdoc::broken_intra_doc_links,
    missing_debug_implementations
)]

//! sellershut API utilities

#[cfg(any(feature = "categories", feature = "users"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "categories", feature = "users"))))]
/// Shared utilities
pub mod common;

#[cfg(feature = "categories")]
#[cfg_attr(docsrs, doc(cfg(feature = "categories")))]
/// Categories API utilities
pub mod category;

#[cfg(feature = "users")]
#[cfg_attr(docsrs, doc(cfg(feature = "users")))]
/// Users API utilities
pub mod user;

#[cfg(feature = "users")]
#[cfg_attr(docsrs, doc(cfg(feature = "users")))]
/// Oauth utilities
pub mod oauth;

mod tables;
pub use tables::*;
