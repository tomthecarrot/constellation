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

use contract::properties::states::{StateHandle, StateId};
use contract::ContractDataHandle;
use eyre::{eyre, Result};
use link::{Link, RealmServerHandle, Session};
use object::ObjectHandle;
use realm::{Realm, RealmID};

pub use engine::Engine;

use lazy_mut::lazy_mut;

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

// #[cxx::bridge]
// mod ffi {
//     extern "Rust" {
//         type Engine;
//         fn init() -> Engine;
//         fn get_u8() -> u8;
//     }
// }

// lazy_mut! {
//     static mut ENGINE: Engine = Engine::new(Realm::new(RealmID::new(String::from("main"))), None).0;
// }

#[repr(C)]
pub struct EngineOwnedByC {
    _engine: Engine,
}

#[no_mangle]
pub unsafe extern "C" fn teleportal_engine_init() -> Box<Engine> {
    let engine = Engine::new(Realm::new(RealmID::new(String::from("main"))), None).0;
    Box::from(engine)
}

#[no_mangle]
pub extern "C" fn teleportal_engine_get_u8() -> u8 {
    10
}

#[no_mangle]
pub extern "C" fn teleportal_engine_get_state_handle_u8(
    engine: Box<Engine>,
    object_handle: Box<ObjectHandle>,
    contract_handle: Box<ContractDataHandle>,
    state_idx: usize,
) -> Box<StateHandle<u8>> {
    if let Ok(object) = engine
        .realm()
        .baseline(baseline::BaselineKind::Fork)
        .object(*object_handle)
    {
        let state_id = StateId::new(state_idx, *contract_handle);
        let state_handle = object.state(state_id);
        Box::from(state_handle)
    } else {
        panic!("TODO");
    }
}

// #[no_mangle]
// pub unsafe fn tp_get_state_value_u8(object_handle: ObjectHandle, state_handle: usize) -> u8 {
//     if let Ok(state) = common_realm_client
//         .local_realm
//         .baseline(baseline::BaselineKind::Fork)
//         .state(state_handle)
//     {
//         state.0
//     }
// }
