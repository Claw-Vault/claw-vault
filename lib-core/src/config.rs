pub struct Config {}

impl Config {
    pub fn get_host_addr() -> String {
        let port = std::env::var("PORT").unwrap_or("8080".into());
        format!("[::]:{port}")
    }

    pub fn get_db_url() -> String {
        std::env::var("DATABASE_URL").expect("Missing db url")
    }

    pub fn get_db_user() -> String {
        std::env::var("DATABASE_USER").expect("Missing db user")
    }

    pub fn get_db_pass() -> String {
        std::env::var("DATABASE_PASS").expect("Missing db pass")
    }

    pub fn get_assets_dir() -> String {
        std::env::var("ASSETS_DIR").expect("Missing assets dir")
    }

    pub fn get_template_dir() -> String {
        std::env::var("TEMPLATE_DIR").expect("Missing template dir")
    }
}
