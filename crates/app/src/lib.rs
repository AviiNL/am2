mod app_state;
mod components;
mod pages;
mod sse;

use app_state::AppState;
use components::Html;
use leptos::*;
use leptos_meta::{provide_meta_context, Link, Stylesheet, Title};
use leptos_router::*;
use pages::*;
use sse::Sse;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let sse = Sse::default();
    provide_context(sse);

    let state = AppState::new();

    provide_context(state);

    let html_attributes =
        Signal::derive(move || AdditionalAttributes::from(vec![("data-theme", state.theme.get().to_string())]));

    view! {
        <Title text="Arma Manager"/>
        <Stylesheet id="leptos" href="/pkg/app.css"/>
        <Link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css" integrity="sha512-iecdLmaskl7CVkqkXNQ/ZH/XLlvWZOJyj7Yy7tcenmpD1ypASozpmT/E0iPtmFIB46ZmdtAc9eNBvH0H/ZpiBw==" crossorigin="anonymous" referrerpolicy="no-referrer" />

        <Html class="bg-base-200" attributes=html_attributes />

        <Router>
            <Routes>
                <ProtectedRoute
                    condition=move || state.user.get().is_some()
                    redirect_path=Page::Login.path()
                    path=Page::Authenticated.path()
                    view=move || Page::Authenticated
                >
                    <Route
                        path=AuthenticatedPage::Dashboard.path()
                        view=move || AuthenticatedPage::Dashboard
                    />
                </ProtectedRoute>
                <ProtectedRoute
                    condition=move || state.user.get().is_none()
                    redirect_path=Page::Authenticated.path()
                    path=Page::Login.path()
                    view=move || Page::Login
                />
                <ProtectedRoute
                    condition=move || state.user.get().is_none()
                    redirect_path=Page::Authenticated.path()
                    path=Page::Register.path()
                    view=move || Page::Register
                />
            </Routes>
        </Router>
    }
}
