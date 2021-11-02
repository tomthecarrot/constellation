//! TODO!: This module just has `Link` only use rchannels. We need it to work
//! across multiple backends instead.

use crate::realm::RealmHandle;

/// Handles communication over the network/process/channel. Multiple
/// `RealmClient`s can be multiplexed over the `Link`.
pub struct Link {
    handle: RealmHandle,
}
impl Link {
    pub fn new(handle: RealmHandle) -> Self {
        Link { handle }
    }

    /// Creates a new session
    pub fn create_session(&self) -> Session {
        Session {
            secret: rand::random(),
        }
    }
}

/// Contains necessary information for authentication of a Realm Client. Only
/// useful with the context of a particular RealmServer
pub struct Session {
    secret: u64, //TODO: Make this use OAuth2 or something
}
