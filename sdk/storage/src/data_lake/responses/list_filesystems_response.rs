use azure_core::errors::AzureError;
use bytes::Bytes;
use http::Response;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct ListFilesystemsResponse {
    //pub request_id: RequestId,
}

impl TryFrom<&Response<Bytes>> for ListFilesystemsResponse {
    type Error = AzureError;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        println!("{}", std::str::from_utf8(response.body())?);
        println!("{:?}", response.headers());

        Ok(ListFilesystemsResponse {
            //request_id: "".into(),
        })
    }
}
