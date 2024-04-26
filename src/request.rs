use spin_sdk::http::Request;

const HEADER_SPIN_PATH_INFO: &str = "spin-path-info";

/// returns route segments of the HTTP request.
///
/// If the request was invoked using the root URL, an empty vector is returned
///
/// # Example
/// ```rust
/// use spin_contrib_http::request::get_route_segments;
///
/// let fake_req = http::request::Builder::new()
///    .header("spin-path-info", "/foo/bar/baz")
///    .body(None).unwrap();
///
/// let segments = get_route_segments(&fake_req).unwrap();
/// assert_eq!(segments.len(), 3);
/// assert_eq!(segments[0], "foo");
/// assert_eq!(segments[1], "bar");
/// assert_eq!(segments[2], "baz");
/// ```
pub fn get_route_segments(req: &Request) -> Option<Vec<&str>> {
    let Some(spin_path_header_value) = req.header(HEADER_SPIN_PATH_INFO) else {
        return None;
    };

    let Some(header_value) = spin_path_header_value.as_str() else {
        return None;
    };

    if header_value.is_empty() || header_value == "/" {
        return Some(vec![]);
    };

    let mut segments = header_value.split('/').into_iter().collect::<Vec<&str>>();
    segments.remove(0);
    Some(segments)
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
        let segments = get_route_segments(&req);
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
            let segments = get_route_segments(&req).unwrap();
            assert_eq!(segments.len(), 0);
        }
    }
}
