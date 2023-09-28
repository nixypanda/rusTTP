use crate::types::{HttpRequest, Response, ResponseBuilder, StatusCode};

pub(crate) fn respond_with_200() -> anyhow::Result<Response> {
    Ok(ResponseBuilder::new().build())
}

pub(crate) fn respond_with_404() -> anyhow::Result<Response> {
    Ok(ResponseBuilder::new()
        .status_code(StatusCode::NotFound)
        .build())
}

pub(crate) fn respond_with_path_content(
    parsed_request: HttpRequest,
) -> Result<Response, anyhow::Error> {
    let content = parsed_request.path.strip_prefix("/echo/").unwrap();
    println!("Responding with {}", content);
    Ok(ResponseBuilder::new().content(content).build())
}
