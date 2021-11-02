mod link;
mod realm;

use eyre::{eyre, Result, WrapErr};
use link::{Link, Session};
use realm::{Realm, RealmHandle};

// TODO: Make this an actual channel or something

#[non_exhaustive]
pub enum Endpoint {
    Local(RealmHandle),
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
pub struct RealmClient {
    session: Session,
    link: Link,
    local_realm: Realm,
}
impl RealmClient {
    fn new(builder: RealmClientBuilder) -> Result<Self> {
        let local_realm = Realm::new(builder.realm_id);
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
