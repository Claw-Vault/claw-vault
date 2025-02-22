pub struct Config {}

impl Config {
    pub fn get_host_addr() -> String {
        let port = env!("PORT");
        format!("[::]:{port}")
    }

    pub fn get_db_url() -> String {
        env!("DATABASE_URL").to_string()
    }

    pub fn get_assets_dir() -> String {
        env!("ASSETS_DIR").to_string()
    }

    pub fn get_template_dir() -> String {
        env!("TEMPLATE_DIR").to_string()
    }
}
