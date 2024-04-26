use anyhow::Result;

use spin_sdk::http::Response;
const CONTENT_TYPE_TEXT: &str = "text/plain";

/// Returns a `Result<spin_sdk::http::Response>` representing a 500 Internal Server Error
/// with the provided error message
///
/// # Arguments
///
/// * `err` - The error message that should be returned in the HTTP response body
/// * `content_type` - The content type of the HTTP response body (Optional) (defaults to `text/plain`)
///
/// # Example
/// ```rust
/// use anyhow::Result;
/// use spin_sdk::{
///  http::{Request, Response},
/// };  
/// use spin_contrib_http::response::internal_server_error;
///
/// fn handler(req: Request) -> Result<Response> {
///    let err = Some("Something went wrong".into());
///    internal_server_error(err, None)
/// }
/// ```
pub fn internal_server_error(
    body: Option<bytes::Bytes>,
    content_type: Option<&str>,
) -> Result<Response> {
    let content_type = content_type.unwrap_or(CONTENT_TYPE_TEXT);
    Ok(Response::builder()
        .status(http::StatusCode::INTERNAL_SERVER_ERROR)
        .header(http::header::CONTENT_TYPE.as_str(), content_type)
        .body(body)
        .build())
}

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

/// Returns a `Result<spin_sdk::http::Response>` representing a 405 Method Not Allowed
///
/// # Example
/// ```rust
/// use anyhow::Result;
/// use spin_sdk::{
///  http::{Request, Response},
/// };  
/// use spin_contrib_http::response::method_not_allowed;
///
/// pub fn handler(req: Request) -> Result<Response> {
///   method_not_allowed()
/// }
/// ```
pub fn method_not_allowed() -> Result<Response> {
    create_response(http::StatusCode::METHOD_NOT_ALLOWED)
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

/// Returns a `Result<spin_sdk::http::Response>` representing a 404 Not Found
///
/// # Example
/// ```rust
/// use anyhow::Result;
/// use spin_sdk::{
///  http::{Request, Response},
/// };  
/// use spin_contrib_http::response::not_found;
///
/// pub fn handler(req: Request) -> Result<Response> {
///  not_found()
/// }
/// ```
pub fn not_found() -> Result<Response> {
    create_response(http::StatusCode::NOT_FOUND)
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
pub fn status_code(code: http::StatusCode) -> Result<Response> {
    create_response(code)
}

fn create_response(s: http::StatusCode) -> Result<Response> {
    Ok(Response::builder().status(s.as_u16()).body(()).build())
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
    fn not_found_should_set_status_code_to_404() {
        let sut = not_found().unwrap();
        assert_eq!(sut.status(), &http::StatusCode::NOT_FOUND.as_u16());
    }

    #[test]
    fn bad_request_should_set_status_code_to_400() {
        let sut = bad_request().unwrap();
        assert_eq!(sut.status(), &http::StatusCode::BAD_REQUEST.as_u16());
    }

    #[test]
    fn method_not_allowed_should_set_status_code_to_405() {
        let sut = method_not_allowed().unwrap();
        assert_eq!(sut.status(), &http::StatusCode::METHOD_NOT_ALLOWED.as_u16());
    }

    #[test]
    fn internal_server_error_should_set_status_code_to_500() {
        let err = String::from("some err");
        let sut = internal_server_error(Some(err.into()), None).unwrap();
        assert_eq!(
            sut.status(),
            &http::StatusCode::INTERNAL_SERVER_ERROR.as_u16()
        );
    }

    #[test]
    fn internal_server_error_should_set_content_type_to_json_by_default() {
        let err = String::from("some err");
        let sut = internal_server_error(Some(err.into()), None).unwrap();
        let actual = sut
            .header(http::header::CONTENT_TYPE.as_str())
            .unwrap()
            .as_str()
            .unwrap();
        assert_eq!(actual, CONTENT_TYPE_TEXT);
    }

    #[test]
    fn internal_server_error_should_set_error_message_as_body() {
        let err = String::from("{ \"error\": \"some err\" }");
        let sut = internal_server_error(Some(err.into()), None).unwrap();
        let body = sut.body().as_ref();
        let actual = std::str::from_utf8(&body[..]).unwrap();
        assert_eq!(actual, "{ \"error\": \"some err\" }");
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
