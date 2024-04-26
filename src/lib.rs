//! Helpers for building HTTP-APIs with Fermyon Spin
//!
//! `spin-contrib-http` contributes the following capabilities to the [Spin SDK for Rust](https://crates.io/crates/spin-sdk):
//!
//! - Creating cookies
//! - Cross-Origin Resource-Sharing (CORS)
//! - Response helpers to produce common HTTP responses
//! - Request helpers to examine incoming HTTP requests

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
