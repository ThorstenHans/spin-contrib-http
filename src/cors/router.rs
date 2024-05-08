use spin_sdk::http::{HeaderValue, Params, Request, Response, ResponseBuilder, Router};

use super::{build_cors_headers, is_method_allowed, CorsConfig, ALL_ORIGINS, NO_ORIGINS};

/// Trait to add CORS capabilities to spin_sdk::http::Router
pub trait CorsRouter {
    /// Register handler for CORS OPTIONS requests
    fn register_options_handler(&mut self, cors_config: &CorsConfig);
}

impl CorsRouter for Router {
    fn register_options_handler(&mut self, cors_config: &CorsConfig) {
        let cfg = cors_config.clone();
        self.options(
            "/*",
            move |req: Request, _: Params| -> anyhow::Result<Response> {
                options_handler(&req, &cfg)
            },
        )
    }
}

fn options_handler(req: &Request, cors_config: &CorsConfig) -> anyhow::Result<Response> {
    let req_origin = req
        .header(http::header::ORIGIN.as_str())
        .unwrap_or(&HeaderValue::string(String::default()))
        .as_str()
        .unwrap()
        .to_string();

    if (cors_config.allowed_origins != ALL_ORIGINS
        && (cors_config.allowed_origins == NO_ORIGINS
            || !cors_config.allowed_origins.contains(&req_origin)))
        || req_origin.is_empty()
    {
        return Ok(Response::new(403, ()));
    }

    let requested_method = req
        .header(http::header::ACCESS_CONTROL_REQUEST_METHOD.as_str())
        .unwrap_or(&HeaderValue::string(String::default()))
        .as_str()
        .unwrap()
        .to_string();

    if requested_method.is_empty()
        || !is_method_allowed(&cors_config.allowed_methods, &requested_method)
    {
        return Ok(Response::new(405, ()));
    }
    let headers = build_cors_headers(req.method(), req_origin, cors_config);
    Ok(ResponseBuilder::new(http::StatusCode::NO_CONTENT)
        .headers(headers)
        .body(())
        .build())
}

#[cfg(test)]
mod tests {
    use spin_sdk::http::{Method, RequestBuilder};

    use crate::cors::{router::options_handler, CorsConfig, ALL_HEADERS};

    #[test]
    fn preflights_with_invalid_origin_should_result_in_forbidden() -> anyhow::Result<()> {
        let req = RequestBuilder::new(Method::Get, "http://foo.bar")
            .header(http::header::ORIGIN.as_str(), "http://bar.com")
            .build();

        let cfg = CorsConfig::new(
            "http://not-bar.com".to_string(),
            "POST".to_string(),
            ALL_HEADERS.to_string(),
            true,
            Some(300),
        );
        let sut = options_handler(&req, &cfg)?;
        assert_eq!(sut.status(), &http::StatusCode::FORBIDDEN.as_u16());
        Ok(())
    }

    #[test]
    fn preflight_must_return_method_not_allowed_if_requested_method_is_not_in_cors_config(
    ) -> anyhow::Result<()> {
        let req = RequestBuilder::new(Method::Get, "http://foo.bar")
            .header(http::header::ORIGIN.as_str(), "http://localhost:4200")
            .header(
                http::header::ACCESS_CONTROL_REQUEST_METHOD.as_str(),
                "PATCH",
            )
            .build();

        let cfg = CorsConfig::new(
            "http://localhost:4200".to_string(),
            "POST".to_string(),
            ALL_HEADERS.to_string(),
            true,
            Some(300),
        );

        let sut = options_handler(&req, &cfg)?;

        assert_eq!(sut.status(), &http::StatusCode::METHOD_NOT_ALLOWED.as_u16());
        Ok(())
    }
}
