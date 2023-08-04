use axum::{
    extract::Path,
    http::StatusCode,
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
    Extension,
};
use tokio_stream::{
    wrappers::{errors::BroadcastStreamRecvError, BroadcastStream},
    Stream,
};

use super::service::SseService;

pub async fn latest_handler(
    Extension(sse): Extension<SseService>,
    Path(channel): Path<String>,
) -> Result<String, StatusCode> {
    sse.get_latest(&channel).ok_or(StatusCode::NOT_FOUND)
}

pub async fn subscribe_handler(
    Extension(sse): Extension<SseService>,
) -> Sse<impl Stream<Item = Result<Event, BroadcastStreamRecvError>>> {
    Sse::new(BroadcastStream::new(sse.subscribe())).keep_alive(KeepAlive::default())
}
