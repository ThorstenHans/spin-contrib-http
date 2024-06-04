use spin_sdk::http::{Method, Response, ResponseBuilder};

use super::{build_cors_headers, is_origin_allowed, CorsConfig, ALL_ORIGINS};

/// Trait to add CORS capabilities
pub trait CorsResponseBuilder {
    /// Build an HTTP response with CORS headers
    fn build_with_cors(
        &mut self,
        request_method: &Method,
        request_origin: String,
        cors_config: &CorsConfig,
    ) -> Response;
}

impl CorsResponseBuilder for ResponseBuilder {
    fn build_with_cors(
        &mut self,
        request_method: &Method,
        request_origin: String,
        cors_config: &CorsConfig,
    ) -> Response {
        if !request_origin.is_empty()
            && cors_config.allowed_origins != ALL_ORIGINS
            && !is_origin_allowed(&cors_config.allowed_origins, &request_origin)
        {
            self.status(403);
            self.body(());
        }

        let headers = build_cors_headers(request_method, request_origin, cors_config);
        self.headers(headers).build()
    }
}

#[cfg(test)]
mod tests {
    use spin_sdk::http::{HeaderValue, Method, RequestBuilder, ResponseBuilder};

    use crate::cors::{CorsConfig, CorsResponseBuilder, ALL_HEADERS, ALL_METHODS, ALL_ORIGINS};

    #[test]
    fn vary_header_should_be_set_when_origins_are_set_explicitly() {
        let req = RequestBuilder::new(Method::Post, "http://foo.bar")
            .header(http::header::ORIGIN.as_str(), "http://localhost:4005")
            .body(())
            .build();

        let test_data = vec![
            "http://localhost:4000",
            "http://localhost:4000,http://localhost:4005",
        ];

        for td in test_data {
            let cfg = CorsConfig {
                allowed_origins: td.to_string(),
                allowed_headers: ALL_HEADERS.to_string(),
                allowed_methods: ALL_METHODS.to_string(),
                allow_credentials: true,
                max_age: None,
            };
            let request_origin = req
                .header(http::header::ORIGIN.as_str())
                .unwrap_or(&HeaderValue::string(String::default()))
                .as_str()
                .unwrap()
                .to_string();
            let sut = ResponseBuilder::new(200).build_with_cors(req.method(), request_origin, &cfg);

            let vary_header = sut.header(http::header::VARY.as_str());
            assert_eq!(vary_header.is_some(), true);
            assert_eq!(vary_header.unwrap().as_str().unwrap(), "Origin");
        }
    }

    #[test]
    fn vary_header_should_not_be_set_if_all_origins_are_allowed() {
        let req = RequestBuilder::new(Method::Get, "http://foo.bar")
            .header(http::header::ORIGIN.as_str(), "http://bar.baz")
            .body(())
            .build();

        let cfg = CorsConfig {
            allowed_origins: ALL_ORIGINS.to_string(),
            allowed_headers: ALL_HEADERS.to_string(),
            allowed_methods: ALL_METHODS.to_string(),
            allow_credentials: true,
            max_age: None,
        };

        let request_origin = req
            .header(http::header::ORIGIN.as_str())
            .unwrap_or(&HeaderValue::string(String::default()))
            .as_str()
            .unwrap()
            .to_string();
        let sut = ResponseBuilder::new(200).build_with_cors(req.method(), request_origin, &cfg);

        let vary_header = sut.header(http::header::VARY.as_str());
        assert_eq!(vary_header.is_none(), true);
    }

    #[test]
    fn builder_with_cors_sets_origins() {
        let allowed_origins = "http://localhost:3000,http://localhost:4200";
        let expected = "http://localhost:4200";

        let req = RequestBuilder::new(Method::Get, "http://foo.bar")
            .header(http::header::ORIGIN.as_str(), expected)
            .body(())
            .build();

        let cfg = CorsConfig {
            allowed_origins: allowed_origins.to_string(),
            allowed_methods: ALL_METHODS.to_string(),
            allowed_headers: ALL_HEADERS.to_string(),
            allow_credentials: true,
            max_age: None,
        };
        let request_origin = req
            .header(http::header::ORIGIN.as_str())
            .unwrap_or(&HeaderValue::string(String::default()))
            .as_str()
            .unwrap()
            .to_string();
        let sut = ResponseBuilder::new(200).build_with_cors(req.method(), request_origin, &cfg);

        let actual = sut
            .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN.as_str())
            .unwrap()
            .as_str()
            .unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn builder_with_cors_should_not_set_cors_headers_for_invalid_origin() {
        let allowed_origins = "http://localhost:3000,http://localhost:4200";
        let expected = "http://localhost:8080";

        let req = RequestBuilder::new(Method::Get, "http://foo.bar")
            .header(http::header::ORIGIN.as_str(), expected)
            .body(())
            .build();

        let cfg = CorsConfig {
            allowed_origins: allowed_origins.to_string(),
            allowed_methods: ALL_METHODS.to_string(),
            allowed_headers: ALL_HEADERS.to_string(),
            allow_credentials: true,
            max_age: None,
        };
        let request_origin = req
            .header(http::header::ORIGIN.as_str())
            .unwrap_or(&HeaderValue::string(String::default()))
            .as_str()
            .unwrap()
            .to_string();
        let sut = ResponseBuilder::new(200).build_with_cors(req.method(), request_origin, &cfg);
        assert_eq!(
            sut.header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN.as_str())
                .is_none(),
            true
        );
        assert_eq!(
            sut.header(http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS.as_str())
                .is_none(),
            true
        );
    }

    #[test]
    fn requests_should_contain_origin_and_credentials() {
        let cfg = CorsConfig::new(
            "http://localhost:4200".to_string(),
            "POST".to_string(),
            ALL_HEADERS.to_string(),
            true,
            Some(300),
        );
        let req = RequestBuilder::new(Method::Get, "http://foo.bar")
            .header(
                http::header::ORIGIN.to_string(),
                "http://localhost:4200".to_string(),
            )
            .build();

        let request_origin = req
            .header(http::header::ORIGIN.as_str())
            .unwrap_or(&HeaderValue::string(String::default()))
            .as_str()
            .unwrap()
            .to_string();
        let sut =
            ResponseBuilder::new(200)
                .body(())
                .build_with_cors(req.method(), request_origin, &cfg);

        assert_eq!(
            sut.header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN.as_str())
                .is_some(),
            true
        );
        assert_eq!(
            sut.header(http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS.as_str())
                .is_some(),
            true
        );
    }

    #[test]
    fn requests_without_origin_header_should_not_respond_with_cors_headers() {
        let req = RequestBuilder::new(Method::Get, "http://foo.bar").build();

        let cfg = CorsConfig::new(
            "http://localhost:4200".to_string(),
            "POST".to_string(),
            ALL_HEADERS.to_string(),
            true,
            Some(300),
        );
        let request_origin = req
            .header(http::header::ORIGIN.as_str())
            .unwrap_or(&HeaderValue::string(String::default()))
            .as_str()
            .unwrap()
            .to_string();
        let sut = ResponseBuilder::new(200).build_with_cors(req.method(), request_origin, &cfg);

        assert_eq!(
            sut.header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN.as_str())
                .is_none(),
            true
        );
        assert_eq!(
            sut.header(http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS.as_str())
                .is_none(),
            true
        );
    }
}
