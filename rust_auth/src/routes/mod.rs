use axum::{Extension, Router};
use sqlx::postgres::PgPool;
use tower_http::cors::{CorsLayer, Any};

mod users_routes;

pub fn routing(pool: PgPool) -> Router {

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .nest("/users", users_routes::user_routing())
        .layer(Extension(pool))
        .layer(cors);
    app
}