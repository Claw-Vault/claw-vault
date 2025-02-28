use std::sync::Arc;

use lib_domain::service::Service;

/// Struct to hold instances of common objects
/// in app.
///
/// This struct is passed on to every API path.
pub struct _App {
    service: Service,
    tera: Arc<tera::Tera>,
}

impl _App {
    /// Initializes app with required objects
    ///
    /// Returns [`_App`]
    pub async fn init() -> Self {
        let service = Service::init().await;
        let tera = lib_core::tera_template::setup_tera();
        let tera = Arc::new(tera);
        Self { service, tera }
    }

    pub async fn bootstrap(&self) {
        self.service.ds().schedule_cleaner().await;
        tracing::info!(message = "App bootstrapped");
    }

    pub fn service(&self) -> &Service {
        &self.service
    }

    pub fn tera(&self) -> Arc<tera::Tera> {
        self.tera.clone()
    }
}

pub type App = Arc<_App>;

pub async fn init() -> App {
    let app = _App::init().await;
    Arc::new(app)
}
