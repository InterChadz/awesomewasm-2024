use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage};
use cosmwasm_std::OwnedDeps;
use neutron_sdk::bindings::query::NeutronQuery;
use std::marker::PhantomData;

pub fn mock_neutron_dependencies() -> OwnedDeps<MockStorage, MockApi, MockQuerier, NeutronQuery> {
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: MockQuerier::default(),
        custom_query_type: PhantomData,
    }
}
