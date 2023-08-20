mod api;
mod extensions;
mod inner_leptos;
mod middleware;
mod response;
mod sse;

use std::net::SocketAddr;

use ::leptos::{leptos_config::get_config_from_env, view};
use app::App;
use axum::{routing::get, Extension, Router, Server};
use error::Result;
use inner_leptos::{fileserv::file_and_error_handler, *};
use leptos_axum::*;
use sse::handler::{latest_handler, subscribe_handler};
pub use sse::service::SseService;

pub async fn start(port: u16) -> Result<SseService> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let sse_service = SseService::new();

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    let conf = get_config_from_env().unwrap();
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(|| view! { <App/> }).await;

    let api = Router::new().route("/sse/:channel", get(latest_handler));
    let sse = Router::new().route("/subscribe", get(subscribe_handler));

    let user_repository = data::repository::UserRepository::new(pool.clone());

    let router = Router::new()
        .nest("/api/v2", api)
        .nest("/sse/v2", sse)
        .route("/api/*fn_name", get(server_fn_handler).post(server_fn_handler))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .layer(Extension(sse_service.clone()))
        .layer(Extension(user_repository))
        .with_state(leptos_options);

    tokio::spawn(async move {
        Server::bind(&format!("0.0.0.0:{}", port).parse().unwrap())
            .serve(router.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .unwrap();
    });

    Ok(sse_service)
}
