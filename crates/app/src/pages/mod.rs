mod authenticated;
mod login;
mod register;
mod thing;

use login::*;
use register::*;
use thing::*;

pub use authenticated::*;

use leptos::{view, IntoView};

pub enum Page {
    Login,
    Register,
    Authenticated,
}

impl Page {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Login => "/",
            Self::Register => "/register",
            Self::Authenticated => "/console",
        }
    }
}

impl IntoView for Page {
    fn into_view(self) -> leptos::View {
        match self {
            Self::Login => view! { <Thing /> },
            Self::Register => view! { <Register /> },
            Self::Authenticated => view! { <Authenticated /> },
        }
    }
}
