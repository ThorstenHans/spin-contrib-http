use spin_sdk::http::{HeaderValue, Request};

use crate::request::Contrib;

use super::CorsConfig;

/// Constant for allowing all HTTP methods in CORS
pub const ALL_METHODS: &str = "*";
/// Constant for allowing all HTTP headers in CORS
pub const ALL_HEADERS: &str = "*";
/// Constant for allowing all origins in CORS
pub const ALL_ORIGINS: &str = "*";
/// Constant for allowing no origins in CORS
pub const NO_ORIGINS: &str = "null";

pub(crate) fn build_cors_headers(req: &Request, cors_config: &CorsConfig) -> Vec<(String, String)> {
    let mut headers: Vec<(String, String)> = vec![];

    if req.header(http::header::ORIGIN.as_str()).is_none() {
        return headers;
    }

    let requested_origin = req
        .header(http::header::ORIGIN.as_str())
        .unwrap_or(&HeaderValue::string(String::default()))
        .as_str()
        .unwrap_or("")
        .to_string();
    // if origin is not allowed, return no cors headers
    if is_origin_allowed(&cors_config.allowed_origins, &requested_origin) {
        headers.push((
            http::header::ACCESS_CONTROL_ALLOW_ORIGIN.to_string(),
            get_origin_header_value(&cors_config.allowed_origins, req),
        ));

        headers.push((
            http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS.to_string(),
            format!("{}", cors_config.allow_credentials),
        ));
    }

    if cors_config.allowed_origins != ALL_ORIGINS && cors_config.allowed_origins != NO_ORIGINS {
        headers.push((http::header::VARY.to_string(), "Origin".to_string()));
    }

    if !req.is_preflight_request() {
        return headers;
    }

    if cors_config.max_age.is_some() {
        headers.push((
            http::header::ACCESS_CONTROL_MAX_AGE.to_string(),
            format!("{}", cors_config.max_age.unwrap()),
        ));
    }
    headers.push((
        http::header::ACCESS_CONTROL_ALLOW_METHODS.to_string(),
        cors_config.allowed_methods.clone(),
    ));
    headers.push((
        http::header::ACCESS_CONTROL_ALLOW_HEADERS.to_string(),
        cors_config.allowed_headers.clone(),
    ));
    headers
}

pub(crate) fn is_method_allowed(allowed_methods: &str, requested_methods: &str) -> bool {
    if requested_methods.is_empty() || allowed_methods.is_empty() {
        return false;
    }

    if allowed_methods == ALL_METHODS {
        return true;
    }

    let allowed_methods: String = allowed_methods.to_uppercase().split_whitespace().collect();
    let requested_methods: String = requested_methods
        .to_uppercase()
        .split_whitespace()
        .collect();

    let allowed_methods: Vec<&str> = allowed_methods.split(',').collect();
    let requested_methods: Vec<&str> = requested_methods.split(',').collect();
    for method in requested_methods {
        if !allowed_methods.contains(&method) {
            return false;
        }
    }
    true
}

pub(crate) fn is_origin_allowed(allowed_origins: &str, origin: &str) -> bool {
    if allowed_origins == NO_ORIGINS {
        return false;
    }

    if allowed_origins == ALL_ORIGINS {
        return true;
    }

    let allowed_origins = allowed_origins
        .to_lowercase()
        .split_whitespace()
        .collect::<String>();
    let allowed_origins: Vec<&str> = allowed_origins.split(',').collect();

    allowed_origins.contains(&origin.to_lowercase().trim())
}

pub(crate) fn get_origin_header_value(allowed_origins: &str, req: &Request) -> String {
    let origin_request_header_value = req
        .header(http::header::ORIGIN.as_str())
        .unwrap_or(&HeaderValue::string(String::default()))
        .as_str()
        .unwrap()
        .to_string();

    if allowed_origins == ALL_ORIGINS {
        return origin_request_header_value;
    }
    if allowed_origins.contains(&origin_request_header_value) {
        return origin_request_header_value;
    }
    NO_ORIGINS.to_string()
}

#[cfg(test)]
mod tests {
    use crate::cors::{is_method_allowed, is_origin_allowed};

    use super::{ALL_ORIGINS, NO_ORIGINS};

    #[test]
    fn is_method_allowed_tests() {
        let test_data = vec![
            ("POST", "POST", true),
            ("POST", "PATCH", false),
            ("POST", "POST,PATCH", false),
            ("PATCH, POST", "PATCH", true),
            ("PATCH, POST", "PATCH, POST", true),
            ("PATCH, POST", "POST, PATCH", true),
            ("PATCH, POST", "POST, PATCH, PUT", false),
            ("PATCH, POST", "", false),
            ("", "PUT", false),
            ("", "PUT,POST", false),
            ("*", "POST, PATCH", true),
            ("*", "POST", true),
        ];

        for (allowed, requested, expected) in test_data {
            assert_eq!(
                is_method_allowed(allowed, requested),
                expected,
                "Allowed were: {}, Requested were: {}",
                allowed,
                requested
            );
        }
    }

    #[test]
    fn is_origin_allowed_tests() {
        let test_data = vec![
            (ALL_ORIGINS, "http://localhost:4200", true),
            (NO_ORIGINS, "http://localhost:4200", false),
            (
                "http://localhost:5000, http://localhost:8080",
                "http://localhost:4200",
                false,
            ),
            ("http://localhost:5000", "http://localhost:4200", false),
            ("http://localhost:4200", "http://localhost:4200", true),
            (
                "http://localhost:4200, http://localhost:8000",
                "http://localhost:4200",
                true,
            ),
            (
                "http://localhost:4200,http://localhost:8000",
                "http://localhost:4200",
                true,
            ),
            (
                "http://localhost:8080, http://localhost:4200",
                "http://localhost:4200",
                true,
            ),
            (
                "http://localhost:8080,http://localhost:4200",
                "http://localhost:4200",
                true,
            ),
        ];
        for (allowed, requested, expected) in test_data {
            assert_eq!(
                is_origin_allowed(allowed, requested),
                expected,
                "Allowed Origins: {}, Requested Origin: {}",
                allowed,
                requested
            );
        }
    }
}
