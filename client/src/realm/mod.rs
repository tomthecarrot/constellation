pub struct Realm {
    realm_id: String,
}
impl Realm {
    pub fn new(realm_id: String) -> Self {
        Self { realm_id }
    }
}

pub struct RealmHandle {}
