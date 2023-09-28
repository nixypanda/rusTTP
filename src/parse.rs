use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, crlf, none_of},
    multi::{many0, separated_list1},
    IResult,
};

use crate::types::HttpRequest;

fn http_request(input: &str) -> IResult<&str, HttpRequest> {
    let (input, method) = http_method(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, path) = http_path(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, version) = http_version(input)?;
    let (input, _) = crlf(input)?;

    Ok((
        input,
        HttpRequest {
            method,
            path,
            version,
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
        };

        assert_eq!(result, Ok(("", expected)));
    }
}
