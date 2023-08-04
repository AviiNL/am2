use data::{ArmaStatus, Player};
use leptos::*;

use crate::sse::Sse;

#[component]
pub fn PlayerList() -> impl IntoView {
    let sse = use_context::<Sse>().expect("there to be a sse");
    let status = sse.subscribe::<ArmaStatus>("arma_status");

    view! {
        <table class="table w-full shadow bg-base-100">
            <thead>
                <tr>
                    <th>"Name"</th>
                    <th>"Score"</th>
                    <th>"Time"</th>
                </tr>
            </thead>
            <tbody>
                { move || {
                    let ArmaStatus::Online(info) = status.get() else {
                        return view! { <tr><td colspan="3">"Offline"</td></tr> }.into_view();
                    };
                    if info.players.is_empty() {
                        return view! { <tr><td colspan="3">"No players"</td></tr> }.into_view();
                    }

                    let mut v = Vec::with_capacity(info.players.len());
                    for player in info.players {
                        v.push(view! { <Player player=player /> });
                    }
                    v.into_view()
                }}
            </tbody>
        </table>
    }
}

#[component]
pub fn Player(player: Player) -> impl IntoView {
    let timestamp = std::time::Duration::from_secs(player.duration);
    let time = indicatif::HumanDuration(timestamp);

    view! {
        <tr>
            <td>{player.name.clone()}</td>
            <td>{player.score}</td>
            <td>{time.to_string()}</td>
        </tr>
    }
}
