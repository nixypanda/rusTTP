use crate::types::{HttpRequest, Response, ResponseBuilder, StatusCode};

pub(crate) struct Handler {}

impl Handler {
    pub(crate) fn respond_with_200(&self) -> anyhow::Result<Response> {
        Ok(ResponseBuilder::new().build())
    }

    pub(crate) fn respond_with_404(&self) -> anyhow::Result<Response> {
        Ok(ResponseBuilder::new()
            .status_code(StatusCode::NotFound)
            .build())
    }

    pub(crate) fn respond_with_path_content(
        &self,
        parsed_request: HttpRequest,
    ) -> Result<Response, anyhow::Error> {
        let content = parsed_request.path.strip_prefix("/echo/").unwrap();
        Ok(ResponseBuilder::new().content(content).build())
    }

    pub(crate) fn respond_with_user_agent(
        &self,
        parsed_request: HttpRequest,
    ) -> Result<Response, anyhow::Error> {
        let content = parsed_request.get_header("User-Agent");
        Ok(ResponseBuilder::new()
            .status_code(StatusCode::Ok)
            .content(content)
            .build())
    }
}
