use anyhow::Result;
use http::response::Builder;
use spin_sdk::http::{Request, Response};

use super::no_content;

/// This struct is used to configure CORS support
pub struct Config {
    /// The origins to allow in CORS (separated by commas)
    pub allowed_origins: String,
    /// The HTTP methods to allow in CORS (separated by commas)
    pub allowed_methods: String,
    /// The HTTP headers to allow in CORS (separated by commas)
    pub allowed_headers: String,
    /// Whether or not to allow credentials in CORS
    pub allow_credentials: bool,
    /// The max age to allow in CORS
    pub max_age: Option<u32>,
}

impl Config {
    /// Checks if the provided origin is allowed
    pub fn is_origin_allowed(&self, origin: &str) -> bool {
        if self.allowed_origins.is_empty() || self.allowed_origins == NO_ORIGINS {
            return false;
        }
        if self.allowed_origins == ALL_ORIGINS {
            return true;
        }
        let allowed_origins: Vec<&str> = self.allowed_origins.split(",").collect();
        for allowed_origin in allowed_origins {
            if allowed_origin == origin {
                return true;
            }
        }
        return false;
    }

    /// Checks if the provided HTTP method is allowed
    pub fn is_method_allowed(&self, method: &str) -> bool {
        if self.allowed_methods.is_empty() {
            return false;
        }
        if self.allowed_methods == ALL_METHODS {
            return true;
        }
        let allowed_methods: Vec<&str> = self.allowed_methods.split(",").collect();
        for allowed_method in allowed_methods {
            if allowed_method == method {
                return true;
            }
        }
        return false;
    }
}

/// Constant for allowing all HTTP methods in CORS
pub const ALL_METHODS: &str = "*";
/// Constant for allowing all HTTP headers in CORS
pub const ALL_HEADERS: &str = "*";
/// Constant for allowing all origins in CORS
pub const ALL_ORIGINS: &str = "*";
/// Constant for allowing no origins in CORS
pub const NO_ORIGINS: &str = "NULL";

/// Creates and returns a new `http::response::Builder` with CORS support
///
/// # Arguments
///
/// * `cors_config` - The CORS configuration ([Config])
///
/// # Example
/// ```rust
/// use anyhow::Result;
/// use spin_sdk::{
///  http::{Request, Response},
/// };  
/// use spin_contrib_http::response::cors::{
///     Config,
///     builder_with_cors,
///     ALL_METHODS, ALL_HEADERS,
/// };
///
/// fn handler(req: Request) -> Result<Response> {
///     let cfg = Config {
///         allowed_origins: "https://example.com".into(),
///         allowed_methods: ALL_METHODS.into(),
///         allowed_headers: ALL_HEADERS.into(),
///         allow_credentials: true,
///         max_age: None,
///     };
///     let builder = builder_with_cors(cfg);
///     let b = Some("Hello World".into());
///     Ok(builder.body(b)?)
/// }
/// ```
pub fn builder_with_cors(cors_config: Config) -> Builder {
    let mut origin = cors_config.allowed_origins.as_str();
    if origin.is_empty() {
        origin = NO_ORIGINS;
    }
    let mut builder = http::response::Builder::new()
        .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, origin)
        .header(
            http::header::ACCESS_CONTROL_ALLOW_METHODS,
            cors_config.allowed_methods.as_str(),
        )
        .header(
            http::header::ACCESS_CONTROL_ALLOW_HEADERS,
            cors_config.allowed_headers.as_str(),
        )
        .header(
            http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
            format!("{}", cors_config.allow_credentials),
        );
    if cors_config.max_age.is_some() {
        builder = builder.header(
            http::header::ACCESS_CONTROL_MAX_AGE,
            format!("{}", cors_config.max_age.unwrap()),
        );
    }
    return builder;
}

/// Handles a CORS preflight request
///
/// # Arguments
///
/// * `req` - The HTTP request
/// * `cors_config` - The CORS configuration
///
/// # Example
/// ```rust
/// use anyhow::Result;
/// use spin_sdk::{
///  http::{Request, Response},
/// };
/// use spin_contrib_http::response::{no_content,cors::{Config, handle_preflight}};
/// pub fn handler(req: Request) -> Result<Response> {
///   let cors_config = Config {
///     allowed_origins: "https://example.com".into(),
///     allowed_methods: "POST,PUT,DELETE".into(),
///     allowed_headers: "Content-Type,Authorization".into(),
///     allow_credentials: true,
///     max_age: None,
///   };
///   if req.method() == http::Method::OPTIONS {
///     return handle_preflight(&req, cors_config);
///   }
///   no_content()
/// }
/// ```
pub fn handle_preflight(req: &Request, cors_config: Config) -> Result<Response> {
    if !req.headers().contains_key(http::header::ORIGIN)
        || !req
            .headers()
            .contains_key(http::header::ACCESS_CONTROL_REQUEST_METHOD)
    {
        return no_content();
    }
    let Ok(origin) = req.headers().get(http::header::ORIGIN).unwrap().to_str() else {
        return no_content();
    };
    let Ok(method) = req.headers().get(http::header::ACCESS_CONTROL_REQUEST_METHOD).unwrap().to_str() else {
        return no_content();
    };

    if origin.is_empty() || method.is_empty() {
        return no_content();
    }
    if cors_config.is_origin_allowed(origin) && cors_config.is_method_allowed(method) {
        let mut builder = http::Response::builder()
            .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, origin)
            .header(http::header::ACCESS_CONTROL_ALLOW_METHODS, method)
            .header(
                http::header::ACCESS_CONTROL_ALLOW_HEADERS,
                cors_config.allowed_headers.as_str(),
            )
            .header(
                http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
                format!("{}", cors_config.allow_credentials),
            );

        if cors_config.max_age.is_some() {
            builder = builder.header(
                http::header::ACCESS_CONTROL_MAX_AGE,
                format!("{}", cors_config.max_age.unwrap()),
            );
        }
        return Ok(builder.status(http::StatusCode::NO_CONTENT).body(None)?);
    }
    no_content()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_with_cors_sets_origins() {
        let origin = "http://localhost:3000";
        let cfg = Config {
            allowed_origins: origin.to_string(),
            allowed_methods: ALL_METHODS.to_string(),
            allowed_headers: ALL_HEADERS.to_string(),
            allow_credentials: true,
            max_age: None,
        };
        let sut = builder_with_cors(cfg);

        let actual = sut
            .headers_ref()
            .unwrap()
            .get(http::header::ACCESS_CONTROL_ALLOW_ORIGIN)
            .unwrap();
        assert_eq!(actual, origin);
    }

    #[test]
    fn builder_with_cors_null_when_origins_is_empty() {
        let origin = "";
        let cfg = Config {
            allowed_origins: origin.to_string(),
            allowed_methods: ALL_METHODS.to_string(),
            allowed_headers: ALL_HEADERS.to_string(),
            allow_credentials: true,
            max_age: None,
        };
        let sut = builder_with_cors(cfg);

        let actual = sut
            .headers_ref()
            .unwrap()
            .get(http::header::ACCESS_CONTROL_ALLOW_ORIGIN)
            .unwrap();
        assert_eq!(actual, NO_ORIGINS);
    }
}
