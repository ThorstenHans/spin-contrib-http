//! spin-contrib-http is a small crate that provides a set of helpers that will improve your development performance when
//! building HTTP applications with [Fermyon Spin](https://developer.fermyon.com/spin/index).
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(missing_docs)]

/// Helpers to simplify working with cookies
pub mod cookies;
/// Helpers to simplify working with Cross-Origin Resource Sharing (CORS)
pub mod cors;
/// Extensions for working with HTTP requests
pub mod request;
/// Extensions for working with HTTP responses
pub mod response;
