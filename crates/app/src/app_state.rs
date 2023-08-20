use data::User;
use gloo_storage::{LocalStorage, Storage};
use leptos::*;

use crate::components::Theme;

#[derive(Clone, Copy)]
pub struct AppState {
    pub theme: RwSignal<Theme>,
    pub user: RwSignal<Option<User>>,
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
        theme.set(LocalStorage::get("theme").unwrap_or(Theme::Default));
    });

    theme
}

fn user_signal() -> RwSignal<Option<User>> {
    // create_rw_signal(Some(String::from("Avii")))
    create_rw_signal(None)
}
