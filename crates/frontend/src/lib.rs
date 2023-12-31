use app::App;
use leptos::*;
#[cfg(debug_assertions)]
use tracing::Level;
#[cfg(debug_assertions)]
use tracing_subscriber::fmt;
#[cfg(debug_assertions)]
use tracing_subscriber::prelude::*;
#[cfg(debug_assertions)]
use tracing_subscriber_wasm::MakeConsoleWriter;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hydrate() {
    // initializes logging using the `log` crate
    #[cfg(debug_assertions)]
    tracing::subscriber::set_global_default(
        fmt::Subscriber::builder()
            .with_env_filter("app=debug,tower_http=debug,leptos=debug,leptos_axum=debug")
            .with_max_level(Level::DEBUG)
            .without_time()
            .with_ansi(false) // firefox no render ansi colors :( chrome does
            .finish()
            // add additional writers
            .with(
                fmt::Layer::default()
                    .with_writer(MakeConsoleWriter::default())
                    .with_ansi(false) // firefox no render ansi colors :( chrome does
                    .without_time(),
            ),
    )
    .expect("Unable to set global tracing subscriber");

    console_error_panic_hook::set_once();

    mount_to_body(move || {
        view! { <App/> }
    });
}
