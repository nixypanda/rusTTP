use crate::types::{Response, ResponseBuilder};

pub(crate) fn respond_with_200() -> anyhow::Result<Response> {
    Ok(ResponseBuilder::new().build())
}
