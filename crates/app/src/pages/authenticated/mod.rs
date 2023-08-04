mod dashboard;

use crate::components::*;
use dashboard::*;
use leptos::{component, view, IntoView};
use leptos_router::Outlet;

pub enum AuthenticatedPage {
    Dashboard,
}

impl AuthenticatedPage {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Dashboard => "",
        }
    }
}

impl IntoView for AuthenticatedPage {
    fn into_view(self) -> leptos::View {
        match self {
            Self::Dashboard => view! { <Dashboard /> },
        }
    }
}

#[component]
pub fn Authenticated() -> impl IntoView {
    view! {
        <div class="grid gap-4 p-4">
            <Navbar />

            <Outlet />
        </div>
    }
}
