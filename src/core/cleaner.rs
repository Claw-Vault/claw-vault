use std::{sync::Arc, time::Duration};

use crate::app::App;

use super::dao;

pub async fn schedule_cleaner(
    app: Arc<App>,
    notify: Arc<tokio::sync::Notify>,
) -> tokio::task::JoinHandle<()> {
    tokio::runtime::Handle::current().spawn_blocking(move || {
        tokio::runtime::Handle::current().block_on(async move {
            tracing::info!("Spawning cleaner");

            loop {
                tokio::select! {
                    _ = cleaner(app.clone()) => {},
                    _ = notify.notified() => break,
                }
                tokio::select! {
                    _ = tokio::time::sleep(Duration::from_secs(45)) => {},
                    _ = notify.notified() => break,
                }
            }

            tracing::info!("Terminated cleaner");
            app.terminate().await
        });
    })
}

async fn cleaner(app: Arc<App>) {
    let (_, db, _) = app.expand();

    let claws = dao::get_all_claws(&db).await.unwrap_or_else(|_| {
        tracing::warn!("Failed to fetch all claws");
        vec![]
    });

    for claw in claws {
        let time = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs() as i64;
        let expire = claw.created_at.timestamp() + claw.validity as i64;
        if time > expire {
            match dao::delete_claw(claw.id.clone(), &db).await {
                Ok(_) => tracing::info!("Deleted expired claw: {}", claw.id),
                Err(_) => tracing::warn!("Failed to delete expired claw: {}", claw.id),
            };
        }
    }
}
