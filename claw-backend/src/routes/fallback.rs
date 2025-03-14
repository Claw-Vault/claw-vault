use axum::response::Redirect;

/// Handler for routes that are not defined
pub async fn fallback_handler() -> Redirect {
    Redirect::temporary("/swagger/#")
}
