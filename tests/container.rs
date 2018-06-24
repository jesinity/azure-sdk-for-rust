#![cfg(all(test, feature = "test_e2e"))]
extern crate azure_sdk_for_rust;
extern crate chrono;
extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate log;
extern crate serde;
extern crate tokio_core;
extern crate uuid;

use azure_sdk_for_rust::core::errors::AzureError;
use azure_sdk_for_rust::core::{ContainerNameSupport, LeaseDurationSupport, LeaseIdSupport};
use azure_sdk_for_rust::storage::{
    client::Client,
    container::{PublicAccess, PublicAccessSupport},
};
use tokio_core::reactor::Core;

#[test]
fn lease_container() {
    use azure_sdk_for_rust::storage::client::Container;

    let container_name: &'static str = "azuresdkrustetoets2";

    let (client, mut core) = initialize().unwrap();
    core.run(
        client
            .create()
            .with_container_name(container_name)
            .with_public_access(PublicAccess::Container)
            .finalize(),
    ).unwrap();

    let future = client
        .acquire_lease()
        .with_container_name(container_name)
        .with_lease_duration(30)
        .finalize();
    let res = core.run(future).unwrap();
    let lease_id = res.lease_id;

    let future = client
        .renew_lease()
        .with_container_name(container_name)
        .with_lease_id(&lease_id)
        .finalize();
    let _res = core.run(future).unwrap();

    let future = client
        .release_lease()
        .with_container_name(container_name)
        .with_lease_id(&lease_id)
        .finalize();
    core.run(future).unwrap();

    let cont_delete = client
        .delete()
        .with_container_name(container_name)
        .with_lease_id(&lease_id) // must pass the lease here too
        .finalize();

    core.run(cont_delete).unwrap();
}

fn initialize() -> Result<(Client, Core), AzureError> {
    let account = std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key = std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");
    let core = Core::new()?;

    Ok((Client::new(&account, &master_key)?, core))
}
