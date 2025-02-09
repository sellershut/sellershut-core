#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "users")]
#[cfg_attr(docsrs, doc(cfg(feature = "users")))]
pub mod users;

#[cfg(feature = "users")]
pub mod google;
