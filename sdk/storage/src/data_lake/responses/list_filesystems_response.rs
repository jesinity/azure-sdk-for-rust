use crate::data_lake::FileSystem;
use crate::{data_lake::file_system::FileSystemList, ContinuationToken};
use azure_core::{errors::AzureError, headers::CommonStorageResponseHeaders, prelude::NextMarker};
use bytes::Bytes;
use http::Response;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct ListFilesystemsResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub file_systems: Vec<FileSystem>,
    pub next_marker: Option<NextMarker>,
}

impl TryFrom<&Response<Bytes>> for ListFilesystemsResponse {
    type Error = AzureError;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        debug!("{}", std::str::from_utf8(response.body())?);
        debug!("{:?}", response.headers());

        let file_system_list: FileSystemList = response.try_into()?;

        Ok(ListFilesystemsResponse {
            common_storage_response_headers: response.headers().try_into()?,
            file_systems: file_system_list.file_systems,
            next_marker: NextMarker::from_header_optional(response.headers())?,
        })
    }
}