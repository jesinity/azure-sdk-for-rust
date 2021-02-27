use crate::clients::StorageClient;
use crate::data_lake::{requests::*, responses::*};
use azure_core::errors::AzureError;
use azure_core::prelude::*;
use bytes::Bytes;
use http::method::Method;
use http::request::{Builder, Request};
use std::sync::Arc;

const DEFAULT_DNS_SUFFIX: &str = "dfs.core.windows.net";

pub trait AsDataLakeClient {
    fn as_data_lake_client<A: Into<String>>(&self, account: A) -> Arc<DataLakeClient>;
    fn as_data_lake_client_with_custom_dns_suffix<DS: Into<String>, A: Into<String>>(
        &self,
        account: A,
        dns_suffix: DS,
    ) -> Arc<DataLakeClient>;
}

impl AsDataLakeClient for Arc<StorageClient> {
    fn as_data_lake_client<A: Into<String>>(&self, account: A) -> Arc<DataLakeClient> {
        DataLakeClient::new(self.clone(), account.into(), DEFAULT_DNS_SUFFIX.to_owned())
    }

    fn as_data_lake_client_with_custom_dns_suffix<DS: Into<String>, A: Into<String>>(
        &self,
        account: A,
        dns_suffix: DS,
    ) -> Arc<DataLakeClient> {
        DataLakeClient::new(self.clone(), account.into(), dns_suffix.into())
    }
}

#[derive(Debug, Clone)]
pub struct DataLakeClient {
    storage_client: Arc<StorageClient>,
    account: String,
    dns_suffix: String,
}

impl DataLakeClient {
    pub(crate) fn new(
        storage_client: Arc<StorageClient>,
        account: String,
        dns_suffix: String,
    ) -> Arc<Self> {
        Arc::new(Self {
            storage_client,
            account,
            dns_suffix,
        })
    }

    pub fn dns_suffix(&self) -> &str {
        &self.dns_suffix
    }

    pub(crate) fn http_client(&self) -> &dyn HttpClient {
        self.storage_client.storage_account_client().http_client()
    }

    pub(crate) fn storage_client(&self) -> &StorageClient {
        &self.storage_client
    }

    pub(crate) fn data_lake_url(&self) -> Result<url::Url, url::ParseError> {
        url::Url::parse(&format!("https://{}.{}", self.account, self.dns_suffix))
    }

    pub fn list(&self) -> ListFilesystemsBuilder {
        ListFilesystemsBuilder::new(self)
    }

    pub(crate) fn prepare_request<'a>(
        &self,
        url: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<Bytes>,
    ) -> Result<(Request<Bytes>, url::Url), AzureError> {
        self.storage_client
            .prepare_request(url, method, http_header_adder, request_body)
    }
}
