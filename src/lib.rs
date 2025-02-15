#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "users")]
#[cfg_attr(docsrs, doc(cfg(feature = "users")))]
pub mod users;

#[cfg(any(feature = "categories", feature = "users"))]
pub mod google;

#[cfg(feature = "categories")]
#[cfg_attr(docsrs, doc(cfg(feature = "categories")))]
/// Categories
pub mod categories;

#[cfg(any(feature = "categories", feature = "users"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "categories", feature = "users"))))]
/// Common
pub mod common;
