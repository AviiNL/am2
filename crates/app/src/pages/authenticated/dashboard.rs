use leptos::*;

use crate::components::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <Stats />
        <PlayerList />
    }
}
