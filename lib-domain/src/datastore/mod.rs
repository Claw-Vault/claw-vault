use std::str::FromStr;

use claw::Claw;
use lib_core::config::Config;

mod claw;

const RECUR_SPAN_SEC: u64 = 30;

pub struct Datastore {
    db: sqlx::SqlitePool,
}

impl Datastore {
    pub async fn init() -> Self {
        let opts = sqlx::sqlite::SqliteConnectOptions::from_str(&Config::get_db_url())
            .expect("Failed to init sqlit connection options")
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .create_if_missing(true);

        let db = sqlx::SqlitePool::connect_with(opts).await.expect("Failed init Sqlite pool");
        tracing::info!("Database connected");

        sqlx::migrate!("./migrations").run(&db).await.expect("Failed to run migrations");
        tracing::info!("Migrations ran");

        Self { db }
    }

    pub async fn schedule_cleaner(&self) {
        let db = self.db.clone();

        tokio::runtime::Handle::current().spawn(async move {
            loop {
                Datastore::__cleaner_job(&db).await;
                tokio::time::sleep(tokio::time::Duration::from_secs(RECUR_SPAN_SEC)).await;
            }
        });
    }

    async fn __cleaner_job(db: &sqlx::SqlitePool) {
        let claws: Vec<Claw> =
            sqlx::query_as(r#"SELECT * FROM claw"#).fetch_all(db).await.unwrap_or(vec![]);

        for claw in claws.into_iter() {
            let time = chrono::Utc::now().timestamp_millis();

            if time >= claw.expiry_at {
                match Self::__delete_claw(db, &claw.id).await {
                    Ok(res) => {
                        if let Some(_) = res {
                            tracing::info!("Deleted expired claw: {}", claw.id);
                        } else {
                            tracing::warn!(
                                "Attempted to deleted but rows affected 0 for expired claw: {}",
                                claw.id
                            );
                        }
                    }

                    Err(err) => {
                        tracing::error!(
                            message = "Failed to delete claw",
                            id = claw.id,
                            err = err.to_string(),
                        );
                    }
                };
            }
        }
    }
}
