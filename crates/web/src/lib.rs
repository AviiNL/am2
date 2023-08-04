mod leptos;
mod response;
mod sse;

use std::net::SocketAddr;

use ::leptos::{leptos_config::get_config_from_env, view};
use app::App;
use axum::{routing::get, Extension, Router, Server};
use error::Result;
use leptos::{fileserv::file_and_error_handler, leptos_routes_handler};
use leptos_axum::*;
use sse::handler::{latest_handler, subscribe_handler};
pub use sse::service::SseService;

pub async fn start(port: u16) -> Result<SseService> {
    let sse_service = SseService::new();

    let conf = get_config_from_env().unwrap();
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(|| view! { <App/> }).await;

    let router = Router::new()
        .route("/api/v2/sse/:channel", get(latest_handler))
        .route("/sse/v2/subscribe", get(subscribe_handler))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .layer(Extension(sse_service.clone()))
        .with_state(leptos_options);

    tokio::spawn(async move {
        Server::bind(&format!("0.0.0.0:{}", port).parse().unwrap())
            .serve(router.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .unwrap();
    });

    Ok(sse_service)
}