use crate::data_lake::clients::DataLakeClient;
use crate::data_lake::requests::*;
use azure_core::errors::AzureError;
use azure_core::prelude::*;
use bytes::Bytes;
use http::method::Method;
use http::request::{Builder, Request};
use std::sync::Arc;
use url::Url;

pub trait AsFileSystemClient<A: Into<String>> {
    fn as_file_system_client(&self, name: A) -> Result<Arc<FileSystemClient>, url::ParseError>;
}

impl<A: Into<String>> AsFileSystemClient<A> for Arc<DataLakeClient> {
    fn as_file_system_client(&self, name: A) -> Result<Arc<FileSystemClient>, url::ParseError> {
        FileSystemClient::new(self.clone(), name.into())
    }
}

#[derive(Debug, Clone)]
pub struct FileSystemClient {
    data_lake_client: Arc<DataLakeClient>,
    name: String,
    url: Url,
}

impl FileSystemClient {
    pub(crate) fn new(
        data_lake_client: Arc<DataLakeClient>,
        name: String,
    ) -> Result<Arc<Self>, url::ParseError> {
        let url = data_lake_client.url().join(&name)?;

        Ok(Arc::new(Self {
            data_lake_client,
            name,
            url,
        }))
    }

    pub(crate) fn http_client(&self) -> &dyn HttpClient {
        self.data_lake_client.http_client()
    }

    pub(crate) fn url(&self) -> &Url {
        &self.url
    }

    pub(crate) fn prepare_request<'a>(
        &self,
        url: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<Bytes>,
    ) -> Result<(Request<Bytes>, url::Url), AzureError> {
        self.data_lake_client
            .prepare_request(url, method, http_header_adder, request_body)
    }
}
