use axum::routing::post;
use axum::Router;

use crate::app::App;

pub mod api;

pub fn bind_routes(router: Router<App>) -> Router<App> {
    router.route("/encrypt", post(api::encrypt)).route("/decrypt", post(api::decrypt))
}
