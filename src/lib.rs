#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "users")]
#[cfg_attr(docsrs, doc(cfg(feature = "users")))]
pub mod users;

#[cfg(any(feature = "categories", feature = "users", feature = "listings"))]
pub mod google;

#[cfg(feature = "categories")]
#[cfg_attr(docsrs, doc(cfg(feature = "categories")))]
/// Categories
pub mod categories;

#[cfg(feature = "listings")]
#[cfg_attr(docsrs, doc(cfg(feature = "listings")))]
/// Listings
pub mod listings;

#[cfg(any(feature = "categories", feature = "users", feature = "listings"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "categories", feature = "users", feature = "listings")))
)]
/// Common
pub mod common;
