use std::fmt::Debug;

use super::NO_ORIGINS;

/// This struct is used to configure CORS support
pub struct CorsConfig {
    /// The origins to allow in CORS (separated by commas)
    pub(crate) allowed_origins: String,
    /// The HTTP methods to allow in CORS (separated by commas)
    pub(crate) allowed_methods: String,
    /// The HTTP headers to allow in CORS (separated by commas)
    pub(crate) allowed_headers: String,
    /// Whether or not to allow credentials in CORS
    pub(crate) allow_credentials: bool,
    /// The max age to allow in CORS
    pub(crate) max_age: Option<u32>,
}

impl CorsConfig {
    /// CorsConfig Constructor
    pub fn new(
        allowed_origins: String,
        allowed_methods: String,
        allowed_headers: String,
        allow_credentials: bool,
        max_age: Option<u32>,
    ) -> Self {
        let mut origin = allowed_origins.clone();
        if allowed_origins.is_empty() {
            origin = NO_ORIGINS.to_string();
        }
        let allowed_methods = allowed_methods.to_uppercase().split_whitespace().collect();
        CorsConfig {
            allowed_origins: origin,
            allowed_methods,
            allowed_headers,
            allow_credentials,
            max_age,
        }
    }
}

impl Debug for CorsConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CorsConfig")
            .field("allowed_origins", &self.allowed_origins)
            .field("allowed_methods", &self.allowed_methods)
            .field("allowed_headers", &self.allowed_headers)
            .field("allow_credentials", &self.allow_credentials)
            .field("max_age", &self.max_age)
            .finish()
    }
}

impl Clone for CorsConfig {
    fn clone(&self) -> Self {
        Self {
            allowed_origins: self.allowed_origins.clone(),
            allowed_methods: self.allowed_methods.clone(),
            allowed_headers: self.allowed_headers.clone(),
            allow_credentials: self.allow_credentials,
            max_age: self.max_age,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::cors::{ALL_HEADERS, ALL_METHODS, NO_ORIGINS};

    use super::*;

    #[test]
    fn cors_config_should_set_null_when_origin_is_provided_as_empty() {
        let origin = "";
        let sut = CorsConfig::new(
            origin.to_string(),
            ALL_METHODS.to_string(),
            ALL_HEADERS.to_string(),
            true,
            None,
        );
        assert_eq!(sut.allowed_origins, NO_ORIGINS);
    }
}
