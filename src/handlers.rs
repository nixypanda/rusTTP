use std::path::PathBuf;

use crate::{
    file,
    types::{HttpRequest, Response, ResponseBuilder, StatusCode},
};

pub(crate) struct Handler {
    env_dir: String,
}

impl Handler {
    pub(crate) fn new(env_dir: String) -> Self {
        Self { env_dir }
    }

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

    pub(crate) fn respond_with_file(
        &self,
        parsed_request: HttpRequest,
    ) -> Result<Response, anyhow::Error> {
        let filename = parsed_request.path.strip_prefix("/files/").unwrap();
        let file_path = PathBuf::from(self.env_dir.clone()).join(filename);
        let file_contents = file::get_file_contents(file_path)?;

        Ok(ResponseBuilder::new()
            .status_code(StatusCode::Ok)
            .file_content(&file_contents)
            .build())
    }

    pub(crate) fn store_file(
        &self,
        parsed_request: HttpRequest,
    ) -> Result<Response, anyhow::Error> {
        let filename = parsed_request.path.strip_prefix("/files/").unwrap();
        let file_path = PathBuf::from(self.env_dir.clone()).join(filename);
        let _ = file::store_file(file_path, &parsed_request.body);

        Ok(ResponseBuilder::new()
            .status_code(StatusCode::Created)
            .build())
    }
}
