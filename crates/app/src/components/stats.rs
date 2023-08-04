use data::ArmaStatus;
use leptos::*;

use crate::app_state::AppState;

#[component]
pub fn Stats() -> impl IntoView {
    let state = use_context::<AppState>().expect("there to be a state");

    let server_name = Signal::derive(move || {
        // let status = state.arma_status.get();
        // match status {
        //     ArmaStatus::Online(info) => info.name,
        //     ArmaStatus::Offline => String::from("Offline"),
        // }
        String::from("Offline")
    });

    let map = Signal::derive(move || {
        // let status = state.arma_status.get();
        // match status {
        //     ArmaStatus::Online(info) => info.map,
        //     ArmaStatus::Offline => String::from("Offline"),
        // }
        String::from("Offline")
    });

    let mission = Signal::derive(move || {
        // let status = state.arma_status.get();
        // match status {
        //     ArmaStatus::Online(info) => info.mission,
        //     ArmaStatus::Offline => String::from("Offline"),
        // }
        String::from("Offline")
    });

    let players = Signal::derive(move || {
        // let status = state.arma_status.get();
        // match status {
        //     ArmaStatus::Online(info) => format!("{}/{}", info.players.len(), info.max_players),
        //     ArmaStatus::Offline => String::from("Offline"),
        // }
        String::from("Offline")
    });

    view! {
        <div class="stats stats-vertical md:stats-horizontal shadow bg-base-100">
            <div class="stat">
                <div class="stat-title">"Server Name"</div>
                <div class="stat-desc">{ move || server_name }</div>
            </div>
            <div class="stat">
                <div class="stat-title">"Map"</div>
                <div class="stat-desc">{ move || map }</div>
            </div>
            <div class="stat">
                <div class="stat-title">"Mission"</div>
                <div class="stat-desc">{ move || mission }</div>
            </div>
            <div class="stat">
                <div class="stat-title">"Players"</div>
                <div class="stat-desc">{ move || players }</div>
            </div>
        </div>
    }
}
