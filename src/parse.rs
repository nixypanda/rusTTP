use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, char, crlf, none_of},
    combinator::{opt, recognize},
    multi::{many0, separated_list0, separated_list1},
    IResult,
};

use crate::types::{HttpHeader, HttpRequest};

fn is_token(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '-' || c == '_'
}

fn parse_header_name(input: &str) -> IResult<&str, &str> {
    recognize(many0(take_while1(is_token)))(input)
}

fn parse_header_value(input: &str) -> IResult<&str, &str> {
    recognize(many0(take_while1(|c| c != '\r' && c != '\n')))(input)
}

fn parse_http_header(input: &str) -> IResult<&str, HttpHeader> {
    let (input, name) = parse_header_name(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, value) = parse_header_value(input)?;

    Ok((
        input,
        HttpHeader {
            name: name.to_string(),
            value: value.to_string(),
        },
    ))
}

fn parse_http_headers(input: &str) -> IResult<&str, Vec<HttpHeader>> {
    separated_list0(crlf, parse_http_header)(input)
}

fn http_request(input: &str) -> IResult<&str, HttpRequest> {
    let (input, method) = http_method(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, path) = http_path(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, version) = http_version(input)?;
    let (input, _) = crlf(input)?;
    let (input, headers) = parse_http_headers(input)?;
    let (input, _) = opt(crlf)(input)?;
    let (input, _) = opt(crlf)(input)?;

    Ok((
        input,
        HttpRequest {
            method,
            path,
            version,
            headers,
        },
    ))
}

fn http_method(input: &str) -> IResult<&str, String> {
    separated_list1(char('/'), alpha1)(input).map(|(input, method_parts)| {
        (
            input,
            method_parts
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join("/"),
        )
    })
}

fn http_path(input: &str) -> IResult<&str, String> {
    let (input, path) = many0(none_of(" \t\n"))(input)?;
    Ok((input, path.into_iter().collect::<String>()))
}

fn http_version(input: &str) -> IResult<&str, String> {
    tag("HTTP/1.1")(input).map(|(input, version)| (input, version.to_string()))
}

pub(crate) fn parse_request(input: &str) -> anyhow::Result<HttpRequest> {
    match http_request(input) {
        Ok((_, request)) => Ok(request),
        Err(_) => anyhow::bail!("Invalid HTTP request"),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_http_request_parsing_simple() {
        let input = "GET /index.html HTTP/1.1\r\n";
        let result = http_request(input);
        let expected = HttpRequest {
            method: "GET".to_string(),
            path: "/index.html".to_string(),
            version: "HTTP/1.1".to_string(),
            headers: vec![],
        };

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_http_request_parsing_with_headers() {
        let input = "GET /index.html HTTP/1.1\r\nUser-Agent: Go-http-client/1.1\r\n\r\n";
        let result = http_request(input);
        let expected = HttpRequest {
            method: "GET".to_string(),
            path: "/index.html".to_string(),
            version: "HTTP/1.1".to_string(),
            headers: vec![HttpHeader {
                name: "User-Agent".to_string(),
                value: "Go-http-client/1.1".to_string(),
            }],
        };

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_http_header_parsing() {
        let input = "User-Agent: Go-http-client/1.1";
        let expected = HttpHeader {
            name: "User-Agent".to_string(),
            value: "Go-http-client/1.1".to_string(),
        };
        let result = parse_http_header(input).unwrap();

        assert_eq!(result, ("", expected));
    }
}
