#[macro_use]
extern crate log;
use azure_core::prelude::*;
use azure_storage::clients::*;
use azure_storage::data_lake::clients::*;
use futures::stream::StreamExt;
use std::convert::TryInto;
use std::error::Error;
use std::num::NonZeroU32;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    let account = std::env::var("ADSL_STORAGE_ACCOUNT")
        .expect("Set env variable ADSL_STORAGE_ACCOUNT first!");
    let master_key = std::env::var("ADSL_STORAGE_MASTER_KEY")
        .expect("Set env variable ADSL_STORAGE_MASTER_KEY first!");

    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));

    let storage_account_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key);

    let data_lake = storage_account_client
        .as_storage_client()
        .as_data_lake_client(account)?;

    let response = data_lake
        .list()
        .max_results(NonZeroU32::new(3).unwrap())
        .execute()
        .await?;

    println!("response == {:?}", response);

    Ok(())
}
