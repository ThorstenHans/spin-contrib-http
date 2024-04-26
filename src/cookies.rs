use spin_sdk::http::{Response, ResponseBuilder};

/// Use this enum to control SameSite property when creating cookies
pub enum SameSite {
    /// Set the SameSite flag to Strict
    Strict,
    /// Set the SameSite flag to Lax
    Lax,
    /// This will also set the Secure flag
    None,
}

/// Representation of a cookie
pub struct Cookie {
    /// The name of the cookie
    name: &'static str,
    /// The value of the cookie
    value: &'static str,
    /// Whether or not the cookie should be sent over HTTPS only
    secure: bool,
    /// Whether or not the cookie should be accessible via JavaScript
    http_only: bool,
    /// The SameSite property of the cookie
    same_site: SameSite,
}

/// Trait for conversion into SameSite
pub trait IntoSameSite {
    /// converts self into SameSite
    fn into_same_site(self) -> SameSite;
}

impl IntoSameSite for SameSite {
    fn into_same_site(self) -> SameSite {
        self
    }
}
impl IntoSameSite for &str {
    fn into_same_site(self) -> SameSite {
        match self.trim().to_lowercase().as_str() {
            "strict" => SameSite::Strict,
            "lax" => SameSite::Lax,
            _ => SameSite::None,
        }
    }
}

impl Cookie {
    /// Creates a new Cookie
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the cookie
    /// * `value` - The value of the cookie
    /// * `secure` - Whether or not the cookie should be sent over HTTPS only
    /// * `http_only` - Whether or not the cookie should be accessible via JavaScript
    /// * `same_site` - The SameSite property of the cookie
    pub fn new(
        name: &'static str,
        value: &'static str,
        secure: bool,
        http_only: bool,
        same_site: impl IntoSameSite,
    ) -> Self {
        Cookie {
            name,
            value,
            secure,
            http_only,
            same_site: same_site.into_same_site(),
        }
    }
}

impl ToString for Cookie {
    fn to_string(&self) -> String {
        let mut value = self.value.to_string();
        let mut secure = self.secure;

        if self.http_only {
            value.push_str("; HttpOnly");
        }
        match self.same_site {
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
        format!("{}={}", self.name, value)
    }
}

/// Trait for adding Cookie Support to spin_sdk::http::ResponseBuilder
pub trait CookieResponseBuilder {
    /// Build an HTTP response with a single cookie
    fn build_with_cookie(&mut self, cookie: Cookie) -> Response;
}

impl CookieResponseBuilder for ResponseBuilder {
    fn build_with_cookie(&mut self, cookie: Cookie) -> Response {
        self.header(http::header::SET_COOKIE.as_str(), cookie.to_string());
        self.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_cookie_should_add_corresponding_header() {
        let name = "a";
        let value = "b";
        let cookie = Cookie::new(name, value, false, false, SameSite::Strict);
        let sut = ResponseBuilder::new(200).build_with_cookie(cookie);

        assert_eq!(
            sut.header(http::header::SET_COOKIE.as_str()).is_some(),
            true
        );
    }

    #[test]
    fn add_cookie_should_set_proper_header_value() {
        let name = "a";
        let value = "b";
        let expected = format!("{}={}; SameSite=Strict", name, value);

        let cookie = Cookie::new(name, value, false, false, SameSite::Strict);
        let sut = ResponseBuilder::new(200).build_with_cookie(cookie);

        let actual = sut
            .header(http::header::SET_COOKIE.as_str())
            .unwrap()
            .as_str()
            .unwrap();

        assert_eq!(actual, &expected);
    }

    #[test]
    fn add_cookie_should_add_secure_flag_when_same_site_is_none() {
        let name = "a";
        let value = "b";
        let expected = format!("{}={}; SameSite=None; Secure", name, value);

        let cookie = Cookie::new(name, value, false, false, SameSite::None);
        let sut = ResponseBuilder::new(200).build_with_cookie(cookie);

        let actual = sut
            .header(http::header::SET_COOKIE.as_str())
            .unwrap()
            .as_str()
            .unwrap();
        assert_eq!(actual, &expected);
    }
}
