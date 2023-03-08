use http::response::Builder;

/// Use this enum to control SameSite property when creating cookies
pub enum SameSite {
    /// Set the SameSite flag to Strict
    Strict,
    /// Set the SameSite flag to Lax
    Lax,
    /// This will also set the Secure flag
    None,
}

/// Send a cookie along with the response
/// 
/// # Arguments
/// 
/// * `builder` - The `http::response::Builder` to add the cookie to
/// * `name` - The name of the cookie
/// * `value` - The value of the cookie
/// * `secure` - Whether or not the cookie should be sent over HTTPS only
/// * `http_only` - Whether or not the cookie should be accessible via JavaScript
/// * `same_site` - The SameSite property of the cookie
/// 
/// # Example
/// ```rust
/// use spin_contrib_http::response::cookies::{SameSite, add_cookie};
/// let builder = http::response::Builder::new();
///
/// let name = "my_cookie";
/// let value = "my_value";
/// 
/// let secure = true;
/// let http_only = true;
/// 
/// let same_site = SameSite::Strict;
/// 
/// let builder = add_cookie(builder, name, value, secure, http_only, same_site);
/// let b : Option<bytes::Bytes> = Some("Hello World".into());
/// let response = builder.body(b);
/// ```
pub fn add_cookie(builder: Builder, name: &str, value: &str, secure: bool, http_only: bool, same_site: SameSite) -> Builder {
    let mut value = value.to_string();
    let mut secure = secure;
    
    if http_only {
        value.push_str("; HttpOnly");
    }
    match same_site {
        SameSite::Strict => value.push_str("; SameSite=Strict"),
        SameSite::Lax => value.push_str("; SameSite=Lax"),
        SameSite::None => {
            secure = true;
            value.push_str("; SameSite=None");
        }
    }
    if secure {
        value.push_str("; Secure");
    }
    builder.header(
        http::header::SET_COOKIE,
        format!("{}={}", name, value).as_str(),
    )
}
