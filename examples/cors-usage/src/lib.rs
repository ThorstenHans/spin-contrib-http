use spin_contrib_http::cors::{
    CorsConfig, CorsResponseBuilder, CorsRouter, ALL_HEADERS, ALL_METHODS, ALL_ORIGINS,
};
use spin_contrib_http::request::Contrib;
use spin_sdk::http::{HeaderValue, IntoResponse, Params, Request, Response, Router};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
fn usage_sample(req: Request) -> anyhow::Result<impl IntoResponse> {
    let cfg = CorsConfig::new(
        ALL_ORIGINS.to_string(),
        ALL_METHODS.to_string(),
        ALL_HEADERS.to_string(),
        false,
        Some(3600),
    );
    let mut router = Router::default();
    router.register_options_handler(&cfg);
    router.get("/", handler);

    let method = &req.method().clone();
    let request_origin = req.get_header_value_as_string("origin");

    Ok(router
        .handle(req)
        .into_builder()
        .build_with_cors(method, request_origin, &cfg))
}

fn handler(_req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    Ok(Response::new(200, ()))
}
