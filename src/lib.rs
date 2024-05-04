#[cfg(any(feature = "categories", feature = "users"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "categories", feature = "users"))))]
pub mod common;

#[cfg(feature = "categories")]
#[cfg_attr(docsrs, doc(cfg(feature = "categories")))]
pub mod category;

#[cfg(feature = "users")]
#[cfg_attr(docsrs, doc(cfg(feature = "users")))]
pub mod user;

#[cfg(feature = "tonic-rpc")]
pub mod reexports {
    pub use tonic;
}
