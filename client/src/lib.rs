#![allow(clippy::diverging_sub_expression)]
#![feature(generic_associated_types)]

pub mod action;
pub mod baseline;
pub mod contract;
pub mod engine;
pub mod link;
pub mod object;
pub mod realm;
pub mod time;

use baseline::BaselineKind;
use contract::ffi_testing_contract::FfiDefaultContract;
use contract::properties::states::{StateHandle, StateId};
use contract::{Contract, ContractDataHandle};
use eyre::{eyre, Result};
use link::{Link, RealmServerHandle, Session};
use object::ObjectHandle;
use realm::{Realm, RealmID};

pub use engine::Engine;

#[non_exhaustive]
pub enum Endpoint {
    Local(RealmServerHandle),
    Wss(String, u16),
    Tcp(String, u16),
}

// This could just be directly args on RealmClient::new(), but we expect building will
// be more complicated in the future.
pub struct RealmClientBuilder {
    endpoint: Endpoint,
    realm_id: String,
}
impl RealmClientBuilder {
    pub fn create(endpoint: Endpoint, realm_id: String) -> Self {
        Self { endpoint, realm_id }
    }

    pub fn build(self) -> Result<RealmClient> {
        RealmClient::new(self)
    }
}

/// Represents an API client for the realm.
#[allow(unused)]
pub struct RealmClient {
    session: Session,
    link: Link,
    local_realm: Realm,
}
impl RealmClient {
    fn new(builder: RealmClientBuilder) -> Result<Self> {
        let local_realm = Realm::new(RealmID::new(builder.realm_id));
        let result = match builder.endpoint {
            Endpoint::Local(handle) => {
                let link = Link::new(handle);
                let session = link.create_session();
                Self {
                    link,
                    session,
                    local_realm,
                }
            }
            _ => return Err(eyre!("We only support local endpoints!")),
        };
        Ok(result)
    }
}

#[no_mangle]
pub unsafe extern "C" fn teleportal_engine_init() -> *mut Engine {
    let mut engine = Engine::new(Realm::new(RealmID::new(String::from("main"))), None).0;
    &mut engine
}

#[no_mangle]
pub unsafe extern "C" fn teleportal_engine_get_contract_ffi_testing(
    engine: &mut Engine,
) -> *const FfiDefaultContract {
    let contract: FfiDefaultContract = engine
        .realm()
        .baseline_mut(BaselineKind::Fork)
        .register_contract()
        .expect("Contract failed to register");
    &contract
}

#[no_mangle]
pub extern "C" fn teleportal_engine_create_object(
    engine: &Engine,
    contract: &FfiDefaultContract,
) -> *const ObjectHandle {
    let object_result = engine
        .realm()
        .baseline(BaselineKind::Fork)
        .object_create(contract, None, None);
    let object = object_result.expect("Object could not be created.");
    &mut object
}

#[no_mangle]
pub extern "C" fn teleportal_engine_get_state_handle_u8(
    engine: &Engine,
    object_handle: &ObjectHandle,
    contract_handle: &ContractDataHandle,
    state_idx: usize,
) -> *const StateHandle<u8> {
    if let Ok(object) = engine
        .realm()
        .baseline(BaselineKind::Fork)
        .object(*object_handle)
    {
        let state_id = StateId::new(state_idx, *contract_handle);
        let state_handle = object.state(state_id);
        &state_handle
    } else {
        panic!("TODO");
    }
}

#[no_mangle]
pub extern "C" fn teleportal_engine_get_state_value_u8(
    engine: Box<Engine>,
    state_handle: StateHandle<u8>,
) -> u8 {
    let value = engine
        .realm()
        .baseline(BaselineKind::Fork)
        .state(state_handle)
        .unwrap();
    value.0
}
