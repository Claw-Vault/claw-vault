use std::{sync::Arc, time::Duration};

use sea_orm::ActiveEnum;

use crate::app::App;

use super::dao;

/// Schedules cleaner to run periodically
///
/// See: [`cleaner`]
///
/// Returns [`tokio::task::JoinHandle`]
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

/// Cleans all the expired claws
///
/// - Loops through all the claws
/// - Delete claw if current time seconds are more the requested validity [`super::models::claw::ValidDuration`]
///
/// This function runs periodically every 45 seconds
async fn cleaner(app: Arc<App>) {
    // get db connection
    let (_, db, _) = app.expand();

    // get claws -> empty array if error
    let claws = dao::get_all_claws(&db).await.unwrap_or_else(|_| {
        tracing::warn!("Failed to fetch all claws");
        vec![]
    });

    // loop through the claws
    for claw in claws {
        let time = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs() as i64;
        let expire = claw.created_at.timestamp() + claw.validity.into_value();
        if time > expire {
            // delete as claw has expired
            match dao::delete_claw(claw.id.clone(), &db).await {
                Ok(_) => tracing::info!("Deleted expired claw: {}", claw.id),
                Err(_) => tracing::warn!("Failed to delete expired claw: {}", claw.id),
            };
        }
    }
}
