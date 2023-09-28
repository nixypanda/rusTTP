use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, char, crlf, none_of},
    combinator::{opt, recognize, rest},
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
    let (input, body) = opt(rest)(input)?;

    Ok((
        input,
        HttpRequest {
            method,
            path,
            version,
            headers,
            body: body.unwrap_or("").to_string(),
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
            body: "".to_string(),
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
            body: "".to_string(),
        };

        assert_eq!(result, Ok(("", expected)));
    }
    #[test]
    fn test_http_request_parsing_with_headers_and_body() {
        let input =
            "GET /index.html HTTP/1.1\r\nUser-Agent: Go-http-client/1.1\r\n\r\ncontent\nlol";
        let result = http_request(input);
        let expected = HttpRequest {
            method: "GET".to_string(),
            path: "/index.html".to_string(),
            version: "HTTP/1.1".to_string(),
            headers: vec![HttpHeader {
                name: "User-Agent".to_string(),
                value: "Go-http-client/1.1".to_string(),
            }],
            body: "content\nlol".to_string(),
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

    #[test]
    fn test_read_data() {
        let input = "POST /files/ee7zpdMV8GvFJTJOOk39BlAcO0ZoKZcjVoxwUbon HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: Go-http-client/1.1\r\nContent-Length: 1000\r\nContent-Type: text/plain\r\nAccept-Encoding: gzip\r\n\r\nDeUIz6z3IyQnjdGweAju8uZy9LwehxnvsV8cK9rnmsH800sfzkblQ3BZW0DC0UgwnimHl3t32SZeCm2XKcmZ3fgZNbJnJQkFFFI7VyaRc8ZgO8itvypA3oMVlmHslOUUEheLSBWLyLOpGrSwIifLyxduunduTpRJHcigXAl290N5g8PPsY1pSSMvdFPTeUNtkDhhcFyElIvZqvWOqA58SPsmQZ0xEc4gb9IticVARvd64hXGs1uB9u4rfY4J7xhriWwbteiSKQTg8im66mHNhTOESiUhPBcdyPfSo95LsZOJ40nDgOAWdauj8snxo16xV6gBXdzfl8wGPOBDy6dWNbSaNwOvcWloXcTqJ9yUdXOlWd5ql4jFPgR2z127VloyK3CqoqI1Ek759HDeFu4fhJxDtZCwrWKemWKXKmXd2An6KTIRj6XYiygrpMzBHBSmw2Bacg6lfjoOFH93EGtiwcP191ojP4VLs0SsBMHtcOMXjmuz0rFkr3aU8hAh0pIPMttRgo7uwVCEILoa4VuwqBvvqJ3YcLv4z4lvMS2sbHPWjXg22ZUbeJe8Ono3MEUi9Aj0FLO0t1oNGO4HuLjOo1ljuGuhH2Z6HgEASa3owtIVyGioM7xPKwvwuz2UJ8xlcg6TD6r4i7XZ5epMHMdhiPgt17M7Nf5OlR1f6ACmlg34HjKkfwMB1ObAfyMWLUodnQ3HnhdimcSezPVbKKpuoNZewNAztqrBaGiaHiNbALgTzypuUyLfNC61Gf0ua2V3BhxrKBU7SIaFfO32DBH0xRI0ZbFHpl5wCa1w1O5H2KxgiYnMtsZ4933Hg94iWb4fYxg3bX9TrRig9Aab9s2gsz2UVmzkaL5arQKBTxJVZeNpnRWSgcUKiuDJvhzlmzKF4SNLLC9Om80TTUzB1A38SYdpjwr0OXzaFNe2kFTrqnCqIeIsKpEDfOydHYC3PTvxnYMjzpxenRIwYEby5wjGN58C7c8EoMct4b1DKVDs";
        let expected = HttpRequest {
            method: "POST".to_string(),
            path: "/files/ee7zpdMV8GvFJTJOOk39BlAcO0ZoKZcjVoxwUbon".to_string(),
            version: "HTTP/1.1".to_string(),
            headers: vec![
                HttpHeader {
                    name: "Host".to_string(),
                    value: "localhost:4221".to_string(),
                },
                HttpHeader {
                    name: "User-Agent".to_string(),
                    value: "Go-http-client/1.1".to_string(),
                },
                HttpHeader {
                    name: "Content-Length".to_string(),
                    value: "1000".to_string(),
                },
                HttpHeader {
                    name: "Content-Type".to_string(),
                    value: "text/plain".to_string(),
                },
                HttpHeader {
                    name: "Accept-Encoding".to_string(),
                    value: "gzip".to_string(),
                },
            ],
            body: "DeUIz6z3IyQnjdGweAju8uZy9LwehxnvsV8cK9rnmsH800sfzkblQ3BZW0DC0UgwnimHl3t32SZeCm2XKcmZ3fgZNbJnJQkFFFI7VyaRc8ZgO8itvypA3oMVlmHslOUUEheLSBWLyLOpGrSwIifLyxduunduTpRJHcigXAl290N5g8PPsY1pSSMvdFPTeUNtkDhhcFyElIvZqvWOqA58SPsmQZ0xEc4gb9IticVARvd64hXGs1uB9u4rfY4J7xhriWwbteiSKQTg8im66mHNhTOESiUhPBcdyPfSo95LsZOJ40nDgOAWdauj8snxo16xV6gBXdzfl8wGPOBDy6dWNbSaNwOvcWloXcTqJ9yUdXOlWd5ql4jFPgR2z127VloyK3CqoqI1Ek759HDeFu4fhJxDtZCwrWKemWKXKmXd2An6KTIRj6XYiygrpMzBHBSmw2Bacg6lfjoOFH93EGtiwcP191ojP4VLs0SsBMHtcOMXjmuz0rFkr3aU8hAh0pIPMttRgo7uwVCEILoa4VuwqBvvqJ3YcLv4z4lvMS2sbHPWjXg22ZUbeJe8Ono3MEUi9Aj0FLO0t1oNGO4HuLjOo1ljuGuhH2Z6HgEASa3owtIVyGioM7xPKwvwuz2UJ8xlcg6TD6r4i7XZ5epMHMdhiPgt17M7Nf5OlR1f6ACmlg34HjKkfwMB1ObAfyMWLUodnQ3HnhdimcSezPVbKKpuoNZewNAztqrBaGiaHiNbALgTzypuUyLfNC61Gf0ua2V3BhxrKBU7SIaFfO32DBH0xRI0ZbFHpl5wCa1w1O5H2KxgiYnMtsZ4933Hg94iWb4fYxg3bX9TrRig9Aab9s2gsz2UVmzkaL5arQKBTxJVZeNpnRWSgcUKiuDJvhzlmzKF4SNLLC9Om80TTUzB1A38SYdpjwr0OXzaFNe2kFTrqnCqIeIsKpEDfOydHYC3PTvxnYMjzpxenRIwYEby5wjGN58C7c8EoMct4b1DKVDs".to_string(),
        };
        let result = http_request(input);
        assert_eq!(result, Ok(("", expected)));
    }
}
