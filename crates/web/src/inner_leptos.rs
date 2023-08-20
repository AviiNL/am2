use app::App;
use leptos::*;

pub mod fileserv;

use axum::response::{IntoResponse, Response};
use axum::{extract::*, http::*};

pub async fn server_fn_handler(
    user_repository: Extension<data::repository::UserRepository>,
    path: Path<String>,
    headers: HeaderMap,
    raw_query: RawQuery,
    request: Request<axum::body::Body>,
) -> impl IntoResponse {
    leptos_axum::handle_server_fns_with_context(
        path,
        headers,
        raw_query,
        move || {
            leptos::provide_context(user_repository.clone().0);
        },
        request,
    )
    .await
}

pub async fn leptos_routes_handler(
    State(leptos_options): State<LeptosOptions>,
    user_repository: Extension<data::repository::UserRepository>,
    req: Request<axum::body::Body>,
) -> Response {
    let handler = leptos_axum::render_app_to_stream_with_context(
        leptos_options.clone(),
        move || {
            leptos::provide_context(user_repository.clone().0);
        },
        || view! { <App/> },
    );
    handler(req).await.into_response()
}
