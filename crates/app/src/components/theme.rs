use crate::app_state::AppState;
use derive_more::Display;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Display, Debug, Serialize, Deserialize, PartialEq)]
pub enum Theme {
    #[display(fmt = "default")]
    Default,
    #[display(fmt = "dark")]
    Dark,
    #[display(fmt = "light")]
    Light,
}

impl Theme {
    pub fn next(self) -> Self {
        match self {
            Self::Default => Self::Dark,
            Self::Dark => Self::Light,
            Self::Light => Self::Default,
        }
    }
}

impl IntoView for Theme {
    fn into_view(self) -> leptos::View {
        let state = use_context::<AppState>().expect("there to be a state");

        let on_input = move |_| {
            use gloo_storage::{LocalStorage, Storage};
            let next = self.next();
            state.theme.set(next);
            LocalStorage::set("theme", next).expect("to be able to write LocalStorage");
        };

        let class = Signal::derive(move || {
            match self {
                Theme::Default => "fa-solid fa-circle-half-stroke",
                Theme::Dark => "fa-solid fa-circle",
                Theme::Light => "fa-regular fa-circle",
            }
            .to_owned()
        });

        leptos::view! {
            <label on:click=on_input class="btn btn-ghost btn-square swap swap-rotate text-2xl">
                <i class={move || class.get()} title={move || format!("{:?}", self)}></i>
            </label>
        }
        .into_view()
    }
}
