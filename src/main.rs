mod models;
mod handlers;
mod auth;

use axum::{
    middleware,
    routing::{get, post},
    Extension, Router
};
use std::sync::Arc;
use crate::models::UserRole;
use crate::handlers::{auth as handlers_auth, public, protected};
use crate::auth::middleware::rbac_middleware;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/login", post(handlers_auth::login_handler))
        .route("/public", get(public::public_handler))
        .route(
            "/user",
            get(protected::user_handler)
                .route_layer(middleware::from_fn_with_state(
                    Arc::new(UserRole::User),
                    |headers, Extension(role), req, next| async move{
                        rbac_middleware(headers, Extension(role), req, next).await
                    }
                ))
        )
        .route(
            "/admin",
            get(protected::admin_handler)
                .route_layer(middleware::from_fn_with_state(
                    Arc::new(UserRole::Admin),
                    |headers, Extension(role), req, next| async move{
                        rbac_middleware(headers, Extension(role), req, next).await
                    }
            ))
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!("Server jalan di http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
