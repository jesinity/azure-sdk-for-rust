use crate::data_lake::clients::DataLakeClient;
use crate::data_lake::responses::*;
use azure_core::prelude::*;
use azure_core::{headers::add_optional_header, AppendToUrlQuery};
use http::method::Method;
use http::status::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreateFileSystemBuilder<'a> {
    data_lake_client: &'a DataLakeClient,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
}

impl<'a> CreateFileSystemBuilder<'a> {
    pub(crate) fn new(data_lake_client: &'a DataLakeClient) -> Self {
        Self {
            data_lake_client,
            client_request_id: None,
            timeout: None,
        }
    }

    setters! {
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(
        &self,
    ) -> Result<ListFileSystemsResponse, Box<dyn std::error::Error + Sync + Send>> {
        // we clone this so we can add custom
        // query parameters
        let mut url = self.data_lake_client.url().clone();

        url.query_pairs_mut().append_pair("resource", "filesystem");
        self.timeout.append_to_url_query(&mut url);

        debug!("list filesystems url = {}", url);

        let request = self.data_lake_client.prepare_request(
            url.as_str(),
            &Method::PUT,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request
            },
            None,
        )?;

        trace!("request == {:?}", request);

        let response = self
            .data_lake_client
            .http_client()
            .execute_request_check_status(request.0, StatusCode::CREATED)
            .await?;

        Ok((&response).try_into()?)
    }
}
