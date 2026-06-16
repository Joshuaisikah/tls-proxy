use std::str::from_utf8;

pub fn inspect(bytes: &[u8]) ->Option<(String,String)> {
    let text = from_utf8(bytes).ok()?;
    let mut lines = text.lines();
    let request_line = lines.next()?;
    let parts: Vec<&str> = request_line.split_whitespace().collect();

    for line in lines {
        if line.starts_with("Host") {
            return Some((parts[0].to_string(), parts[1].to_string()));
        }
    }
    None
}

pub fn inspect_response(bytes: &[u8]) -> Option<u16> {
    let text = from_utf8(bytes).ok()?;
    let first_line = text.lines().next()?;
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    parts.get(1)?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inspect_parses_method_and_path() {
        let input = b"GET /api/v1/repos HTTP/1.1\r\nHost: github.com\r\n\r\n";
        assert_eq!(inspect(input), Some(("GET".to_string(), "/api/v1/repos".to_string())));
    }

    #[test]
    fn test_inspect_returns_none_without_host_header() {
        let input = b"GET / HTTP/1.1\r\nAccept: */*\r\n\r\n";
        assert_eq!(inspect(input), None);
    }

    #[test]
    fn test_inspect_response_200() {
        let input = b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
        assert_eq!(inspect_response(input), Some(200));
    }

    #[test]
    fn test_inspect_response_301() {
        let input = b"HTTP/1.1 301 Moved Permanently\r\n\r\n";
        assert_eq!(inspect_response(input), Some(301));
    }

    #[test]
    fn test_inspect_response_invalid_utf8_returns_none() {
        let input = &[0xff, 0xfe, 0x00];
        assert_eq!(inspect_response(input), None);
    }
}