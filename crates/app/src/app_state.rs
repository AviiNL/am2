use gloo_storage::{LocalStorage, Storage};
use leptos::*;

use crate::components::Theme;

#[derive(Clone, Copy)]
pub struct AppState {
    pub theme: RwSignal<Theme>,
    pub user: RwSignal<Option<String>>, // TODO: make user object
}

impl AppState {
    pub fn new() -> Self {
        Self {
            theme: theme_signal(),
            user: user_signal(),
        }
    }
}

fn theme_signal() -> RwSignal<Theme> {
    let theme = create_rw_signal(Theme::Default);

    create_effect(move |_| {
        let set_theme = match LocalStorage::get("theme") {
            Ok(theme) => theme,
            Err(_) => Theme::Default,
        };
        theme.set(set_theme);
    });

    theme
}

fn user_signal() -> RwSignal<Option<String>> {
    // create_rw_signal(Some(String::from("Avii")))
    create_rw_signal(None)
}
