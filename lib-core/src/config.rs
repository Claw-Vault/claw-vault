pub struct Config {}

impl Config {
    pub fn get_host_addr() -> String {
        let port = std::env::var("PORT").unwrap_or("8080".into());
        format!("[::]:{port}")
    }

    pub fn get_db_url() -> String {
        std::env::var("DATABASE_URL").expect("Missing db url")
    }

    pub fn get_migrations_path() -> String {
        std::env::var("DB_MIGRATION_PATH").expect("Missing db migration path")
    }
}
