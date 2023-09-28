use crate::types::{Response, ResponseBuilder, StatusCode};

pub(crate) fn respond_with_200() -> anyhow::Result<Response> {
    Ok(ResponseBuilder::new().build())
}

pub(crate) fn respond_with_404() -> anyhow::Result<Response> {
    Ok(ResponseBuilder::new()
        .status_code(StatusCode::NotFound)
        .build())
}
