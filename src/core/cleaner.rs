use std::{sync::Arc, thread, time::Duration};

use crate::app::App;

use super::dao;

pub fn schedule_cleaner(app: Arc<App>) {
    thread::spawn(|| cleaner(app));
}

async fn cleaner(app: Arc<App>) {
    let (_, db, _) = app.expand();

    loop {
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
        thread::sleep(Duration::from_secs(45));
    }
}
