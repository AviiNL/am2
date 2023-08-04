use app::App;
use leptos::*;

pub mod fileserv;

use axum::response::{IntoResponse, Response};
use axum::{body::Body as AxumBody, extract::*, http::*};

pub async fn leptos_routes_handler(State(leptos_options): State<LeptosOptions>, req: Request<AxumBody>) -> Response {
    let handler = leptos_axum::render_app_to_stream(leptos_options.clone(), || view! { <App/> });
    handler(req).await.into_response()
}
