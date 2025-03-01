use std::sync::Arc;

use claw::Claw;
use lib_core::config::Config;
use surrealdb::{
    RecordId, Surreal,
    engine::remote::ws::{Client, Ws, Wss},
    opt::auth::Root,
    sql::Id,
};

mod claw;

const RECUR_SPAN_SEC: u64 = 30;

pub(crate) trait DbThing {
    fn get_name() -> &'static str;
    fn rand_id() -> RecordId {
        RecordId::from_table_key(Self::get_name(), Id::rand().to_raw())
    }
    fn id(id: &str) -> RecordId {
        RecordId::from_table_key(Self::get_name(), id)
    }
}

pub struct Datastore {
    db: Arc<Surreal<Client>>,
}

impl Datastore {
    pub async fn init() -> Self {
        let db_url = Config::get_db_url();
        let is_wss = db_url.starts_with("wss://");
        let db = if is_wss {
            Surreal::new::<Wss>(db_url.trim_start_matches("wss://"))
        } else {
            Surreal::new::<Ws>(db_url.trim_start_matches("ws://"))
        };

        let db = db.await.expect("Failed to connect to DB");
        db.signin(Root { username: &Config::get_db_user(), password: &Config::get_db_pass() })
            .await
            .expect("Failed to login database");

        db.use_ns("claw-vault").await.expect("Failed to select ns");
        db.use_db("claw-vault").await.expect("Failed to select db");

        tracing::info!("Database connected");
        Self { db: Arc::new(db) }
    }

    pub async fn schedule_cleaner(&self) {
        let db = self.db.clone();

        tokio::runtime::Handle::current().spawn(async move {
            loop {
                Datastore::__cleaner_job(db.as_ref()).await;
                tokio::time::sleep(tokio::time::Duration::from_secs(RECUR_SPAN_SEC)).await;
            }
        });
    }

    async fn __cleaner_job(db: &Surreal<Client>) {
        let claws: Vec<Claw> = db.select(Claw::get_name()).await.unwrap_or(vec![]);

        for claw in claws.into_iter() {
            let time = chrono::Utc::now().timestamp_millis();

            if time >= claw.expiry_at {
                let res: Result<Option<Claw>, surrealdb::Error> = db.delete(&claw.id).await;
                match res {
                    Ok(_) => {
                        tracing::info!("Deleted expired claw: {}", claw.id.key());
                    }

                    Err(err) => {
                        tracing::error!(
                            message = "Failed to delete claw",
                            id = claw.id.key().to_string(),
                            err = err.to_string(),
                        );
                    }
                };
            }
        }
    }
}
