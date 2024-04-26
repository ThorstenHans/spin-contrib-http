use spin_sdk::http::Request;

const HEADER_SPIN_PATH_INFO: &str = "spin-path-info";

/// Extensions for spin_sdk::http::Request
pub trait Contrib {
    /// returns route segments of the HTTP request.
    ///
    /// If the request was invoked using the root URL, an empty vector is returned
    ///
    /// # Example
    /// ```rust
    /// use spin_sdk::http::RequestBuilder;
    /// use spin_contrib_http::request::Contrib;
    ///
    /// let fake_req = RequestBuilder::new(spin_sdk::http::Method::Get, "http://foo/bar")
    ///    .header("spin-path-info", "/foo/bar/baz")
    ///    .body(()).build();
    ///
    /// let segments = fake_req.get_route_segments();
    ///
    /// assert_eq!(segments.is_some(), true);
    /// let segments = segments.unwrap();
    /// assert_eq!(segments.len(), 3);
    /// assert_eq!(segments[0], "foo");
    /// assert_eq!(segments[1], "bar");
    /// assert_eq!(segments[2], "baz");
    /// ```
    fn get_route_segments(&self) -> Option<Vec<&str>>;
}

impl Contrib for Request {
    fn get_route_segments(&self) -> Option<Vec<&str>> {
        let spin_path_header_value = self.header(HEADER_SPIN_PATH_INFO)?;
        let header_value = spin_path_header_value.as_str()?;
        if header_value.trim().is_empty() || header_value == "/" {
            return None;
        };

        let mut segments = header_value.split('/').collect::<Vec<&str>>();
        segments.remove(0);
        Some(segments)
    }
}

#[cfg(test)]
mod tests {
    use spin_sdk::http::RequestBuilder;

    use super::*;

    #[test]
    fn get_route_segments_should_return_provided_segments() {
        let req = RequestBuilder::new(spin_sdk::http::Method::Get, "http::/foo.bar")
            .header(HEADER_SPIN_PATH_INFO, "/foo/bar/baz")
            .body(())
            .build();
        let segments = req.get_route_segments();
        assert_eq!(segments.is_some(), true);
        let segments = segments.unwrap();
        println!("{:?}", segments);
        assert_eq!(segments.len(), 3);
        assert_eq!(segments[0], "foo");
        assert_eq!(segments[1], "bar");
        assert_eq!(segments[2], "baz");
    }

    #[test]
    fn get_route_segments_should_empty_vector_for_root_url() {
        let test_data = vec!["/", "", " "];
        for data in test_data {
            let req = RequestBuilder::new(spin_sdk::http::Method::Get, "http://foo.bar")
                .header(HEADER_SPIN_PATH_INFO, data)
                .body(())
                .build();

            assert_eq!(req.get_route_segments(), None);
        }
    }
}
