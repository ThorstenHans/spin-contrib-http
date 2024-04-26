use spin_sdk::http::{Params, Request, Response, ResponseBuilder, Router};
/// This struct is used to configure CORS support
pub struct CorsConfig {
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

impl CorsConfig {
    /// Checks if the provided origin is allowed
    pub fn is_origin_allowed(&self, origin: &str) -> bool {
        if self.allowed_origins.is_empty() || self.allowed_origins == NO_ORIGINS {
            return false;
        }
        if self.allowed_origins == ALL_ORIGINS {
            return true;
        }
        let allowed_origins: Vec<&str> = self.allowed_origins.split(',').collect();
        for allowed_origin in allowed_origins {
            if allowed_origin == origin {
                return true;
            }
        }
        false
    }

    /// Checks if the provided HTTP method is allowed
    pub fn is_method_allowed(&self, method: &str) -> bool {
        if self.allowed_methods.is_empty() {
            return false;
        }
        if self.allowed_methods == ALL_METHODS {
            return true;
        }
        let allowed_methods: Vec<&str> = self.allowed_methods.split(',').collect();
        for allowed_method in allowed_methods {
            if allowed_method == method {
                return true;
            }
        }
        false
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

/// Trait to add CORS capabilities
pub trait CorsResponseBuilder {
    /// Build an HTTP response with CORS headers
    fn build_with_cors(&mut self, cors_config: CorsConfig) -> Response;
}

impl CorsResponseBuilder for ResponseBuilder {
    fn build_with_cors(&mut self, cors_config: CorsConfig) -> Response {
        let mut origin = cors_config.allowed_origins.as_str();
        if origin.is_empty() {
            origin = NO_ORIGINS;
        }
        let mut headers: Vec<(String, String)> = vec![
            (
                http::header::ACCESS_CONTROL_ALLOW_ORIGIN.to_string(),
                origin.to_string(),
            ),
            (
                http::header::ACCESS_CONTROL_ALLOW_METHODS.to_string(),
                cors_config.allowed_methods,
            ),
            (
                http::header::ACCESS_CONTROL_ALLOW_HEADERS.to_string(),
                cors_config.allowed_headers,
            ),
            (
                http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS.to_string(),
                format!("{}", cors_config.allow_credentials),
            ),
        ];
        if cors_config.max_age.is_some() {
            headers.push((
                http::header::ACCESS_CONTROL_MAX_AGE.to_string(),
                format!("{}", cors_config.max_age.unwrap()),
            ));
        }
        self.headers(headers).build()
    }
}

/// Trait to add CORS capabilities to spin_sdk::http::Router
pub trait CorsRouter {
    /// Register handler for CORS OPTIONS requests
    fn register_options_handler(&mut self, cors_config: CorsConfig);
}

impl CorsRouter for Router {
    fn register_options_handler(&mut self, cors_config: CorsConfig) {
        self.options(
            "/*",
            move |req: Request, _: Params| -> anyhow::Result<Response> {
                println!("Checking headers on req");
                let origin_header = req.header(http::header::ORIGIN.as_str());
                let access_control_req_method_header =
                    req.header(http::header::ACCESS_CONTROL_REQUEST_METHOD.as_str());

                if origin_header.is_none() || access_control_req_method_header.is_none() {
                    return Ok(Response::new(http::StatusCode::NO_CONTENT, ()));
                }

                let Some(origin) = origin_header.unwrap().as_str() else {
                    return Ok(Response::new(http::StatusCode::NO_CONTENT, ()));
                };
                let Some(method) = access_control_req_method_header.unwrap().as_str() else {
                    return Ok(Response::new(http::StatusCode::NO_CONTENT, ()));
                };

                if origin.is_empty() || method.is_empty() {
                    return Ok(Response::new(http::StatusCode::NO_CONTENT, ()));
                }
                if cors_config.is_origin_allowed(origin) && cors_config.is_method_allowed(method) {
                    let mut headers: Vec<(String, String)> = vec![
                        (
                            http::header::ACCESS_CONTROL_ALLOW_ORIGIN.to_string(),
                            origin.to_string(),
                        ),
                        (
                            http::header::ACCESS_CONTROL_ALLOW_METHODS.to_string(),
                            method.to_string(),
                        ),
                        (
                            http::header::ACCESS_CONTROL_ALLOW_HEADERS.to_string(),
                            cors_config.allowed_headers.clone(),
                        ),
                        (
                            http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS.to_string(),
                            format!("{}", cors_config.allow_credentials),
                        ),
                    ];
                    if cors_config.max_age.is_some() {
                        headers.push((
                            http::header::ACCESS_CONTROL_MAX_AGE.to_string(),
                            format!("{}", cors_config.max_age.unwrap()),
                        ));
                    }

                    return Ok(Response::builder()
                        .status(http::StatusCode::NO_CONTENT.as_u16())
                        .headers(headers)
                        .body(())
                        .build());
                }
                Ok(Response::new(http::StatusCode::NO_CONTENT, ()))
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_with_cors_sets_origins() {
        let origin = "http://localhost:3000";
        let cfg = CorsConfig {
            allowed_origins: origin.to_string(),
            allowed_methods: ALL_METHODS.to_string(),
            allowed_headers: ALL_HEADERS.to_string(),
            allow_credentials: true,
            max_age: None,
        };
        let sut = ResponseBuilder::new(200).build_with_cors(cfg);

        let actual = sut
            .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN.as_str())
            .unwrap()
            .as_str()
            .unwrap();
        assert_eq!(actual, origin);
    }

    #[test]
    fn builder_with_cors_null_when_origins_is_empty() {
        let origin = "";
        let cfg = CorsConfig {
            allowed_origins: origin.to_string(),
            allowed_methods: ALL_METHODS.to_string(),
            allowed_headers: ALL_HEADERS.to_string(),
            allow_credentials: true,
            max_age: None,
        };
        let sut = ResponseBuilder::new(200).build_with_cors(cfg);

        let actual = sut
            .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN.as_str())
            .unwrap()
            .as_str()
            .unwrap();
        assert_eq!(actual, NO_ORIGINS);
    }
}
