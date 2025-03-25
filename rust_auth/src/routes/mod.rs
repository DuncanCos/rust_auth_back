use axum::{Extension, Router, routing::get};
use sqlx::postgres::PgPool;
use tower_http::cors::{CorsLayer, Any};
use tower_sessions::{ Expiry, MemoryStore, SessionManagerLayer};
use time::Duration;


mod users_routes;

use tower_http::trace::{TraceLayer, DefaultMakeSpan, DefaultOnResponse};
use tracing_subscriber::FmtSubscriber;
use tracing::Level;

use axum::middleware;

use crate::controllers::users_controllers;
use crate::custom_middleware;

pub fn routing(pool: PgPool, session_store: MemoryStore) -> Router {


    
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(3600)));

    let app = Router::new()
        .nest("/users", users_routes::user_routing())
        .layer(middleware::from_fn( custom_middleware::test_middleware))
        .layer(Extension(pool))
        .route("/login", get(users_controllers::login))
        .route("/logout", get(users_controllers::logout))
        .route("/session", get(users_controllers::get_session))
        .layer(session_layer)
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO)) // Log des requêtes entrantes
                .on_response(DefaultOnResponse::new().level(Level::INFO)), // Log des réponses
        );
    app
}

