use anyhow::Result;

use spin_sdk::http::Response;

/// Returns a `Result<spin_sdk::http::Response>` representing a redirect to the provided URL
/// with the provided status code and Location header
///
/// # Arguments
///
/// * `url` - The URL to redirect to
/// * `permanent` - Whether or not the redirect should be permanent
///
/// # Example
/// ```rust
/// use anyhow::Result;
/// use spin_sdk::{
///  http::{Request, Response},
/// };  
/// use spin_contrib_http::response::redirect;
///
/// pub fn handler(req: Request) -> Result<Response> {
///   let target = "https://example.com";
///   let permanent = false;
///   redirect(target, permanent)
/// }
/// ```
pub fn redirect(url: &str, permanent: bool) -> Result<Response> {
    let mut status_code = http::StatusCode::TEMPORARY_REDIRECT;
    if permanent {
        status_code = http::StatusCode::PERMANENT_REDIRECT;
    }
    Ok(Response::builder()
        .status(status_code)
        .header(http::header::LOCATION.as_str(), url)
        .body(())
        .build())
}

/// Returns a `Result<spin_sdk::http::Response>` representing a 400 Bad Request
///
/// # Example
/// ```rust
/// use anyhow::Result;
/// use spin_sdk::{
///  http::{Request, Response},
/// };
/// use spin_contrib_http::response::bad_request;  
///
/// pub fn handler(req: Request) -> Result<Response> {
///   bad_request()
/// }
/// ```
pub fn bad_request() -> Result<Response> {
    create_response(http::StatusCode::BAD_REQUEST)
}

/// Returns a `Result<spin_sdk::http::Response>` representing a 204 No Content
///
/// # Example
/// ```rust
/// use anyhow::Result;
/// use spin_sdk::{
///  http::{Request, Response},
/// };  
/// use spin_contrib_http::response::no_content;
///
/// pub fn handler(req: Request) -> Result<Response> {
///  no_content()
/// }
/// ```
pub fn no_content() -> Result<Response> {
    create_response(http::StatusCode::NO_CONTENT)
}

/// Returns a `Result<spin_sdk::http::Response>` with desired status code
///
/// # Arguments
///
/// * `code` - The desired status code
///
/// # Example
/// ```rust
/// use anyhow::Result;
/// use spin_sdk::{
///  http::{Request, Response},
/// };  
/// use spin_contrib_http::response::status_code;
///
/// pub fn handler(req: Request) -> Result<Response> {
///   status_code(http::StatusCode::OK)
/// }
/// ```
pub fn status_code(status_code: http::StatusCode) -> Result<Response> {
    create_response(status_code)
}

fn create_response(status_code: http::StatusCode) -> Result<Response> {
    Ok(Response::new(status_code, ()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_content_should_set_status_code_to_204() {
        let sut = no_content().unwrap();
        assert_eq!(sut.status(), &http::StatusCode::NO_CONTENT.as_u16());
    }

    #[test]
    fn bad_request_should_set_status_code_to_400() {
        let sut = bad_request().unwrap();
        assert_eq!(sut.status(), &http::StatusCode::BAD_REQUEST.as_u16());
    }

    #[test]
    fn redirect_should_set_status_code_307_for_temporary() {
        let sut = redirect("http://localhost:3000", false).unwrap();
        assert_eq!(sut.status(), &http::StatusCode::TEMPORARY_REDIRECT.as_u16());
    }

    #[test]
    fn redirect_should_set_status_code_302_for_permanent() {
        let sut = redirect("http://localhost:3000", true).unwrap();
        assert_eq!(sut.status(), &http::StatusCode::PERMANENT_REDIRECT.as_u16());
    }

    #[test]
    fn redirect_should_set_location_header() {
        let target = "http://localhost:3000";
        let sut_permanent = redirect(target, true).unwrap();
        let actual_permanent = sut_permanent
            .header(http::header::LOCATION.as_str())
            .expect("Header LOCATION not present")
            .as_str()
            .expect("Could not convert value to str");
        assert_eq!(actual_permanent, target);

        let sut_temp = redirect(target, false).unwrap();
        let actual_temp = sut_temp
            .header(http::header::LOCATION.as_str())
            .expect("Header LOCATION not present")
            .as_str()
            .expect("Could not convert value to str");
        assert_eq!(actual_temp, target);
    }
}
