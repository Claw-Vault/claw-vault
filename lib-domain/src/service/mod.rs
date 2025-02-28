use crate::data::Datastore;

mod vault;

pub struct Service {
    ds: Datastore,
}

impl Service {
    pub async fn init() -> Self {
        let ds = Datastore::init().await;
        Self { ds }
    }

    pub fn ds(&self) -> &Datastore {
        &self.ds
    }
}
